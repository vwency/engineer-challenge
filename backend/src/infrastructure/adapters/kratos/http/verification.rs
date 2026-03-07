use crate::domain::errors::DomainError;
use crate::domain::ports::verification::{
    SendCodeRequest, SubmitCodeRequest, VerificationPort, VerifyByLinkRequest,
};
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
    async fn verify_by_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            cookie,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let flow_id = flow.flow["id"].as_str().ok_or(DomainError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "link",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(map_verification_error)?;

        Ok(())
    }

    async fn send_verification_code(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            cookie,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let flow_id = flow.flow["id"].as_str().ok_or(DomainError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "code",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(map_verification_error)?;

        Ok(())
    }

    async fn submit_verification_code(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            Some(cookie),
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let flow_id = flow.flow["id"].as_str().ok_or(DomainError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "code",
            "code": request.code,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(map_verification_error)?;

        Ok(())
    }
}
