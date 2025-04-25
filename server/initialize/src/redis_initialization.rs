#![allow(dead_code)]
use config::{redis::{RedisConfig, RedisInstancesConfig, RedisMode}, OptionalConfigs};
use redis::{cluster::ClusterClient, Client};
use shared::global::{get_config, RedisConnection, GLOBAL_PRIMARY_REDIS, GLOBAL_REDIS_POOL};
use tracing::{error, info};

use std::{process, sync::Arc};


/// 初始化主Redis
pub async fn init_primary_redis() {
    if let Some(config) = get_config::<RedisConfig>().await {
        match create_redis_connection(&config).await {
            Ok(connection) => {
                *GLOBAL_PRIMARY_REDIS.write().await = Some(connection);
                info!(
                    "Primary Redis connection initialized ({})",
                    if config.mode == RedisMode::Cluster {
                        "Cluster mode"
                    } else {
                        "Single mode"
                    }
                );
            },
            Err(e) => {
                error!("Failed to initialize primary Redis: {}", e);
                process::exit(1);
            },
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
    let url = config
        .get_url()
        .ok_or_else(|| "URL is required for single mode Redis".to_string())?;

    let client = redis::Client::open(url.as_str())
        .map_err(|e| format!("Failed to create Redis client: {}", e))?;

    test_single_connection(&client).await?;

    Ok(RedisConnection::Single(Arc::new(client)))
}

async fn test_single_connection(client: &Client) -> Result<(), String> {
    let mut con = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| format!("Failed to create connection manager: {}", e))?;

    let _: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

async fn create_cluster_connection(config: &RedisConfig) -> Result<RedisConnection, String> {
    let urls = config
        .get_urls()
        .ok_or_else(|| "URLs are required for cluster mode".to_string())?;

    if urls.is_empty() {
        return Err("Cluster mode requires at least one URL".to_string());
    }

    let client =
        redis::cluster::ClusterClient::new(urls.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .map_err(|e| format!("Failed to create Redis cluster client: {}", e))?;

    test_cluster_connection(&client).await?;

    Ok(RedisConnection::Cluster(Arc::new(client)))
}

async fn test_cluster_connection(client: &ClusterClient) -> Result<(), String> {
    let mut con = client
        .get_async_connection()
        .await
        .map_err(|e| format!("Failed to connect to Redis cluster: {}", e))?;

    let _: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

    Ok(())
}

pub async fn init_redis_pool(
    redis_instances_config: Option<Vec<RedisInstancesConfig>>,
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
            GLOBAL_REDIS_POOL
                .write()
                .await
                .insert(name.to_string(), connection);
            info!("Redis '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to initialize Redis '{}': {}", name, e);
            error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

/// 初始化所有 Redis 连接
pub async fn init_redis_pools() {
    if let Some(redis_instances_config) =
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
    redis_pool
        .remove(name)
        .ok_or_else(|| format!("Redis connection '{}' not found", name))?;
    info!("Redis connection '{}' removed", name);
    Ok(())
}