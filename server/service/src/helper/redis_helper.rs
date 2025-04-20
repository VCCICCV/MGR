#![allow(dead_code)]
use redis::{aio::MultiplexedConnection, cluster_async::ClusterConnection, ErrorKind, RedisError};
use shared::{global::{RedisConnection, GLOBAL_PRIMARY_REDIS, GLOBAL_REDIS_POOL}, web::error::AppError};
/// Redis连接来源
#[derive(Debug, Clone)]
pub enum RedisSource {
    /// 主Redis实例
    Primary,
    /// 命名的Redis实例
    Named(String),
}

/// 获取Redis连接
pub async fn get_redis_connection(source: RedisSource) -> Result<MultiplexedConnection, AppError> {
    match source {
        RedisSource::Primary => {
            let redis = GLOBAL_PRIMARY_REDIS.read().await.clone().ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis not initialized",
                )))
            })?;
            match redis {
                RedisConnection::Single(client) => {
                    Ok(client.get_multiplexed_async_connection().await?)
                },
                RedisConnection::Cluster(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis is not Single mode",
                )))),
            }
        },
        RedisSource::Named(name) => {
            let pools = GLOBAL_REDIS_POOL.read().await;
            let redis = pools.get(&name).ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Redis pool not found",
                )))
            })?;
            match redis {
                RedisConnection::Single(client) => {
                    Ok(client.get_multiplexed_async_connection().await?)
                },
                RedisConnection::Cluster(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Named Redis is not Single mode",
                )))),
            }
        },
    }
}

/// 获取Redis集群连接
pub async fn get_redis_cluster_connection(
    source: RedisSource,
) -> Result<ClusterConnection, AppError> {
    match source {
        RedisSource::Primary => {
            let redis = GLOBAL_PRIMARY_REDIS.read().await.clone().ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis not initialized",
                )))
            })?;
            match redis {
                RedisConnection::Single(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Primary Redis is not Cluster mode",
                )))),
                RedisConnection::Cluster(client) => Ok(client.get_async_connection().await?),
            }
        },
        RedisSource::Named(name) => {
            let pools = GLOBAL_REDIS_POOL.read().await;
            let redis = pools.get(&name).ok_or_else(|| {
                AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Redis pool not found",
                )))
            })?;
            match redis {
                RedisConnection::Single(_) => Err(AppError::from(RedisError::from((
                    ErrorKind::IoError,
                    "Named Redis is not Cluster mode",
                )))),
                RedisConnection::Cluster(client) => Ok(client.get_async_connection().await?),
            }
        },
    }
}
