use crate::domain::errors::DomainError;
use crate::domain::ports::login::{AuthenticationPort, LoginCredentials};
use std::sync::Arc;

pub struct LoginUseCase {
    auth_port: Arc<dyn AuthenticationPort>,
}

impl LoginUseCase {
    pub fn new(auth_port: Arc<dyn AuthenticationPort>) -> Self {
        Self { auth_port }
    }

    pub async fn execute(
        &self,
        credentials: LoginCredentials,
        cookie: Option<&str>,
    ) -> Result<String, DomainError> {
        let flow_id = self.auth_port.initiate_login(cookie).await?;
        self.auth_port.complete_login(&flow_id, credentials).await
    }
}
