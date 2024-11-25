use std::sync::Arc;
// use use_case::customer_use_case::{ CustomerUseCase, CustomerUseCaseImpl };
use domain::{
    interface::customer_service::CustomerService,
    model::reponse::error::AppResult,
    repositories::{
        customer_repository::CustomerRepository,
        redis_repository::{ self, RedisRepository },
    },
    service::customer_service_impl::CustomerServiceImpl,
};

use infrastructure::{
    client::{
        builder::ClientBuilder,
        database::{ DatabaseClient, DatabaseClientExt },
        email::EmailClient,
        redis::RedisClient,
    },
    config::AppConfig,
    persistence::{
        customer_repository_impl::CustomerRepositoryImpl,
        redis_repository_impl::RedisRepositoryImpl,
        // redis_repository_impl::RedisRepositoryImpl,
    },
};

// 使用Arc来共享数据，避免数据的复制和所有权的转移
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
    pub email: Arc<EmailClient>,
    // DI
    pub customer_repository: Arc<dyn CustomerRepository>,
    pub customer_service: Arc<dyn CustomerService>,
    pub redis_repository: Arc<dyn RedisRepository>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let email = Arc::new(EmailClient::build_from_config(&config)?);

        // DI ，这里使用Arc来共享数据，避免数据的复制和所有权的转移，clone()本质上是增加引用计数
        let customer_repository = Arc::new(CustomerRepositoryImpl::new(db.clone()));
        let redis_repository = Arc::new(RedisRepositoryImpl::new(redis.clone()));
        let customer_service = Arc::new(
            CustomerServiceImpl::new(customer_repository.clone(), redis_repository.clone())
        );

        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
            email,
            customer_repository,
            customer_service,
            redis_repository,
        })
    }
}
