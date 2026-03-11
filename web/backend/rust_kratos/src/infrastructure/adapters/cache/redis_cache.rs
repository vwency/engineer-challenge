use crate::domain::errors::DomainError;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisCache {
    connection: redis::aio::ConnectionManager,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> Result<Self, DomainError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;
        let connection = redis::aio::ConnectionManager::new(client)
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;
        Ok(Self { connection })
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.connection.clone();
        conn.get::<_, String>(key).await.ok()
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl_seconds: u64) {
        let mut conn = self.connection.clone();
        let _: Result<(), _> = conn.set_ex(key, value, ttl_seconds).await;
    }
    pub async fn delete(&self, key: &str) {
        let mut conn = self.connection.clone();
        let _: Result<(), _> = conn.del(key).await;
    }
}
