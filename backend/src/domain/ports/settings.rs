use crate::domain::errors::DomainError;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum SettingsCommand {
    Password {
        password: String,
    },
    Profile {
        traits: Value,
    },
    LookupSecret {
        confirm: Option<bool>,
        disable: Option<bool>,
        regenerate: Option<bool>,
        reveal: Option<bool>,
    },
}

#[derive(Debug, Clone)]
pub struct SettingsData {
    pub command: SettingsCommand,
    pub transient_payload: Option<Value>,
}

#[async_trait]
pub trait SettingsPort: Send + Sync {
    async fn initiate_settings(&self, cookie: &str) -> Result<String, DomainError>;
    async fn update_settings(
        &self,
        flow_id: &str,
        data: SettingsData,
        cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError>;
}
