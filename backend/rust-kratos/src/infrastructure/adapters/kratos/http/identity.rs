use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use crate::domain::ports::identity::IdentityPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use async_trait::async_trait;
use reqwest::header;
use std::sync::Arc;

pub struct KratosIdentityAdapter {
    client: Arc<KratosClient>,
}

impl KratosIdentityAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl IdentityPort for KratosIdentityAdapter {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, DomainError> {
        let url =
            format!("{}/sessions/whoami", self.client.public_url).replace("localhost", "127.0.0.1");

        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::NotAuthenticated);
        }

        let session_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        let email = session_data["identity"]["traits"]["email"]
            .as_str()
            .ok_or_else(|| DomainError::Unknown("Email not found".to_string()))?
            .to_string();

        let username = session_data["identity"]["traits"]["username"]
            .as_str()
            .ok_or_else(|| DomainError::Unknown("Username not found".to_string()))?
            .to_string();

        let geo_location = session_data["identity"]["traits"]["geo_location"]
            .as_str()
            .map(|s| s.to_string());

        Ok(UserProfile {
            email,
            username,
            geo_location,
        })
    }
}
