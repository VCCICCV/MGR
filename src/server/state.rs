use std::sync::Arc;

use crate::{
    client::{
        database::{ DatabaseClient, DatabaseClientExt },
        kafka::{ KafkaClientConsumer, KafkaClientExt, KafkaClientProducer },
        redis::RedisClient,
    },
    configure::AppConfig,
    error::AppError,
};
use crate::client::ClientBuilder;

// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
    pub kafka_producer: Arc<KafkaClientProducer>,
    pub kafka_consumer: Arc<KafkaClientConsumer>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let config = Arc::new(config);
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let kafka_producer = Arc::new(KafkaClientProducer::build_from_config(&config).await?);
        let kafka_consumer = Arc::new(KafkaClientConsumer::build_from_config(&config).await?);
        Ok(Self {
            config,
            db,
            redis,
            kafka_producer,
            kafka_consumer,
        })
    }
}
