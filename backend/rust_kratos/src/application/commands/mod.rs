use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler<C, R = ()>: Send + Sync {
    async fn handle(&self, command: C) -> Result<R, DomainError>;
}

pub mod login;
pub mod logout;
pub mod recovery;
pub mod register;
pub mod settings;
pub mod verification;
