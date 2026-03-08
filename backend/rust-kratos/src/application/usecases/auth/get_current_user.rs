use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use crate::domain::ports::IdentityPort;
use std::sync::Arc;

pub struct GetCurrentUserUseCase {
    identity_port: Arc<dyn IdentityPort>,
}

impl GetCurrentUserUseCase {
    pub fn new(identity_port: Arc<dyn IdentityPort>) -> Self {
        Self { identity_port }
    }

    pub async fn execute(&self, cookie: Option<&str>) -> Result<UserProfile, DomainError> {
        let cookie = cookie.ok_or(DomainError::NotAuthenticated)?;
        self.identity_port.get_current_user(cookie).await
    }
}
