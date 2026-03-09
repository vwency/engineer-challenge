use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct LogoutQuery;

#[Object]
impl LogoutQuery {
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        use_cases.logout.execute(cookie).await?;
        Ok(true)
    }
}
