use moka::sync::Cache;

use super::api_key::NONCE_TTL_SECS;

/// In-memory store for managing nonces with automatic expiration.
///
/// Uses moka cache to store nonces with a TTL of 10 minutes. Once a nonce
/// expires, it can be reused. This helps prevent replay attacks while
/// maintaining memory efficiency.
#[derive(Clone)]
pub struct MemoryNonceStore {
    nonces: Cache<String, ()>,
}

impl MemoryNonceStore {
    /// Creates a new instance of MemoryNonceStore with a 10-minute TTL.
    #[inline]
    pub fn new() -> Self {
        Self {
            nonces: Cache::builder()
                .time_to_live(std::time::Duration::from_secs(NONCE_TTL_SECS))
                .build(),
        }
    }

    /// Validates and stores a nonce.
    ///
    /// # Arguments
    /// * `nonce` - The nonce string to validate and store
    ///
    /// # Returns
    /// * `true` if the nonce is valid and not previously used
    /// * `false` if the nonce is invalid or has been used before
    #[inline]
    pub async fn check_and_set(&self, nonce: &str) -> bool {
        if self.nonces.contains_key(nonce) {
            false
        } else {
            self.nonces.insert(nonce.to_string(), ());
            true
        }
    }
}

impl Default for MemoryNonceStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// Note: This function has been replaced by the one in nonce_store.rs
// We keep it for backward compatibility, but it now just calls the function in nonce_store.rs
/// Creates a factory function for in-memory NonceStore
pub fn create_memory_nonce_store_factory() -> super::nonce_store::NonceStoreFactory {
    super::nonce_store::create_memory_store_factory()
}
