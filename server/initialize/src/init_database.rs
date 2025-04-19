// use std::time::Duration;
// use config::AppConfig;
// use opentelemetry::global;
// use sea_orm::{ ConnectOptions, Database, DatabaseConnection };
// use shared::error::AppError;
// use tracing::info;

// // 类型别名
// pub type DatabaseClient = DatabaseConnection;

// pub trait DatabaseClientExt: Sized + Send + Sync + 'static {
//     fn build_from_config(
//         config: &AppConfig
//     ) -> impl std::future::Future<Output = Result<Self, AppError>>;
// }

// impl DatabaseClientExt for DatabaseClient {
//     async fn build_from_config() -> Result<Self, AppError> {
//         let mut opt = ConnectOptions::new(shared::global::GLOBAL_CONFIG.);

//         opt.max_connections(100)
//             .min_connections(5)
//             .connect_timeout(Duration::from_secs(8))
//             .acquire_timeout(Duration::from_secs(8))
//             .idle_timeout(Duration::from_secs(8))
//             .max_lifetime(Duration::from_secs(8))
//             .sqlx_logging(false)
//             .sqlx_logging_level(log::LevelFilter::Info);
//         let db = Database::connect(opt).await?;
//         info!("Database connected");
//         Ok(db)
//     }
// }

use std::{process, sync::Arc, time::Duration};

use config::{database::{DatabaseConfig, DatabasesInstancesConfig}, OptionalConfigs};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::global::{get_config, GLOBAL_DB_POOL, GLOBAL_PRIMARY_DB};
use tracing::{error, info};

// 连接
pub async fn init_primary_connection() {
    let db_config = get_config::<DatabaseConfig>().await.unwrap();
    let opt = build_connect_options(&db_config);
    match Database::connect(opt).await {
        Ok(db) => {
            *GLOBAL_PRIMARY_DB.write().await = Some(Arc::new(db));
            info!("Primary database connection initialized");
        },
        Err(e) => {
            error!("Failed to connect to primary database: {}", e);
            process::exit(1);
        },
    }
}

/// 初始化多数据库连接
pub async fn init_db_pools() {
    if let Some(databases_instances_config) =
        get_config::<OptionalConfigs<DatabasesInstancesConfig>>().await
    {
        if let Some(databases_instances) = &databases_instances_config.configs {
            let _ = init_db_pool_connections(Some(databases_instances.clone())).await;
        }
    }
}

pub async fn init_db_pool_connections(
    databases_config: Option<Vec<DatabasesInstancesConfig>>,
) -> Result<(), String> {
    if let Some(dbs) = databases_config {
        for db_config in dbs {
            init_db_connection(&db_config.name, &db_config.database).await?;
        }
    }
    Ok(())
}

async fn init_db_connection(name: &str, db_config: &DatabaseConfig) -> Result<(), String> {
    let opt = build_connect_options(db_config);
    match Database::connect(opt).await {
        Ok(db) => {
            GLOBAL_DB_POOL
                .write()
                .await
                .insert(name.to_string(), Arc::new(db));
            info!("Database '{}' initialized", name);
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to connect to database '{}': {}", name, e);
            error!("{}", error_msg);
            Err(error_msg)
        },
    }
}

fn build_connect_options(db_config: &DatabaseConfig) -> ConnectOptions {
    let mut opt = ConnectOptions::new(db_config.url.clone());
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout))
        .sqlx_logging(false);
    opt
}

pub async fn get_primary_db_connection() -> Option<Arc<DatabaseConnection>> {
    GLOBAL_PRIMARY_DB.read().await.clone()
}

pub async fn get_db_pool_connection(name: &str) -> Option<Arc<DatabaseConnection>> {
    GLOBAL_DB_POOL.read().await.get(name).cloned()
}

pub async fn add_or_update_db_pool_connection(
    name: &str,
    db_config: &DatabaseConfig,
) -> Result<(), String> {
    init_db_connection(name, db_config).await
}

pub async fn remove_db_pool_connection(name: &str) -> Result<(), String> {
    let mut db_pool = GLOBAL_DB_POOL.write().await;
    db_pool
        .remove(name)
        .ok_or_else(|| "Connection not found".to_string())?;
    info!("Database connection '{}' removed", name);
    Ok(())
}