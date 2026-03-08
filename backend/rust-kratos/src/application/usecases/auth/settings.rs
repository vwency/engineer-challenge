use crate::domain::errors::DomainError;
use crate::domain::ports::settings::{SettingsData, SettingsPort};
use std::sync::Arc;

pub struct UpdateSettingsUseCase {
    settings_port: Arc<dyn SettingsPort>,
}

impl UpdateSettingsUseCase {
    pub fn new(settings_port: Arc<dyn SettingsPort>) -> Self {
        Self { settings_port }
    }

    pub async fn execute(
        &self,
        data: SettingsData,
        cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError> {
        let flow_id = self.settings_port.initiate_settings(cookie).await?;
        self.settings_port
            .update_settings(&flow_id, data, cookie)
            .await
    }
}
