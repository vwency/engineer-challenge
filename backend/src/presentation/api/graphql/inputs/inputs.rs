use crate::domain::ports::login::LoginCommand;
use crate::domain::ports::recovery::RecoveryRequest;
use crate::domain::ports::registration::RegistrationData;
use crate::domain::ports::settings::{SettingsCommand, SettingsData};
use crate::domain::ports::verification::{TransientPayload, VerificationCommand};
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
pub struct LoginPasswordInput {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct LoginCodeInput {
    pub code: String,
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
pub struct UpdatePasswordInput {
    pub password: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct UpdateProfileInput {
    pub traits: Value,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct UpdateLookupSecretInput {
    pub confirm: Option<bool>,
    pub disable: Option<bool>,
    pub regenerate: Option<bool>,
    pub reveal: Option<bool>,
    pub transient_payload: Option<Value>,
}

impl From<LoginPasswordInput> for LoginCommand {
    fn from(input: LoginPasswordInput) -> Self {
        LoginCommand::Password {
            identifier: input.identifier,
            password: input.password,
            address: input.address,
        }
    }
}

impl From<LoginCodeInput> for LoginCommand {
    fn from(input: LoginCodeInput) -> Self {
        LoginCommand::Code {
            code: input.code,
            resend: input.resend,
        }
    }
}

impl From<RecoveryInput> for RecoveryRequest {
    fn from(input: RecoveryInput) -> Self {
        Self { email: input.email }
    }
}

impl From<RegisterInput> for RegistrationData {
    fn from(input: RegisterInput) -> Self {
        Self {
            email: input.email,
            username: input.username.unwrap_or_default(),
            password: input.password,
            geo_location: input.geo_location,
        }
    }
}

impl From<UpdatePasswordInput> for SettingsData {
    fn from(input: UpdatePasswordInput) -> Self {
        Self {
            command: SettingsCommand::Password {
                password: input.password,
            },
            transient_payload: input.transient_payload,
        }
    }
}

impl From<UpdateProfileInput> for SettingsData {
    fn from(input: UpdateProfileInput) -> Self {
        Self {
            command: SettingsCommand::Profile {
                traits: input.traits,
            },
            transient_payload: input.transient_payload,
        }
    }
}

impl From<UpdateLookupSecretInput> for SettingsData {
    fn from(input: UpdateLookupSecretInput) -> Self {
        Self {
            command: SettingsCommand::LookupSecret {
                confirm: input.confirm,
                disable: input.disable,
                regenerate: input.regenerate,
                reveal: input.reveal,
            },
            transient_payload: input.transient_payload,
        }
    }
}

impl From<VerifyByLinkInput> for VerificationCommand {
    fn from(input: VerifyByLinkInput) -> Self {
        VerificationCommand::ByLink {
            email: input.email,
            transient_payload: input.transient_payload.map(TransientPayload),
        }
    }
}

impl From<SendVerificationCodeInput> for VerificationCommand {
    fn from(input: SendVerificationCodeInput) -> Self {
        VerificationCommand::SendCode {
            email: input.email,
            transient_payload: input.transient_payload.map(TransientPayload),
        }
    }
}

impl From<SubmitVerificationCodeInput> for VerificationCommand {
    fn from(input: SubmitVerificationCodeInput) -> Self {
        VerificationCommand::SubmitCode {
            code: input.code,
            transient_payload: input.transient_payload.map(TransientPayload),
        }
    }
}
