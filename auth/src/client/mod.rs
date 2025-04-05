pub mod database;
pub mod redis;
pub mod kafka;
use crate::configure::AppConfig;
// 传输配置文件到客户端
pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> anyhow::Result<Self, anyhow::Error>;
}
