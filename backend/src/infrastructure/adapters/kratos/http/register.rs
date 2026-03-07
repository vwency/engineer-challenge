use crate::domain::errors::DomainError;
use crate::domain::ports::registration::{RegistrationData, RegistrationPort};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct KratosRegistrationAdapter {
    client: Arc<KratosClient>,
}

impl KratosRegistrationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RegistrationPort for KratosRegistrationAdapter {
    async fn initiate_registration(&self, cookie: Option<&str>) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            cookie,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(DomainError::FlowNotFound)
    }

    async fn complete_registration(
        &self,
        flow_id: &str,
        data: RegistrationData,
    ) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            None,
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let payload = serde_json::json!({
            "method": "password",
            "password": data.password,
            "traits": {
                "email": data.email,
                "username": data.username,
            },
            "csrf_token": flow.csrf_token,
        });

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| {
            let id = e.body["ui"]["messages"][0]["id"].as_u64().unwrap_or(0);
            match (e.status, id) {
                (StatusCode::BAD_REQUEST, 4000007) => {
                    DomainError::Conflict("Email already exists".to_string())
                }
                (StatusCode::BAD_REQUEST, 4000010) => {
                    DomainError::InvalidData("Password is too weak".to_string())
                }
                (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(
                    e.body["ui"]["messages"][0]["text"]
                        .as_str()
                        .unwrap_or("Invalid registration data")
                        .to_string(),
                ),
                (StatusCode::GONE, _) => DomainError::FlowNotFound,
                _ => DomainError::Network(e.to_string()),
            }
        })?;

        if result.data.get("session").is_none() && result.data.get("identity").is_none() {
            return Err(DomainError::Unknown(
                "Neither session nor identity found in response".to_string(),
            ));
        }

        result
            .cookies
            .into_iter()
            .find(|c| c.contains("ory_kratos_session"))
            .ok_or_else(|| DomainError::Unknown("No session cookie was created".to_string()))
    }
}
