use std::time::Duration;

use domain::model::reponse::error::AppResult;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection };
use tracing::info;
use crate::config::AppConfig;

// 类型别名
pub type DatabaseClient = DatabaseConnection;

pub trait DatabaseClientExt: Sized {
    fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = AppResult<Self>>;
}

impl DatabaseClientExt for DatabaseClient {
    async fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        let mut opt = ConnectOptions::new(config.db.get_url());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);
        let db = Database::connect(opt).await?;
        info!("Database connected");
        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constant::CONFIG;

    #[tokio::test]
    async fn test_ping_database() {
        DatabaseClient::build_from_config(&CONFIG).await
            .unwrap()
            .ping().await
            .expect("Database ping failed.")
    }
}
