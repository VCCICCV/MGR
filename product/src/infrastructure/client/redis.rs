use redis::{ Client, RedisError };
use std::time::Duration;
use tracing::log::info;
use crate::{configure::{ redis::RedisConfig, AppConfig }, error::AppResult};
use super::builder::ClientBuilder;
// 类型别名
pub type RedisClient = redis::Client;
// 方法trait
pub trait RedisClientExt: ClientBuilder {
    fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration
    ) -> impl std::future::Future<Output = Result<(), RedisError>>;
    fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
    fn get(
        &self,
        key: &str
    ) -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
    fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
    fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64, RedisError>>;
}
// 客户端
impl ClientBuilder for RedisClient {
    fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        Ok(redis::Client::open(config.redis.get_url())?)
    }
}

pub struct RedisTestContext {
    pub config: RedisConfig,
    pub redis: RedisClient,
}

impl RedisClientExt for Client {
    // ping redis server
    async fn ping(&self) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
        info!("ping redis server");
        Ok(value)
    }
    // set key value expire
    async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg: String = redis::cmd("SET").arg(&[key, value]).query_async(&mut conn).await?;
        info!("set key redis: {msg}");
        let msg: i32 = redis
            ::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn).await?;
        info!("set expire time redis: {msg}");
        Ok(())
    }
    // check key exists
    async fn exist(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        info!("check key exists: {key}");
        Ok(value)
    }
    // get value
    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        info!("get value: {key}");
        Ok(value)
    }
    // delete value
    async fn del(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        info!("delete value: {key}");
        Ok(value == 1)
    }
    // get TTL value
    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        info!("get TTL value: {key}");
        Ok(value)
    }
}