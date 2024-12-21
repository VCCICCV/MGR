use once_cell::sync::Lazy;
use crate::configure::env::get_env_source;
// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// 配置
pub static CONFIG: Lazy<crate::configure::AppConfig> = Lazy::new(||
    crate::configure::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
);
