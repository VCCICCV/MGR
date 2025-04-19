use std::{process, sync::Arc};

use config::{redis::{RedisConfig, RedisInstancesConfig, RedisMode}, OptionalConfigs};
use redis::{cluster::ClusterClient, Client};
use shared::global::{get_config, RedisConnection, GLOBAL_PRIMARY_REDIS, GLOBAL_REDIS_POOL};
use tracing::{error, info};

/// 初始化主Redis
pub async fn init_primary_redis() {
    // 从全局中读取配置
    if let Some(config) = get_config::<RedisConfig>().await {
        match create_redis_connection(&config).await {
            Ok(connection) => {
                *GLOBAL_PRIMARY_REDIS.write().await = Some(connection);
                info!("Primary Redis connection initialized ({})", if
                    config.mode == RedisMode::Cluster
                {
                    "Cluster mode"
                } else {
                    "Single mode"
                });
            }
            Err(e) => {
                error!("Failed to initialize primary Redis: {}", e);
                process::exit(1);
            }
        }
    }
}

async fn create_redis_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    if config.mode == RedisMode::Cluster {
        create_cluster_connection(config).await
    } else {
        create_single_connection(config).await
    }
}

async fn create_single_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    let url = config.get_url().ok_or_else(|| "URL is required for single mode Redis".to_string())?;

    let client = redis::Client
        ::open(url.as_str())
        .map_err(|e| format!("Failed to create Redis client: {}", e))?;

    test_single_connection(&client).await?;

    Ok(RedisConnection::Single(Arc::new(client)))
}

async fn test_single_connection(client: &Client) -> Result<(), String> {
    let mut con = client
        .get_multiplexed_async_connection().await
        .map_err(|e| format!("Failed to create connection manager: {}", e))?;

    let _: String = redis
        ::cmd("PING")
        .query_async(&mut con).await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

async fn create_cluster_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    let urls = config.get_urls().ok_or_else(|| "URLs are required for cluster mode".to_string())?;

    if urls.is_empty() {
        return Err("Cluster mode requires at least one URL".to_string());
    }

    let client = redis::cluster::ClusterClient
        ::new(
            urls
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
        )
        .map_err(|e| format!("Failed to create Redis cluster client: {}", e))?;

    test_cluster_connection(&client).await?;

    Ok(RedisConnection::Cluster(Arc::new(client)))
}

