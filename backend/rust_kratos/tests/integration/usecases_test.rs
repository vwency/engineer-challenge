use rust_kratos::application::usecases::auth::{
    get_current_user::GetCurrentUserUseCase, login::LoginUseCase, logout::LogoutUseCase,
    recovery::RecoveryUseCase, register::RegisterUseCase, settings::UpdateSettingsUseCase,
};
use rust_kratos::domain::ports::login::LoginCredentials;
use rust_kratos::domain::ports::recovery::RecoveryRequest;
use rust_kratos::domain::ports::registration::RegistrationData;
use rust_kratos::domain::ports::settings::SettingsData;
use rust_kratos::infrastructure::adapters::kratos::http::{
    identity::KratosIdentityAdapter, login::KratosAuthenticationAdapter,
    logout::KratosSessionAdapter, recovery::KratosRecoveryAdapter,
    register::KratosRegistrationAdapter, settings::KratosSettingsAdapter,
};

#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

fn make_register_use_case(ctx: &TestContext) -> RegisterUseCase {
    RegisterUseCase::new(std::sync::Arc::new(KratosRegistrationAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_login_use_case(ctx: &TestContext) -> LoginUseCase {
    LoginUseCase::new(std::sync::Arc::new(KratosAuthenticationAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_logout_use_case(ctx: &TestContext) -> LogoutUseCase {
    LogoutUseCase::new(std::sync::Arc::new(KratosSessionAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_get_current_user_use_case(ctx: &TestContext) -> GetCurrentUserUseCase {
    GetCurrentUserUseCase::new(std::sync::Arc::new(KratosIdentityAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_recovery_use_case(ctx: &TestContext) -> RecoveryUseCase {
    RecoveryUseCase::new(std::sync::Arc::new(KratosRecoveryAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_settings_use_case(ctx: &TestContext) -> UpdateSettingsUseCase {
    UpdateSettingsUseCase::new(std::sync::Arc::new(KratosSettingsAdapter::new(
        ctx.client.clone(),
    )))
}

async fn register_and_login(ctx: &TestContext, email: &str, password: &str) -> String {
    let register = make_register_use_case(ctx);
    let data = RegistrationData {
        email: email.to_string(),
        password: password.to_string(),
        username: format!("user_{}", uuid::Uuid::new_v4()),
        geo_location: None,
    };
    register.execute(data).await.unwrap().session_cookie
}

#[tokio::test]
async fn test_register_use_case_returns_session_cookie() {
    let ctx = TestContext::new();
    let use_case = make_register_use_case(&ctx);
    let data = RegistrationData {
        email: TestContext::random_email(),
        password: "Test1234!@#$".to_string(),
        username: format!("user_{}", uuid::Uuid::new_v4()),
        geo_location: None,
    };
    let result = use_case.execute(data).await;
    assert!(result.is_ok());
    let register_result = result.unwrap();
    assert!(!register_result.session_cookie.is_empty());
    assert!(!register_result.flow_id.is_empty());
}

#[tokio::test]
async fn test_login_use_case_returns_session_cookie() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    register_and_login(&ctx, &email, password).await;
    let use_case = make_login_use_case(&ctx);
    let credentials = LoginCredentials {
        identifier: email.clone(),
        password: password.to_string(),
        address: None,
        code: None,
        resend: None,
    };
    let result = use_case.execute(credentials, None).await;
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn test_login_use_case_with_invalid_credentials_fails() {
    let ctx = TestContext::new();
    let use_case = make_login_use_case(&ctx);
    let credentials = LoginCredentials {
        identifier: "nonexistent@example.com".to_string(),
        password: "wrongpassword".to_string(),
        address: None,
        code: None,
        resend: None,
    };
    let result = use_case.execute(credentials, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logout_use_case_without_cookie_fails() {
    let ctx = TestContext::new();
    let use_case = make_logout_use_case(&ctx);
    let result = use_case.execute(None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logout_use_case_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let use_case = make_logout_use_case(&ctx);
    let result = use_case.execute(Some(&cookie)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_user_without_cookie_fails() {
    let ctx = TestContext::new();
    let use_case = make_get_current_user_use_case(&ctx);
    let result = use_case.execute(None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_current_user_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let use_case = make_get_current_user_use_case(&ctx);
    let result = use_case.execute(Some(&cookie)).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, email);
}

#[tokio::test]
async fn test_recovery_use_case_with_valid_email() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    register_and_login(&ctx, &email, password).await;
    let use_case = make_recovery_use_case(&ctx);
    let result = use_case
        .execute(
            RecoveryRequest {
                email: email.clone(),
            },
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_recovery_use_case_with_unknown_email() {
    let ctx = TestContext::new();
    let use_case = make_recovery_use_case(&ctx);
    let result = use_case
        .execute(
            RecoveryRequest {
                email: "ghost@example.com".to_string(),
            },
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_settings_use_case_without_session_fails() {
    let ctx = TestContext::new();
    let use_case = make_settings_use_case(&ctx);
    let data = SettingsData {
        method: "password".to_string(),
        password: Some("NewPass1234!".to_string()),
        traits: None,
        lookup_secret_confirm: None,
        lookup_secret_disable: None,
        lookup_secret_regenerate: None,
        lookup_secret_reveal: None,
        transient_payload: None,
    };
    let result = use_case.execute(data, "invalid_cookie").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_settings_use_case_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let use_case = make_settings_use_case(&ctx);
    let data = SettingsData {
        method: "password".to_string(),
        password: Some("NewPass5678!@#$".to_string()),
        traits: None,
        lookup_secret_confirm: None,
        lookup_secret_disable: None,
        lookup_secret_regenerate: None,
        lookup_secret_reveal: None,
        transient_payload: None,
    };
    let result = use_case.execute(data, &cookie).await;
    assert!(result.is_err());
}
