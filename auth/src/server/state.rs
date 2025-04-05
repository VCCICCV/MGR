use std::sync::Arc;

use crate::{client::{database::{DatabaseClient, DatabaseClientExt}, redis::RedisClient}, configure::AppConfig};
use crate::client::ClientBuilder;

// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
        })
    }
}
