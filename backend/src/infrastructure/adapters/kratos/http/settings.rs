use crate::domain::errors::DomainError;
use crate::domain::ports::settings::{SettingsCommand, SettingsData, SettingsPort};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;

pub struct KratosSettingsAdapter {
    client: Arc<KratosClient>,
}

impl KratosSettingsAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SettingsPort for KratosSettingsAdapter {
    async fn initiate_settings(&self, cookie: &str) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            Some(cookie),
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(DomainError::FlowNotFound)
    }

    async fn update_settings(
        &self,
        flow_id: &str,
        data: SettingsData,
        cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            Some(cookie),
        )
        .await
        .map_err(|e| DomainError::Network(e.to_string()))?;

        let csrf_token = flow.csrf_token.clone();

        let mut payload = match &data.command {
            SettingsCommand::Password { password } => serde_json::json!({
                "method": "password",
                "password": password,
                "csrf_token": csrf_token,
            }),
            SettingsCommand::Profile { traits } => serde_json::json!({
                "method": "profile",
                "traits": traits,
                "csrf_token": csrf_token,
            }),
            SettingsCommand::LookupSecret {
                confirm,
                disable,
                regenerate,
                reveal,
            } => {
                let mut p = serde_json::json!({
                    "method": "lookup_secret",
                    "csrf_token": csrf_token,
                });
                if let Some(v) = confirm {
                    p["lookup_secret_confirm"] = Value::Bool(*v);
                }
                if let Some(v) = disable {
                    p["lookup_secret_disable"] = Value::Bool(*v);
                }
                if let Some(v) = regenerate {
                    p["lookup_secret_regenerate"] = Value::Bool(*v);
                }
                if let Some(v) = reveal {
                    p["lookup_secret_reveal"] = Value::Bool(*v);
                }
                p
            }
        };

        if let Some(tp) = data.transient_payload {
            payload["transient_payload"] = tp;
        }

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| match (e.status, e.message_id()) {
            (StatusCode::FORBIDDEN, _) => DomainError::PrivilegedSessionRequired,
            (StatusCode::UNAUTHORIZED, _) => DomainError::NotAuthenticated,
            (StatusCode::GONE, _) => DomainError::FlowNotFound,
            (StatusCode::BAD_REQUEST, 4000010) => {
                DomainError::InvalidData("Password is too weak".to_string())
            }
            (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().to_string()),
            _ => DomainError::Network(e.to_string()),
        })?;

        let state = result
            .data
            .get("state")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| DomainError::Unknown("No state in response".to_string()))?;

        Ok((state, result.cookies))
    }
}
