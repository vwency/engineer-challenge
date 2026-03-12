pub mod current_user_query;
pub mod logout_query;

use async_graphql::Context;
pub(super) fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
