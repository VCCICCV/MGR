use std::sync::Arc;
use crate::configure;
use configure::AppConfig;

// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let config = Arc::new(config);
        Ok(Self {
            config,
        })
    }
}
