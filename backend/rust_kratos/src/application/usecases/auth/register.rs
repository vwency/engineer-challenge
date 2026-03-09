use crate::domain::errors::DomainError;
use crate::domain::ports::registration::{RegistrationData, RegistrationPort};
use std::sync::Arc;

pub struct RegisterUseCase {
    registration_port: Arc<dyn RegistrationPort>,
}

impl RegisterUseCase {
    pub fn new(registration_port: Arc<dyn RegistrationPort>) -> Self {
        Self { registration_port }
    }

    pub async fn execute(&self, data: RegistrationData) -> Result<RegisterResult, DomainError> {
        let flow_id = self.registration_port.initiate_registration(None).await?;
        let session_cookie = self
            .registration_port
            .complete_registration(&flow_id, data)
            .await?;
        Ok(RegisterResult {
            flow_id,
            session_cookie,
        })
    }
}

pub struct RegisterResult {
    pub flow_id: String,
    pub session_cookie: String,
}
