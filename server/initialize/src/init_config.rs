use config::database::{ DatabaseConfig, DatabasesInstancesConfig };
use config::jwt::JwtConfig;
use config::redis::{RedisConfig, RedisInstancesConfig};
use config::server::ServerConfig;
use config::{AppConfig, OptionalConfigs};
use shared::constant::ENV_PREFIX;
use config::env::get_env_source;
use shared::global;
pub async fn init_config() {
    // 读取配置
    let config = AppConfig::read(get_env_source(ENV_PREFIX)).expect("读取出错");
    // 写入全局
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
}
