use crate::domain::errors::DomainError;
use crate::domain::ports::login::LoginCredentials;
use crate::domain::ports::recovery::RecoveryRequest;
use crate::domain::ports::registration::RegistrationData;
use crate::domain::ports::settings::SettingsData;
use crate::domain::ports::verification::{SendCodeRequest, SubmitCodeRequest, VerifyByLinkRequest};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(InputObject, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct LoginInput {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub resend: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryInput {
    pub email: String,
}

#[derive(InputObject)]
pub struct VerifyByLinkInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SendVerificationCodeInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SubmitVerificationCodeInput {
    pub code: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct UpdateSettingsInput {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<Value>,
}

impl TryFrom<LoginInput> for LoginCredentials {
    type Error = DomainError;

    fn try_from(input: LoginInput) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: Email::new(input.identifier)?,
            password: Password::new(input.password)?,
            address: input.address,
            code: input.code,
            resend: input.resend,
        })
    }
}

impl TryFrom<RecoveryInput> for RecoveryRequest {
    type Error = DomainError;

    fn try_from(input: RecoveryInput) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(input.email)?,
        })
    }
}

impl TryFrom<RegisterInput> for RegistrationData {
    type Error = DomainError;

    fn try_from(input: RegisterInput) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(input.email)?,
            username: input.username.unwrap_or_default(),
            password: Password::new(input.password)?,
            geo_location: input.geo_location,
        })
    }
}

impl TryFrom<UpdateSettingsInput> for SettingsData {
    type Error = DomainError;

    fn try_from(input: UpdateSettingsInput) -> Result<Self, Self::Error> {
        Ok(Self {
            method: input.method,
            password: input.password.map(Password::new).transpose()?,
            traits: input.traits,
            lookup_secret_confirm: input.lookup_secret_confirm,
            lookup_secret_disable: input.lookup_secret_disable,
            lookup_secret_regenerate: input.lookup_secret_regenerate,
            lookup_secret_reveal: input.lookup_secret_reveal,
            transient_payload: input.transient_payload,
        })
    }
}

impl TryFrom<VerifyByLinkInput> for VerifyByLinkRequest {
    type Error = DomainError;

    fn try_from(input: VerifyByLinkInput) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(input.email)?,
            transient_payload: input.transient_payload,
        })
    }
}

impl TryFrom<SendVerificationCodeInput> for SendCodeRequest {
    type Error = DomainError;

    fn try_from(input: SendVerificationCodeInput) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(input.email)?,
            transient_payload: input.transient_payload,
        })
    }
}

impl From<SubmitVerificationCodeInput> for SubmitCodeRequest {
    fn from(input: SubmitVerificationCodeInput) -> Self {
        Self {
            code: input.code,
            transient_payload: input.transient_payload,
        }
    }
}
