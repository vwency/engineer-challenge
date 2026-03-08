use crate::domain::errors::DomainError;
use crate::domain::ports::login::{AuthenticationPort, LoginCommand};
use crate::domain::ports::session::SessionPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::{debug, error};

pub struct KratosAuthenticationAdapter {
    client: Arc<KratosClient>,
    session_adapter: KratosSessionAdapter,
}

impl KratosAuthenticationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        let session_adapter = KratosSessionAdapter::new(client.clone());
        Self {
            client,
            session_adapter,
        }
    }
}

#[async_trait]
impl AuthenticationPort for KratosAuthenticationAdapter {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, DomainError> {
        if self.session_adapter.check_active_session(cookie).await {
            if !self.session_adapter.is_recovery_session(cookie).await {
                error!("Login attempt with an already active session");
                return Err(DomainError::AlreadyLoggedIn);
            }
        }

        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(DomainError::FlowNotFound)
    }

    async fn complete_login(
        &self,
        flow_id: &str,
        command: LoginCommand,
    ) -> Result<String, DomainError> {
        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        let csrf_token = flow.csrf_token.clone();
        debug!("Using flow_id: {}, csrf_token: {}", flow_id, csrf_token);

        let payload = match command {
            LoginCommand::Password {
                identifier,
                password,
                address,
            } => {
                let mut p = serde_json::json!({
                    "method": "password",
                    "identifier": identifier,
                    "password": password,
                    "csrf_token": csrf_token,
                });
                if let Some(addr) = address {
                    p["address"] = serde_json::json!(addr);
                }
                p
            }
            LoginCommand::Code { code, resend } => {
                let mut p = serde_json::json!({
                    "method": "code",
                    "code": code,
                    "csrf_token": csrf_token,
                });
                if let Some(resend) = resend {
                    p["resend"] = serde_json::json!(resend);
                }
                p
            }
        };

        debug!(
            "Login payload: {}",
            serde_json::to_string_pretty(&payload).unwrap()
        );

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| {
            error!("Failed to post login flow: {}", e);
            match (e.status, e.message_id()) {
                (StatusCode::BAD_REQUEST, 4000006) => DomainError::InvalidCredentials,
                (StatusCode::BAD_REQUEST, 4000010) => DomainError::InvalidCredentials,
                (StatusCode::BAD_REQUEST, _) => {
                    DomainError::InvalidData(e.message_text().to_string())
                }
                (StatusCode::GONE, _) => DomainError::FlowNotFound,
                (StatusCode::UNAUTHORIZED, _) => DomainError::NotAuthenticated,
                _ => DomainError::Network(e.to_string()),
            }
        })?;

        debug!("Received cookies: {:?}", result.cookies);
        debug!("Response data: {:?}", result.data);

        if result.cookies.is_empty() {
            error!("No cookies in response");
            return Err(DomainError::Unknown(
                "No cookies received from server".to_string(),
            ));
        }

        result
            .cookies
            .into_iter()
            .find(|c| c.contains("session") || c.starts_with("ory_"))
            .ok_or_else(|| {
                error!("Session cookie not found in response cookies");
                DomainError::Unknown("Session token not found".to_string())
            })
    }
}
