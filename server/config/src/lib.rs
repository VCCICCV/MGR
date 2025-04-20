pub mod database;
pub mod redis;
pub mod server;
pub mod kafka;
pub mod profile;
pub mod jwt;
pub mod config;
use config::Config;
use shared::global;
use thiserror::Error;
use tokio::fs;
use tracing::{ error, info };
use std::path::Path;
use database::{ DatabaseConfig, DatabasesInstancesConfig };
use jwt::JwtConfig;

use redis::{ RedisConfig, RedisInstancesConfig };
use server::ServerConfig;

/// 可选配置集合的包装类
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OptionalConfigs<T> {
    pub configs: Option<Vec<T>>,
}

impl<T> From<Option<Vec<T>>> for OptionalConfigs<T> {
    fn from(configs: Option<Vec<T>>) -> Self {
        Self { configs }
    }
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")] ReadError(#[from] std::io::Error),
    #[error("Failed to parse YAML config: {0}")] YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse TOML config: {0}")] TomlError(#[from] toml::de::Error),
    #[error("Failed to parse JSON config: {0}")] JsonError(#[from] serde_json::Error),
    #[error("Unsupported config file format: {0}")] UnsupportedFormat(String),
}

async fn parse_config(file_path: &str, content: String) -> Result<Config, ConfigError> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
        "toml" => Ok(toml::from_str(&content)?),
        "json" => Ok(serde_json::from_str(&content)?),
        _ => Err(ConfigError::UnsupportedFormat(extension)),
    }
}

pub async fn init_from_file(file_path: &str) -> Result<(), ConfigError> {
    let config_data = fs::read_to_string(file_path).await.map_err(|e| {
        error!("Failed to read config file: {}", e);
        ConfigError::ReadError(e)
    })?;

    let config = parse_config(file_path, config_data).await.map_err(|e| {
        error!("Failed to parse config file: {}", e);
        e
    })?;

    global::init_config::<Config>(config.clone()).await;
    global::init_config::<DatabaseConfig>(config.database).await;

    global::init_config::<OptionalConfigs<DatabasesInstancesConfig>>(
        config.database_instances.into()
    ).await;

    global::init_config::<ServerConfig>(config.server).await;
    global::init_config::<JwtConfig>(config.jwt).await;

    if let Some(redis_config) = config.redis {
        global::init_config::<RedisConfig>(redis_config).await;
    }
    global::init_config::<OptionalConfigs<RedisInstancesConfig>>(
        config.redis_instances.into()
    ).await;
    info!("Configuration initialized successfully");
    Ok(())
}
