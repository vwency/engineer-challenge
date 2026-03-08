use crate::domain::errors::DomainError;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TransientPayload(pub Value);

#[derive(Debug, Clone)]
pub enum VerificationCommand {
    ByLink {
        email: String,
        transient_payload: Option<TransientPayload>,
    },
    SendCode {
        email: String,
        transient_payload: Option<TransientPayload>,
    },
    SubmitCode {
        code: String,
        transient_payload: Option<TransientPayload>,
    },
}

#[async_trait]
pub trait VerificationPort: Send + Sync {
    async fn verify(
        &self,
        command: VerificationCommand,
        cookie: Option<&str>,
    ) -> Result<(), DomainError>;
}
