use crate::application::commands::CommandHandler;
use crate::application::commands::account::verification::{
    SendVerificationCodeCommand, SubmitVerificationCodeCommand, VerifyByLinkCommand,
};
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::{
    SendVerificationCodeInput, SubmitVerificationCodeInput, VerifyByLinkInput,
};
use crate::presentation::api::graphql::mutations::extract_cookie;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct VerificationMutation;

#[Object]
impl VerificationMutation {
    async fn verify_by_link(&self, ctx: &Context<'_>, input: VerifyByLinkInput) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        use_cases
            .commands
            .verification
            .handle(VerifyByLinkCommand {
                request: input
                    .try_into()
                    .map_err(|e: crate::domain::errors::DomainError| {
                        async_graphql::Error::new(e.to_string())
                    })?,
                cookie: extract_cookie(ctx),
            })
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    async fn send_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SendVerificationCodeInput,
    ) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        use_cases
            .commands
            .verification
            .handle(SendVerificationCodeCommand {
                request: input
                    .try_into()
                    .map_err(|e: crate::domain::errors::DomainError| {
                        async_graphql::Error::new(e.to_string())
                    })?,
                cookie: extract_cookie(ctx),
            })
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    async fn submit_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SubmitVerificationCodeInput,
    ) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let cookie = extract_cookie(ctx).ok_or_else(|| {
            async_graphql::Error::new("Cookie is required to submit verification code")
        })?;
        use_cases
            .commands
            .verification
            .handle(SubmitVerificationCodeCommand {
                request: input.into(),
                cookie,
            })
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}
