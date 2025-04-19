use config::jwt::JwtConfig;
use shared::{global, Validation};
use tracing::{error, info};
use tokio::sync::Mutex;
use std::sync::Arc;
pub async fn initialize_keys_and_validation() {
    let jwt_config = match global::get_config::<JwtConfig>().await {
        Some(cfg) => cfg,
        None => {
            error!("Failed to load JWT config");
            return;
        }
    };

    let keys = global::Keys::new(jwt_config.jwt_secret.as_bytes());
    if global::KEYS.set(Arc::new(Mutex::new(keys))).is_err() {
        error!("Failed to set KEYS");
    }

    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[&jwt_config.issuer]);
    if global::VALIDATION.set(Arc::new(Mutex::new(validation))).is_err() {
        error!("Failed to set VALIDATION");
    }

    info!("JWT keys and validation initialized");
}
