use axum::async_trait;
use domain::{ model::reponse::error::AppResult, repositories::redis_repository::RedisRepository };
use tracing::info;
use std::sync::Arc;
use crate::client::redis::RedisClient;
use crate::client::redis::RedisClientExt;
use std::time::Duration;
pub struct RedisRepositoryImpl {
    redis: Arc<RedisClient>,
}

impl RedisRepositoryImpl {
    pub fn new(redis: Arc<RedisClient>) -> Self {
        Self {
            redis,
        }
    }
}
#[async_trait]
impl RedisRepository for RedisRepositoryImpl {
    async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult {
        info!("Set value to redis key :{key:?} value :{value:?}");
        Ok(self.redis.set(key, value, expire).await?)
    }
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        info!("Get value from redis key :{key}");
        Ok(self.redis.get(key).await?)
    }
    async fn del(&self, key: &str) -> AppResult<bool> {
        info!("Delete key in redis :{key:?}");
        Ok(self.redis.del(key).await?)
    }
    async fn get_ttl(&self, key: &str) -> AppResult<i64> {
        info!("Get ttl key in redis :{key:?}");
        Ok(self.redis.ttl(key).await?)
    }
    async fn check_exist_key(&self, key: &str) -> AppResult<bool> {
        info!("Check key exist in redis :{key:?}");
        Ok(self.redis.exist(key).await?)
    }
}
