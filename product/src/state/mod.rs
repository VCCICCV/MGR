use std::sync::Arc;
use crate::{configure::AppConfig, error::AppResult, infrastructure::client::{builder::ClientBuilder, database::{DatabaseClient, DatabaseClientExt}, redis::RedisClient}};
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);

        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
        })
    }
}
