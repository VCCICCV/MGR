pub mod env;
pub mod server;
pub mod tracing;
pub mod kafka;
pub mod profile;
pub mod email;
use std::str::FromStr;
use email::EmailConfig;
use kafka::KafkaConfig;
use server::ServerConfig;
use config::{ ConfigError, Environment };
use serde::Deserialize;
use ::tracing::info;

use profile::Profile;

use crate::utils::dir::get_project_root;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub kafka: KafkaConfig,
    pub email:EmailConfig
}

impl AppConfig {
    pub fn read(env_src: Environment) -> anyhow::Result<Self, config::ConfigError> {
        // 获取配置文件目录
        let config_dir = get_settings_dir()?;
        info!("config_dir: {:#?}", config_dir);
        // 获取配置文件环境
        let run_mode = std::env
            ::var("RUN_MODE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_e| Ok(Profile::Dev))?;
        // 当前配置文件名
        let profile_filename = format!("{run_mode}.toml");
        // 获取配置
        let config = config::Config
            ::builder()
            // 添加默认配置
            .add_source(config::File::from(config_dir.join("default.toml")))
            // 添加自定义前缀配置
            .add_source(config::File::from(config_dir.join(profile_filename)))
            // 添加环境变量
            .add_source(env_src)
            .build()?;
        info!("Successfully read config profile: {run_mode}.");
        // 反序列化
        config.try_deserialize()
    }
}
// 获取配置文件目录
pub fn get_settings_dir() -> anyhow::Result<std::path::PathBuf, ConfigError> {
    Ok(
        get_project_root()
            .map_err(|e| ConfigError::Message(e.to_string()))?
            .join("settings")
    )
}