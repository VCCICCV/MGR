pub mod dir;
pub mod hash;
pub mod ip;
pub mod api_key;
pub mod api_key_middleware;
pub mod jwt;
pub mod secure_util;
pub mod tree_util;
pub mod memory_nonce_store;
pub mod nonce_store;
pub mod redis_nonce_store;



pub use api_key::{
    ApiKeyConfig,
    ComplexApiKeyValidator,
    MemoryNonceStore,
    SignatureAlgorithm,
    SimpleApiKeyValidator,
};
pub use api_key_middleware::{
    api_key_middleware,
    protect_route,
    ApiKeySource,
    ApiKeyValidation,
    ComplexApiKeyConfig,
    SimpleApiKeyConfig,
};

use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

pub enum ValidatorType {
    Simple,
    Complex,
}

static API_KEY_VALIDATORS: Lazy<
    (Arc<RwLock<SimpleApiKeyValidator>>, Arc<RwLock<ComplexApiKeyValidator>>)
> = Lazy::new(|| {
    (
        Arc::new(RwLock::new(SimpleApiKeyValidator::new())),
        Arc::new(RwLock::new(ComplexApiKeyValidator::new(None))),
    )
});

pub async fn get_simple_validator() -> SimpleApiKeyValidator {
    API_KEY_VALIDATORS.0.read().await.clone()
}

pub async fn get_complex_validator() -> ComplexApiKeyValidator {
    API_KEY_VALIDATORS.1.read().await.clone()
}

pub async fn add_key(validator_type: ValidatorType, key: &str, secret: Option<&str>) {
    match validator_type {
        ValidatorType::Simple => {
            API_KEY_VALIDATORS.0.write().await.add_key(key.to_string());
        }
        ValidatorType::Complex => {
            if let Some(secret) = secret {
                API_KEY_VALIDATORS.1.write().await.add_key_secret(
                    key.to_string(),
                    secret.to_string()
                );
            }
        }
    }
}

pub async fn remove_key(validator_type: ValidatorType, key: &str) {
    match validator_type {
        ValidatorType::Simple => {
            API_KEY_VALIDATORS.0.write().await.remove_key(key);
        }
        ValidatorType::Complex => {
            API_KEY_VALIDATORS.1.write().await.remove_key(key);
        }
    }
}

pub async fn init_validators(config: Option<ApiKeyConfig>) {
    let complex_validator = ComplexApiKeyValidator::new(config);
    *API_KEY_VALIDATORS.1.write().await = complex_validator;
}

#[derive(Debug, Clone)]
pub struct ApiKeyEvent {
    pub api_key: String,
}
