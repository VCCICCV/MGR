use redis::Client;
use anyhow::{Result, Context};
use std::time::Duration;
use tracing::log::info;
use test_context::AsyncTestContext;
use crate::{configure::{AppConfig, redis::RedisConfig}, constant::CONFIG};
use crate::client::ClientBuilder;

// 类型别名
pub type RedisClient = redis::Client;

// 方法trait
pub trait RedisClientExt: ClientBuilder {
    fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>>>;
    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration
    ) -> impl std::future::Future<Output = Result<()>>;
    fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool>>;
    fn get(&self, key: &str) -> impl std::future::Future<Output = Result<Option<String>>>;
    fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool>>;
    fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64>>;
}

// 客户端构建实现
impl ClientBuilder for RedisClient {
    fn build_from_config(config: &AppConfig) -> Result<redis::Client, anyhow::Error> {
        redis::Client::open(config.redis.get_url())
            .context("Failed to create Redis client")
    }
}

// 测试上下文
pub struct RedisTestContext {
    pub config: RedisConfig,
    pub redis: RedisClient,
}

impl AsyncTestContext for RedisTestContext {
    async fn setup() -> Self {
        info!("setup redis config for the test");
        let redis = RedisClient::build_from_config(&CONFIG)
            .expect("Failed to create Redis client for test");
        Self {
            config: CONFIG.redis.clone(),
            redis,
        }
    }
}

// Redis操作实现
impl RedisClientExt for Client {
    async fn ping(&self) -> Result<Option<String>> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let value: Option<String> = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .context("PING command failed")?;
        
        info!("ping redis server");
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<()> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let msg: String = redis::cmd("SET")
            .arg(&[key, value])
            .query_async(&mut conn)
            .await
            .context("SET command failed")?;
        
        info!("set key redis: {msg}");
        
        let msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await
            .context("EXPIRE command failed")?;
        
        info!("set expire time redis: {msg}");
        Ok(())
    }

    async fn exist(&self, key: &str) -> Result<bool> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let value: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
            .context("EXISTS command failed")?;
        
        info!("check key exists: {key}");
        Ok(value)
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .context("GET command failed")?;
        
        info!("get value: {key}");
        Ok(value)
    }

    async fn del(&self, key: &str) -> Result<bool> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let value: i32 = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .context("DEL command failed")?;
        
        info!("delete value: {key}");
        Ok(value == 1)
    }

    async fn ttl(&self, key: &str) -> Result<i64> {
        let mut conn = self.get_multiplexed_async_connection()
            .await
            .context("Failed to get Redis connection")?;
        
        let value: i64 = redis::cmd("TTL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .context("TTL command failed")?;
        
        info!("get TTL value: {key}");
        Ok(value)
    }
}
// use redis::{ Client, RedisError };
// use anyhow::Result;
// use std::time::Duration;
// use tracing::log::info;
// use test_context::AsyncTestContext;
// use crate::{ config::{ AppConfig, redis::RedisConfig }, constant::CONFIG };
// use super::builder::ClientBuilder;
// // 类型别名
// pub type RedisClient = redis::Client;
// // 方法trait
// pub trait RedisClientExt: ClientBuilder {
//     fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
//     fn set(
//         &self,
//         key: &str,
//         value: &str,
//         expire: Duration
//     ) -> impl std::future::Future<Output = Result<(), RedisError>>;
//     fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
//     fn get(
//         &self,
//         key: &str
//     ) -> impl std::future::Future<Output = Result<Option<String>, RedisError>>;
//     fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool, RedisError>>;
//     fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64, RedisError>>;
// }
// // 客户端
// impl ClientBuilder for RedisClient {
//     fn build_from_config(config: &AppConfig) -> Result<Self,Box<dyn std::error::Error>> {
//         Ok(redis::Client::open(config.redis.get_url())?)
//     }
// }

// pub struct RedisTestContext {
//     pub config: RedisConfig,
//     pub redis: RedisClient,
// }

// impl AsyncTestContext for RedisTestContext {
//     async fn setup() -> Self {
//         info!("setup redis config for the test");
//         let redis = RedisClient::build_from_config(&CONFIG).unwrap();
//         Self {
//             config: CONFIG.redis.clone(),
//             redis,
//         }
//     }
// }

// impl RedisClientExt for Client {
//     // ping redis server
//     async fn ping(&self) -> anyhow::Result<Option<String>, RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
//         info!("ping redis server");
//         Ok(value)
//     }
//     // set key value expire
//     async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let msg: String = redis::cmd("SET").arg(&[key, value]).query_async(&mut conn).await?;
//         info!("set key redis: {msg}");
//         let msg: i32 = redis
//             ::cmd("EXPIRE")
//             .arg(&[key, &expire.as_secs().to_string()])
//             .query_async(&mut conn).await?;
//         info!("set expire time redis: {msg}");
//         Ok(())
//     }
//     // check key exists
//     async fn exist(&self, key: &str) -> Result<bool, RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
//         info!("check key exists: {key}");
//         Ok(value)
//     }
//     // get value
//     async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
//         info!("get value: {key}");
//         Ok(value)
//     }
//     // delete value
//     async fn del(&self, key: &str) -> Result<bool, RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
//         info!("delete value: {key}");
//         Ok(value == 1)
//     }
//     // get TTL value
//     async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;
//         let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
//         info!("get TTL value: {key}");
//         Ok(value)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::constant::REDIS;

    use super::*;

    use fake::{ Fake, Faker };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_ping_redis_server() {
        let resp = REDIS.ping().await.unwrap();
        let pong = "PONG";
        assert!(matches!(resp, Some(p) if p == pong));
    }

    #[tokio::test]
    async fn test_set_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(5)).await.unwrap();
        let resp = REDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = REDIS.ttl(&key).await.unwrap();
        assert!(resp > 0);
    }
    #[tokio::test]
    async fn test_set_key_code() {
        let key: String = "email".to_string();
        let value = "123456".to_string();
        REDIS.set(&key, &value, Duration::from_secs(60)).await.unwrap();
        let resp = REDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = REDIS.ttl(&key).await.unwrap();
        assert!(resp > 0);
    }
    #[tokio::test]
    async fn test_exist_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
        let resp = REDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = REDIS.exist(&key).await.unwrap();
        assert!(resp);
        let key: String = Faker.fake();
        let resp = REDIS.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_del_key_redis() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
        let resp = REDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = REDIS.exist(&key).await.unwrap();
        assert!(resp);
        REDIS.del(&key).await.unwrap();
        let resp = REDIS.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_key_ttl_redis() {
        let key: String = Faker.fake();
        let ttl = 4;
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(ttl)).await.unwrap();
        let resp = REDIS.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = REDIS.ttl(&key).await.unwrap();
        assert!(resp <= (ttl as i64) && resp > 0);
        REDIS.del(&key).await.unwrap();
        let resp = REDIS.ttl(&key).await.unwrap();
        assert!(resp < 0);
    }
    #[tokio::test]
    async fn test_get_value_without_key() {
        let key: String = Faker.fake();
        let result = REDIS.get(&key).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_value_with_key() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(5)).await.unwrap();
        let result = REDIS.get(&key).await;
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
        // assert!(matches!(result.unwrap(), Some(v) if v == value));
    }

    #[tokio::test]
    async fn test_get_value_after_expiration() {
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        REDIS.set(&key, &value, Duration::from_secs(1)).await.unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;
        let result = REDIS.get(&key).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
