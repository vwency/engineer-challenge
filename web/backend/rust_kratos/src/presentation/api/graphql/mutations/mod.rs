pub mod login_mutation;
pub mod recovery_mutation;
pub mod register_mutation;
pub mod settings_mutation;
pub mod verify_mutation;

use async_graphql::Context;

pub(super) fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
