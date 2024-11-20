use std::sync::Arc;
// use use_case::customer_use_case::{ CustomerUseCase, CustomerUseCaseImpl };
use domain::{
    repositories::{ customer_repository::CustomerRepository, customer_service::CustomerService },
    service::customer_service_impl::CustomerServiceImpl,
};
use shared::error::AppResult;

use infrastructure::{
    client::{
        builder::ClientBuilder,
        database::{ DatabaseClient, DatabaseClientExt },
        email::EmailClient,
        redis::RedisClient,
    },
    config::AppConfig,
    persistence::customer_repository_impl::CustomerRepositoryImpl,
};

// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
    pub email: Arc<EmailClient>,
    // pub producer: Arc<KafkaClientProducer>,
    // pub consumer: Arc<KafkaClientConsumer>,
    // DI
    pub customer_repository: Arc<dyn CustomerRepository>,
    pub customer_service: Arc<dyn CustomerService>,
    // pub customer_use_case: Arc<dyn CustomerUseCase>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let email = Arc::new(EmailClient::build_from_config(&config)?);
        // let producer = Arc::new(KafkaClientProducer::build_from_config(&config).await?);
        // let consumer = Arc::new(KafkaClientConsumer::build_from_config(&config).await?);
        // DI ，这里使用Arc来共享数据，避免数据的复制和所有权的转移，clone()本质上是增加引用计数
        let customer_repository = Arc::new(CustomerRepositoryImpl::new(db.clone(), redis.clone()));
        let customer_service = Arc::new(CustomerServiceImpl::new(customer_repository.clone()));
        // let customer_use_case = Arc::new(CustomerUseCaseImpl::new(customer_repository.clone(), customer_service.clone()));
        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
            email,
            customer_repository,
            customer_service,
            // producer,
            // consumer,
            // customer_use_case,
        })
    }
}
