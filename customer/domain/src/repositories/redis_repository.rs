use axum::async_trait;
use std::time::Duration;
use crate::model::reponse::error::AppResult;

#[async_trait]
pub trait RedisRepository: Send + Sync {
    async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult;
    async fn get(&self, key: &str) -> AppResult<Option<String>>;
    async fn del(&self, key: &str) -> AppResult<bool>;
    async fn get_ttl(&self, key: &str) -> AppResult<i64>;
    async fn check_exist_key(&self, key: &str) -> AppResult<bool>;
}
