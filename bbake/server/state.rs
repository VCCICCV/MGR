use std::sync::Arc;

use crate::{
    client::{
        database::{ DatabaseClient, DatabaseClientExt },
        kafka::{ KafkaClientConsumer, KafkaClientExt, KafkaClientProducer },
        redis::RedisClient,
        ClientBuilder,
    },
    configure::AppConfig,
    service::admin::sys_user_service::{ SysUserService, SysUserServiceImpl },
    shared::error::AppError,
};

// 使用Arc来共享数据，避免数据的复制和所有权的转移
// 在https://github.com/get-eventually/eventually-rs/tree/main作者的指导下不必要的客户端移除Arc
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: RedisClient,
    pub db: DatabaseClient,
    pub kafka_producer: KafkaClientProducer,
    pub kafka_consumer: Arc<KafkaClientConsumer>,
    pub sys_user_service: Arc<dyn SysUserService+'static>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let config = Arc::new(config);
        let redis = RedisClient::build_from_config(&config)?;
        let db = DatabaseClient::build_from_config(&config).await?;
        let kafka_producer = KafkaClientProducer::build_from_config(&config).await?;
        let kafka_consumer = Arc::new(KafkaClientConsumer::build_from_config(&config).await?);
        let sys_user_service = Arc::new(SysUserServiceImpl::new(db.clone(),redis.clone()));
        Ok(Self {
            config,
            db,
            redis,
            kafka_producer,
            kafka_consumer,
            sys_user_service,
        })
    }
}
