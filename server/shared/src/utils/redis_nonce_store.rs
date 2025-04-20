use crate::global::{self, RedisConnection};

use super::api_key::NONCE_TTL_SECS;

/// Redis implementation of Nonce storage
#[derive(Clone)]
pub struct RedisNonceStore {
    /// Redis key prefix
    prefix: String,
}

impl RedisNonceStore {
    /// Creates a new RedisNonceStore instance
    #[inline]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }

    /// Gets the complete Redis key name
    fn get_key(&self, nonce: &str) -> String {
        format!("{}_nonce:{}", self.prefix, nonce)
    }
}

impl Default for RedisNonceStore {
    #[inline]
    fn default() -> Self {
        Self::new("api_key")
    }
}

impl RedisNonceStore {
    pub async fn check_and_set(&self, nonce: &str) -> bool {
        let redis_connection = match global::GLOBAL_PRIMARY_REDIS
            .read()
            .await
            .clone()
        {
            Some(conn) => conn,
            None => return false,
        };

        let key = self.get_key(nonce);

        match redis_connection {
            RedisConnection::Single(client) => {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let result: Option<bool> = redis::cmd("SET")
                        .arg(&key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(NONCE_TTL_SECS)
                        .query_async(&mut conn)
                        .await
                        .ok();

                    result.unwrap_or(false)
                } else {
                    false
                }
            },
            RedisConnection::Cluster(client) => {
                if let Ok(mut conn) = client.get_async_connection().await {
                    let result: Option<bool> = redis::cmd("SET")
                        .arg(&key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(NONCE_TTL_SECS)
                        .query_async(&mut conn)
                        .await
                        .ok();

                    result.unwrap_or(false)
                } else {
                    false
                }
            },
        }
    }
}

// Note: This function has been replaced by the one in nonce_store.rs
// We keep it for backward compatibility, but it now just calls the function in nonce_store.rs
/// Creates a factory function for Redis NonceStore
pub fn create_redis_nonce_store_factory(
    prefix: impl Into<String> + Clone + Send + Sync + 'static,
) -> super::nonce_store::NonceStoreFactory {
    super::nonce_store::create_redis_store_factory(prefix)
}
