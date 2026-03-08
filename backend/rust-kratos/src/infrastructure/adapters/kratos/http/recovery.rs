use crate::domain::errors::DomainError;
use crate::domain::ports::recovery::{RecoveryPort, RecoveryRequest};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::debug;

pub struct KratosRecoveryAdapter {
    client: Arc<KratosClient>,
}

impl KratosRecoveryAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RecoveryPort for KratosRecoveryAdapter {
    async fn initiate_recovery(
        &self,
        request: RecoveryRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "recovery",
            cookie,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let flow_id = flow.flow["id"].as_str().ok_or(DomainError::FlowNotFound)?;

        let payload = serde_json::json!({
            "method": "link",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "recovery",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| match (e.status, e.message_id()) {
            (StatusCode::BAD_REQUEST, 4060001) => {
                DomainError::InvalidData("Invalid email address".to_string())
            }
            (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().to_string()),
            (StatusCode::GONE, _) => DomainError::FlowNotFound,
            (StatusCode::UNAUTHORIZED, _) => DomainError::NotAuthenticated,
            _ => DomainError::Network(e.to_string()),
        })?;

        if result.cookies.is_empty() {
            debug!("No cookies returned from Kratos");
        } else {
            debug!(
                cookies_count = result.cookies.len(),
                cookies = ?result.cookies,
                "Cookies returned from Kratos"
            );
        }

        Ok(())
    }
}
