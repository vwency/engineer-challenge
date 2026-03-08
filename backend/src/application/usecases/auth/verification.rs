use crate::domain::errors::DomainError;
use crate::domain::ports::{VerificationCommand, VerificationPort};
use std::sync::Arc;

pub struct VerificationUseCase {
    verification_port: Arc<dyn VerificationPort>,
}

impl VerificationUseCase {
    pub fn new(verification_port: Arc<dyn VerificationPort>) -> Self {
        Self { verification_port }
    }

    pub async fn execute(
        &self,
        command: VerificationCommand,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        self.verification_port.verify(command, cookie).await
    }
}
