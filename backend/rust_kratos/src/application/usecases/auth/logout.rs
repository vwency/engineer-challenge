use crate::domain::errors::DomainError;
use crate::domain::ports::SessionPort;
use std::sync::Arc;

pub struct LogoutUseCase {
    session_port: Arc<dyn SessionPort>,
}

impl LogoutUseCase {
    pub fn new(session_port: Arc<dyn SessionPort>) -> Self {
        Self { session_port }
    }

    pub async fn execute(&self, cookie: Option<&str>) -> Result<(), DomainError> {
        let cookie = cookie.ok_or(DomainError::NotAuthenticated)?;
        self.session_port.logout(cookie).await
    }
}
