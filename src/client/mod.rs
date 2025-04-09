pub mod database;
pub mod redis;
pub mod kafka;
use crate::{configure::AppConfig, error::AppError};
// 传输配置文件到客户端
pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> Result<Self,AppError>;
}