async fn test_cluster_connection(client: &ClusterClient) -> Result<(), String> {
    let mut con = client
        .get_async_connection().await
        .map_err(|e| format!("Failed to connect to Redis cluster: {}", e))?;

    let _: String = redis
        ::cmd("PING")
        .query_async(&mut con).await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

pub async fn init_redis_pool(
    redis_instances_config: Option<Vec<RedisInstancesConfig>>
) -> Result<(), String> {
    if let Some(redis_instances) = redis_instances_config {
        for redis_instance in redis_instances {
            init_redis_connection(&redis_instance.name, &redis_instance.redis).await?;
        }
    }
    Ok(())
}

async fn init_redis_connection(name: &str, config: &RedisConfig) -> Result<(), String> {
    match create_redis_connection(config).await {
        Ok(connection) => {
            GLOBAL_REDIS_POOL.write().await.insert(name.to_string(), connection);
            info!("Redis '{}' initialized", name);
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to initialize Redis '{}': {}", name, e);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 初始化所有 Redis 连接
pub async fn init_redis_pools() {
    if
        let Some(redis_instances_config) =
            get_config::<OptionalConfigs<RedisInstancesConfig>>().await
    {
        if let Some(redis_instances) = &redis_instances_config.configs {
            let _ = init_redis_pool(Some(redis_instances.clone())).await;
        }
    }
}

pub async fn get_primary_redis() -> Option<RedisConnection> {
    GLOBAL_PRIMARY_REDIS.read().await.clone()
}

pub async fn get_redis_pool_connection(name: &str) -> Option<RedisConnection> {
    GLOBAL_REDIS_POOL.read().await.get(name).cloned()
}

pub async fn add_or_update_redis_pool(name: &str, config: &RedisConfig) -> Result<(), String> {
    init_redis_connection(name, config).await
}

pub async fn remove_redis_pool(name: &str) -> Result<(), String> {
    let mut redis_pool = GLOBAL_REDIS_POOL.write().await;
    redis_pool.remove(name).ok_or_else(|| format!("Redis connection '{}' not found", name))?;
    info!("Redis connection '{}' removed", name);
    Ok(())
}
// use config::AppConfig;
// use redis::Client;
// use shared::error::AppError;
// use std::time::Duration;
// use tracing::log::info;

// use super::ClientBuilder;

// // 类型别名
// pub type RedisClient = redis::Client;

// // 方法trait
// pub trait RedisClientExt: ClientBuilder {
//     fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>, AppError>>;
//     fn set(
//         &self,
//         key: &str,
//         value: &str,
//         expire: Duration
//     ) -> impl std::future::Future<Output = Result<(), AppError>>;
//     fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool, AppError>>;
//     fn get(&self, key: &str) -> impl std::future::Future<Output = Result<Option<String>, AppError>>;
//     fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool, AppError>>;
//     fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64, AppError>>;
// }

// // 客户端构建实现
// impl ClientBuilder for RedisClient {
//     fn build_from_config(config: &AppConfig) -> Result<redis::Client, AppError> {
//         redis::Client::open(config.redis.get_url()).map_err(|e| {
//             AppError::RedisError(e)
//         })
//     }
// }

// // 测试上下文
// pub struct RedisTestContext {
//     pub config: RedisConfig,
//     pub redis: RedisClient,
// }

// impl AsyncTestContext for RedisTestContext {
//     async fn setup() -> Self {
//         info!("setup redis config for the test");
//         let redis = RedisClient::build_from_config(&CONFIG).expect(
//             "Failed to create Redis client for test"
//         );
//         Self {
//             config: CONFIG.redis.clone(),
//             redis,
//         }
//     }
// }

// // Redis操作实现
// impl RedisClientExt for Client {
//     async fn ping(&self) -> Result<Option<String>, AppError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;

//         let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;

//         info!("ping redis server");
//         Ok(value)
//     }

//     async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), AppError> {
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

//     async fn exist(&self, key: &str) -> Result<bool, AppError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;

//         let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;

//         info!("check key exists: {key}");
//         Ok(value)
//     }

//     async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;

//         let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;

//         info!("get value: {key}");
//         Ok(value)
//     }

//     async fn del(&self, key: &str) -> Result<bool, AppError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;

//         let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;

//         info!("delete value: {key}");
//         Ok(value == 1)
//     }

//     async fn ttl(&self, key: &str) -> Result<i64, AppError> {
//         let mut conn = self.get_multiplexed_async_connection().await?;

//         let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;

//         info!("get TTL value: {key}");
//         Ok(value)
//     }
// }
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

// #[cfg(test)]
// mod tests {
//     use crate::constant::REDIS;

//     use super::*;

//     use fake::{ Fake, Faker };
//     use uuid::Uuid;

//     #[tokio::test]
//     async fn test_ping_redis_server() {
//         let resp = REDIS.ping().await.unwrap();
//         let pong = "PONG";
//         assert!(matches!(resp, Some(p) if p == pong));
//     }

//     #[tokio::test]
//     async fn test_set_key_redis() {
//         let key: String = Faker.fake();
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(5)).await.unwrap();
//         let resp = REDIS.get(&key).await.unwrap();
//         assert!(matches!(resp, Some(v) if v == value));
//         let resp = REDIS.ttl(&key).await.unwrap();
//         assert!(resp > 0);
//     }
//     #[tokio::test]
//     async fn test_set_key_code() {
//         let key: String = "email".to_string();
//         let value = "123456".to_string();
//         REDIS.set(&key, &value, Duration::from_secs(60)).await.unwrap();
//         let resp = REDIS.get(&key).await.unwrap();
//         assert!(matches!(resp, Some(v) if v == value));
//         let resp = REDIS.ttl(&key).await.unwrap();
//         assert!(resp > 0);
//     }
//     #[tokio::test]
//     async fn test_exist_key_redis() {
//         let key: String = Faker.fake();
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
//         let resp = REDIS.get(&key).await.unwrap();
//         assert!(matches!(resp, Some(v) if v == value));
//         let resp = REDIS.exist(&key).await.unwrap();
//         assert!(resp);
//         let key: String = Faker.fake();
//         let resp = REDIS.exist(&key).await.unwrap();
//         assert!(!resp);
//     }

//     #[tokio::test]
//     async fn test_del_key_redis() {
//         let key: String = Faker.fake();
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(4)).await.unwrap();
//         let resp = REDIS.get(&key).await.unwrap();
//         assert!(matches!(resp, Some(v) if v == value));
//         let resp = REDIS.exist(&key).await.unwrap();
//         assert!(resp);
//         REDIS.del(&key).await.unwrap();
//         let resp = REDIS.exist(&key).await.unwrap();
//         assert!(!resp);
//     }

//     #[tokio::test]
//     async fn test_key_ttl_redis() {
//         let key: String = Faker.fake();
//         let ttl = 4;
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(ttl)).await.unwrap();
//         let resp = REDIS.get(&key).await.unwrap();
//         assert!(matches!(resp, Some(v) if v == value));
//         let resp = REDIS.ttl(&key).await.unwrap();
//         assert!(resp <= (ttl as i64) && resp > 0);
//         REDIS.del(&key).await.unwrap();
//         let resp = REDIS.ttl(&key).await.unwrap();
//         assert!(resp < 0);
//     }
//     #[tokio::test]
//     async fn test_get_value_without_key() {
//         let key: String = Faker.fake();
//         let result = REDIS.get(&key).await;
//         assert!(result.is_ok());
//         assert!(result.unwrap().is_none());
//     }

//     #[tokio::test]
//     async fn test_get_value_with_key() {
//         let key: String = Faker.fake();
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(5)).await.unwrap();
//         let result = REDIS.get(&key).await;
//         assert!(result.is_ok());
//         println!("{:?}", result.unwrap());
//         // assert!(matches!(result.unwrap(), Some(v) if v == value));
//     }

//     #[tokio::test]
//     async fn test_get_value_after_expiration() {
//         let key: String = Faker.fake();
//         let value = Uuid::new_v4().to_string();
//         REDIS.set(&key, &value, Duration::from_secs(1)).await.unwrap();
//         tokio::time::sleep(Duration::from_secs(2)).await;
//         let result = REDIS.get(&key).await;
//         assert!(result.is_ok());
//         assert!(result.unwrap().is_none());
//     }
// }
