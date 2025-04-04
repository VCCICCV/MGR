

use domain::model::reponse::error::AppResult;

use crate::config::AppConfig;
// 传输配置文件到客户端
pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> AppResult<Self>;
}
