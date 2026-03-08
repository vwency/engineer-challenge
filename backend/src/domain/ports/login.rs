use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum LoginCommand {
    Password {
        identifier: String,
        password: String,
        address: Option<String>,
    },
    Code {
        code: String,
        resend: Option<String>,
    },
}

#[async_trait]
pub trait AuthenticationPort: Send + Sync {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, DomainError>;
    async fn complete_login(
        &self,
        flow_id: &str,
        command: LoginCommand,
    ) -> Result<String, DomainError>;
}
