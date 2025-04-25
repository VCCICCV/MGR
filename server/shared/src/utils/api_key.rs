use md5::{Digest, Md5};
use parking_lot::RwLock;
use ring::{digest, hmac};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use super::nonce_store::{create_memory_store_factory, NonceStore, NonceStoreFactory};

/// Supported signature algorithms for API key validation.
///
/// These algorithms are used to generate and validate signatures for API requests.
/// The algorithms are ordered by performance (fastest to slowest).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignatureAlgorithm {
    /// MD5 signature algorithm (default, fastest)
    Md5,
    /// SHA1 signature algorithm
    Sha1,
    /// SHA256 signature algorithm
    Sha256,
    /// HMAC-SHA256 signature algorithm (most secure)
    HmacSha256,
}

impl Default for SignatureAlgorithm {
    #[inline]
    fn default() -> Self {
        Self::Md5
    }
}

/// Configuration for API key validation.
///
/// This struct holds configuration options for the API key validation system.
/// It is designed to be lightweight and efficiently cloneable.
#[derive(Debug, Clone, Copy)]
pub struct ApiKeyConfig {
    /// The signature algorithm to use for request validation
    pub algorithm: SignatureAlgorithm,
}

impl Default for ApiKeyConfig {
    #[inline]
    fn default() -> Self {
        Self {
            algorithm: SignatureAlgorithm::default(),
        }
    }
}

/// Constants for validation timeouts and expiration.
pub const NONCE_TTL_SECS: u64 = 600; // 10 minutes
pub const TIMESTAMP_DISPARITY_MS: i64 = 300_000; // 5 minutes

/// Capacity hints for collections
const DEFAULT_CAPACITY: usize = 32;

/// Simple API key validator that checks against a set of predefined keys.
///
/// This validator provides basic API key validation by comparing against a set of valid keys.
/// Keys are stored permanently and can only be modified through explicit API calls.
#[derive(Clone)]
pub struct SimpleApiKeyValidator {
    keys: Arc<RwLock<HashMap<String, ()>>>,
}

impl SimpleApiKeyValidator {
    /// Creates a new SimpleApiKeyValidator with an empty set of valid keys.
    #[inline]
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
        }
    }

    /// Validates if an API key is valid.
    ///
    /// # Arguments
    /// * `key` - The API key to validate
    ///
    /// # Returns
    /// * `true` if the key is valid
    /// * `false` if the key is invalid
    #[inline]
    pub fn validate_key(&self, key: &str) -> bool {
        self.keys.read().contains_key(key)
    }

    /// Adds a new valid API key.
    ///
    /// # Arguments
    /// * `key` - The API key to add
    #[inline]
    pub fn add_key(&self, key: String) {
        self.keys.write().insert(key, ());
    }

    /// Removes an API key from the set of valid keys.
    ///
    /// # Arguments
    /// * `key` - The API key to remove
    #[inline]
    pub fn remove_key(&self, key: &str) {
        self.keys.write().remove(key);
    }
}

impl Default for SimpleApiKeyValidator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Complex API key validator that supports multiple signature algorithms and nonce validation.
///
/// This validator provides advanced API key validation with features including:
/// - Multiple signature algorithms (MD5, SHA1, SHA256, HMAC-SHA256)
/// - Timestamp validation to prevent replay attacks
/// - Nonce validation with automatic expiration
/// - URL parameter signing
///
/// API keys and their corresponding secrets are stored permanently and can only be
/// modified through explicit API calls.
#[derive(Clone)]
pub struct ComplexApiKeyValidator {
    secrets: Arc<RwLock<HashMap<String, String>>>,
    nonce_store: NonceStore,
    nonce_store_factory: NonceStoreFactory,
    config: ApiKeyConfig,
}

impl ComplexApiKeyValidator {
    /// Creates a new ComplexApiKeyValidator with optional configuration.
    ///
    /// # Arguments
    /// * `config` - Optional API key validation configuration. If None, uses default configuration.
    #[inline]
    pub fn new(config: Option<ApiKeyConfig>) -> Self {
        Self::with_nonce_store(config, create_memory_store_factory())
    }

