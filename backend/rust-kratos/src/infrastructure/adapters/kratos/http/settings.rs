use crate::domain::errors::DomainError;
use crate::domain::ports::settings::{SettingsData, SettingsPort};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::debug;

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
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(DomainError::NotFound("settings flow".into()))
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
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        let csrf_token = flow.csrf_token.clone();
        debug!("Using flow_id: {}, csrf_token: {}", flow_id, csrf_token);

        let mut payload = serde_json::json!({
            "method": data.method,
            "csrf_token": csrf_token,
        });

        if let Some(password) = data.password {
            payload["password"] = serde_json::Value::String(password);
        }
        if let Some(traits) = data.traits {
            payload["traits"] = traits;
        }
        if let Some(v) = data.lookup_secret_confirm {
            payload["lookup_secret_confirm"] = serde_json::Value::Bool(v);
        }
        if let Some(v) = data.lookup_secret_disable {
            payload["lookup_secret_disable"] = serde_json::Value::Bool(v);
        }
        if let Some(v) = data.lookup_secret_regenerate {
            payload["lookup_secret_regenerate"] = serde_json::Value::Bool(v);
        }
        if let Some(v) = data.lookup_secret_reveal {
            payload["lookup_secret_reveal"] = serde_json::Value::Bool(v);
        }
        if let Some(payload_extra) = data.transient_payload {
            payload["transient_payload"] = payload_extra;
        }

        debug!(
            "Settings payload: {}",
            serde_json::to_string_pretty(&payload).unwrap()
        );

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
            (StatusCode::GONE, _) => DomainError::NotFound("settings flow".into()),
            (StatusCode::BAD_REQUEST, 4000010) => {
                DomainError::InvalidData("Password is too weak".into())
            }
            (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().into()),
            _ => DomainError::ServiceUnavailable(e.to_string()),
        })?;

        debug!("Settings response: {:?}", result.data);
        debug!("Settings response cookies: {:?}", result.cookies);

        let state = result
            .data
            .get("state")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| DomainError::ServiceUnavailable("No state in response".into()))?;

        Ok((state, result.cookies))
    }
}
