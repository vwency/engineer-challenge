use crate::domain::errors::DomainError;
use crate::domain::ports::{
    SendCodeRequest, SubmitCodeRequest, VerificationPort, VerifyByLinkRequest,
};
use std::sync::Arc;

pub struct VerificationUseCase {
    verification_port: Arc<dyn VerificationPort>,
}

impl VerificationUseCase {
    pub fn new(verification_port: Arc<dyn VerificationPort>) -> Self {
        Self { verification_port }
    }

    pub async fn execute_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        self.verification_port.verify_by_link(request, cookie).await
    }

    pub async fn execute_code_send(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        self.verification_port
            .send_verification_code(request, cookie)
            .await
    }

    pub async fn execute_code_submit(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), DomainError> {
        self.verification_port
            .submit_verification_code(request, cookie)
            .await
    }
}
