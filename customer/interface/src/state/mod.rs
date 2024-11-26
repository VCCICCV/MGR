use std::sync::Arc;
use application::use_case::{
    customer_use_case::CustomerUseCase,
    customer_use_case_impl::CustomerUseCaseImpl,
};
use domain::{
    model::reponse::error::AppResult,
    repositories::customer_repository::CustomerRepository,
    service::customer_service::CustomerService,
    utils::redis::RedisUtil,
};

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
use infrastructure::domain::customer_service_impl::CustomerServiceImpl;
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
    pub customer_use_case: Arc<dyn CustomerUseCase>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let email = Arc::new(EmailClient::build_from_config(&config)?);

        // DI ，这里使用Arc来共享数据，避免数据的复制和所有权的转移，clone()本质上是增加引用计数
        let customer_repository = Arc::new(CustomerRepositoryImpl::new(db.clone()));
        let customer_service = Arc::new(
            CustomerServiceImpl::new(customer_repository.clone(), redis.clone())
        );
        let customer_use_case = Arc::new(
            CustomerUseCaseImpl::new(db.clone(), customer_service.clone())
        );
        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
            email,
            customer_repository,
            customer_service,
            customer_use_case,
        })
    }
}