    /// 创建一个新的 ComplexApiKeyValidator 实例，使用指定的 nonce 存储工厂函数
    ///
    /// # 参数
    /// * `config` - 可选的 API 密钥验证配置。如果为 None，则使用默认配置。
    /// * `nonce_store_factory` - 用于创建 nonce 存储的工厂函数
    #[inline]
    pub fn with_nonce_store(
        config: Option<ApiKeyConfig>,
        nonce_store_factory: NonceStoreFactory,
    ) -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
            nonce_store: (nonce_store_factory)(),
            nonce_store_factory,
            config: config.unwrap_or_default(),
        }
    }

    /// 获取一个新的 nonce 存储实例
    #[inline]
    pub fn get_new_nonce_store(&self) -> NonceStore {
        (self.nonce_store_factory)()
    }

    /// Validates if a timestamp is within the allowed 5-minute window.
    #[inline]
    fn validate_timestamp(&self, timestamp: i64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        (now - timestamp).abs() < TIMESTAMP_DISPARITY_MS
    }

    /// Calculates signature for a signing string using the configured algorithm.
    ///
    /// # Arguments
    /// * `signing_string` - The string to sign
    /// * `secret` - The secret key to use for signing
    ///
    /// # Returns
    /// The calculated signature as a hexadecimal string
    #[inline]
    pub fn calculate_signature(&self, signing_string: &str, secret: &str) -> String {
        let signing_string = format!("{}&key={}", signing_string, secret);
        match self.config.algorithm {
            SignatureAlgorithm::Md5 => {
                let mut hasher = Md5::new();
                hasher.update(signing_string.as_bytes());
                hex::encode(hasher.finalize())
            },
            SignatureAlgorithm::Sha1 => {
                let mut context = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::Sha256 => {
                let mut context = digest::Context::new(&digest::SHA256);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::HmacSha256 => {
                let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
                let tag = hmac::sign(&key, signing_string.as_bytes());
                hex::encode(tag.as_ref())
            },
        }
    }

    /// Validates a signed API request.
    ///
    /// # Arguments
    /// * `api_key` - The API key to validate
    /// * `params` - Vector of key-value pairs representing request parameters
    /// * `signature` - The signature to validate
    /// * `timestamp` - Request timestamp in milliseconds since UNIX epoch
    /// * `nonce` - Unique request identifier to prevent replay attacks
    ///
    /// # Returns
    /// * `true` if the request is valid
    /// * `false` if any validation check fails
    pub fn validate_signature(
        &self,
        api_key: &str,
        params: &[(String, String)],
        signature: &str,
        timestamp: i64,
        nonce: &str,
    ) -> bool {
        if !self.validate_timestamp(timestamp) {
            return false;
        }

        let check_result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.nonce_store.check_and_set(nonce).await })
        });

        if !check_result {
            return false;
        }

        let secrets_guard = self.secrets.read();
        let secret = match secrets_guard.get(api_key) {
            Some(s) => Cow::Borrowed(s),
            None => return false,
        };

        // Pre-allocate with capacity to avoid reallocations
        let mut sorted_params: Vec<_> = Vec::with_capacity(params.len());
        sorted_params.extend_from_slice(params);
        sorted_params.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // Pre-calculate total length to avoid reallocations
        let total_len = sorted_params.iter().fold(0, |acc, (k, v)| {
            acc + k.len() + v.len() + 2 // +2 for '=' and '&'
        });

        let mut signing_string = String::with_capacity(total_len);
        for (i, (k, v)) in sorted_params.iter().enumerate() {
            if i > 0 {
                signing_string.push('&');
            }
            signing_string.push_str(k);
            signing_string.push('=');
            // Only URL encode if necessary
            if v.chars().any(|c| !c.is_ascii_alphanumeric()) {
                signing_string.push_str(&urlencoding::encode(v));
            } else {
                signing_string.push_str(v);
            }
        }

        self.calculate_signature(&signing_string, &secret) == signature
    }

    /// Adds a new API key and its corresponding secret.
    ///
    /// # Arguments
    /// * `key` - The API key to add
    /// * `secret` - The secret corresponding to the API key
    #[inline]
    pub fn add_key_secret(&self, key: String, secret: String) {
        self.secrets.write().insert(key, secret);
    }

    /// Removes an API key and its secret.
    ///
    /// # Arguments
    /// * `key` - The API key to remove
    #[inline]
    pub fn remove_key(&self, key: &str) {
        self.secrets.write().remove(key);
    }

    /// Updates the API key validation configuration.
    ///
    /// # Arguments
    /// * `config` - New API key validation configuration
    #[inline]
    pub fn update_config(&mut self, config: ApiKeyConfig) {
        self.config = config;
    }
}