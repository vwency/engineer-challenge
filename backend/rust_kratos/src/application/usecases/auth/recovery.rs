use crate::domain::errors::DomainError;
use crate::domain::ports::{RecoveryPort, RecoveryRequest};
use std::sync::Arc;
use tracing::{error, info};

pub struct RecoveryUseCase {
    recovery_port: Arc<dyn RecoveryPort>,
}

impl RecoveryUseCase {
    pub fn new(recovery_port: Arc<dyn RecoveryPort>) -> Self {
        Self { recovery_port }
    }

    pub async fn execute(
        &self,
        request: RecoveryRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        info!(
            email = &request.email,
            cookie_present = cookie.is_some(),
            "Starting recovery process"
        );
        self.recovery_port
            .initiate_recovery(request, cookie)
            .await
            .map_err(|e| {
                error!(error = %e, "Recovery failed");
                e
            })?;
        info!("Recovery email sent successfully");
        Ok(())
    }
}
