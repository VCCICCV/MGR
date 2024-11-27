use crate::client::redis::RedisClientExt;
use axum::async_trait;
use std::time::Duration;
use domain::{ model::reponse::error::AppResult, utils::redis::RedisUtil };
use std::sync::Arc;
use crate::client::redis::RedisClient;

pub struct RedisUtilImpl {
    redis: Arc<RedisClient>,
}
impl RedisUtilImpl {
    pub fn new(redis: Arc<RedisClient>) -> Self {
        RedisUtilImpl { redis }
    }
}
#[async_trait]
impl RedisUtil for RedisUtilImpl {
    async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult {
        self.redis.set(key, value, expire).await.map_err(|e| e.into())
    }
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.redis.get(key).await.map_err(|e| e.into())
    }
    async fn del(&self, key: &str) -> AppResult<bool> {
        self.redis.del(key).await.map_err(|e| e.into())
    }
    async fn ttl(&self, key: &str) -> AppResult<i64> {
        self.redis.ttl(key).await.map_err(|e| e.into())
    }
    async fn check_exist_key(&self, key: &str) -> AppResult<bool> {
        self.redis.exist(key).await.map_err(|e| e.into())
    }
}
