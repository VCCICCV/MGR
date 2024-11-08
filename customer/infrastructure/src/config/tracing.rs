use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct TracingConfig {
    log_level: String,
}
impl TracingConfig {
    // 获取地址
    pub fn get_log_level(&self) -> String {
        format!("{}", self.log_level)
    }
}
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn app_config_http_addr_test() {
        let config = TracingConfig {
            log_level: "debug".to_string(),
        };
        assert_eq!(config.get_log_level(), "debug");
    }
}
