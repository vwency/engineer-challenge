use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::inputs::{LoginCodeInput, LoginPasswordInput};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login_password(&self, ctx: &Context<'_>, input: LoginPasswordInput) -> Result<bool> {
        self.execute_login(ctx, input.into()).await
    }

    async fn login_code(&self, ctx: &Context<'_>, input: LoginCodeInput) -> Result<bool> {
        self.execute_login(ctx, input.into()).await
    }
}

impl LoginMutation {
    async fn execute_login(
        &self,
        ctx: &Context<'_>,
        command: crate::domain::ports::login::LoginCommand,
    ) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let session_token = use_cases
            .login
            .execute(command, cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            response_cookies.add_cookie(session_token).await;
        }

        Ok(true)
    }
}
