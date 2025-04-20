use std::sync::Arc;

use super::{memory_nonce_store, redis_nonce_store};

/// Nonce storage enum that supports different storage implementations
#[derive(Clone)]
pub enum NonceStore {
    /// In-memory storage implementation
    Memory(Arc<memory_nonce_store::MemoryNonceStore>),
    /// Redis storage implementation
    Redis(Arc<redis_nonce_store::RedisNonceStore>),
}

impl NonceStore {
    /// Checks and sets a nonce
    ///
    /// # Arguments
    /// * `nonce` - The nonce string to validate and store
    ///
    /// # Returns
    /// * `true` - If the nonce is valid and has not been used before
    /// * `false` - If the nonce is invalid or has been used before
    pub async fn check_and_set(&self, nonce: &str) -> bool {
        match self {
            NonceStore::Memory(store) => store.check_and_set(nonce).await,
            NonceStore::Redis(store) => store.check_and_set(nonce).await,
        }
    }
}

/// Factory function type for creating NonceStore instances
pub type NonceStoreFactory = Arc<dyn Fn() -> NonceStore + Send + Sync>;

/// Creates an in-memory version of NonceStore
pub fn create_memory_store() -> NonceStore {
    NonceStore::Memory(Arc::new(
        memory_nonce_store::MemoryNonceStore::new(),
    ))
}

/// Creates a Redis version of NonceStore
pub fn create_redis_store(prefix: impl Into<String>) -> NonceStore {
    NonceStore::Redis(Arc::new(
        redis_nonce_store::RedisNonceStore::new(prefix),
    ))
}

/// Creates a factory function for in-memory NonceStore
pub fn create_memory_store_factory() -> NonceStoreFactory {
    Arc::new(|| create_memory_store())
}

/// Creates a factory function for Redis NonceStore
pub fn create_redis_store_factory(
    prefix: impl Into<String> + Clone + Send + Sync + 'static,
) -> NonceStoreFactory {
    let prefix = prefix.into();
    Arc::new(move || create_redis_store(prefix.clone()))
}
