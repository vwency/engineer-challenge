use crate::domain::errors::DomainError;
use crate::domain::ports::verification::{VerificationCommand, VerificationPort};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct KratosVerificationAdapter {
    client: Arc<KratosClient>,
}

impl KratosVerificationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }

    async fn fetch_verification_flow(
        &self,
        cookie: Option<&str>,
    ) -> Result<(String, String, Vec<String>), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            cookie,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or(DomainError::FlowNotFound)?
            .to_string();

        Ok((flow_id, flow.csrf_token, flow.cookies))
    }
}

fn map_verification_error(
    e: crate::infrastructure::adapters::kratos::models::errors::KratosFlowError,
) -> DomainError {
    match (e.status, e.message_id()) {
        (StatusCode::BAD_REQUEST, 4070006) => {
            DomainError::InvalidData("Invalid verification code".to_string())
        }
        (StatusCode::BAD_REQUEST, 4070001) => {
            DomainError::InvalidData("Invalid email address".to_string())
        }
        (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().to_string()),
        (StatusCode::GONE, _) => DomainError::FlowNotFound,
        (StatusCode::UNAUTHORIZED, _) => DomainError::NotAuthenticated,
        _ => DomainError::Network(e.to_string()),
    }
}

#[async_trait]
impl VerificationPort for KratosVerificationAdapter {
    async fn verify(
        &self,
        command: VerificationCommand,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        let (flow_id, csrf_token, cookies) = self.fetch_verification_flow(cookie).await?;

        let mut payload = match &command {
            VerificationCommand::ByLink { email, .. } => serde_json::json!({
                "method": "link",
                "email": email,
                "csrf_token": csrf_token,
            }),
            VerificationCommand::SendCode { email, .. } => serde_json::json!({
                "method": "code",
                "email": email,
                "csrf_token": csrf_token,
            }),
            VerificationCommand::SubmitCode { code, .. } => serde_json::json!({
                "method": "code",
                "code": code,
                "csrf_token": csrf_token,
            }),
        };

        let transient = match command {
            VerificationCommand::ByLink {
                transient_payload, ..
            } => transient_payload,
            VerificationCommand::SendCode {
                transient_payload, ..
            } => transient_payload,
            VerificationCommand::SubmitCode {
                transient_payload, ..
            } => transient_payload,
        };

        if let Some(t) = transient {
            payload["transient_payload"] = serde_json::to_value(t.0).unwrap_or_default();
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            &flow_id,
            payload,
            &cookies,
        )
        .await
        .map_err(map_verification_error)?;

        Ok(())
    }
}
