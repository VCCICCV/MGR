use std::sync::Arc;
use sea_orm::{ConnAcquireErr, DatabaseConnection, DbErr};
use shared::{global::{GLOBAL_DB_POOL, GLOBAL_PRIMARY_DB}, web::error::AppError};

pub async fn get_db_connection() -> Result<Arc<DatabaseConnection>, AppError> {
    let db = GLOBAL_PRIMARY_DB.read().await;
    db.as_ref()
        .cloned()
        .ok_or_else(|| AppError::from(DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)))
}

/// 获取命名数据库连接
#[allow(dead_code)]
pub async fn get_named_connection(name: &str) -> Result<Arc<DatabaseConnection>, AppError> {
    let pools = GLOBAL_DB_POOL.read().await;
    let db = pools.get(name).ok_or_else(|| AppError {
        code: 500,
        message: format!("Database pool '{}' not found", name),
    })?;
    Ok(db.clone())
}
