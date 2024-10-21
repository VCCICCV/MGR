use std::sync::Arc;

use anyhow::Error;
use crate::{client::{builder::ClientBuilder, database::{DatabaseClient, DatabaseClientExt}, redis::RedisClient}, config::AppConfig, persistence::user_repository_impl};
// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
  pub config: Arc<AppConfig>,
  pub redis: Arc<RedisClient>,
  pub db: Arc<DatabaseClient>,
  // pub user_repository: UserRepositoryImpl,
}

impl AppState {
  pub async fn new(config: AppConfig) -> Result<Self,Error> {
    let redis = Arc::new(RedisClient::build_from_config(&config)?);
    let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
    // let user_repository = UserRepositoryImpl::new(*db.clone());
    Ok(Self {
      config: Arc::new(config),
      db,
      redis,
      // user_repository,
    })
  }
}
