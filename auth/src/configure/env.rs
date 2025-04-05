use std::str::FromStr;
use config::ConfigError;
use super::profile::Profile;
// 获取环境变量配置，例如本例中prefix为"APP"，环境变量前缀分隔符和环境变量分隔符都设置为"__"
// 使用，则将环境变量中的DATABASE_URL作为配置项
pub fn get_env_source(prefix: &str) -> config::Environment {
    // 创建新的环境变量配置
  config::Environment::with_prefix(prefix)
  // 设置环境变量前缀分隔符和环境变量分隔符
    .prefix_separator("__")
    .separator("__")
}
// 从环境变量中获取profile，开发环境还是测试环境
pub fn get_profile() -> Result<Profile, config::ConfigError> {
  std::env::var("RUN_MODE")
    .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
    .unwrap_or_else(|_e| Ok(Profile::Dev))
}
