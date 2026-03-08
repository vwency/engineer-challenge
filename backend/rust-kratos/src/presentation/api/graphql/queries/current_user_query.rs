use crate::domain::entities::user_profile::UserProfile;
use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct CurrentUserQuery;

#[Object]
impl CurrentUserQuery {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<UserProfile> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let user_profile = use_cases
            .get_current_user
            .execute(cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(user_profile)
    }
}
