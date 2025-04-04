use std::sync::Arc;
use application::executor::{
    customer_use_case::CustomerUseCase,
    customer_use_case_impl::CustomerUseCaseImpl,
};
use domain::{
    model::reponse::error::AppResult,
    repositories::customer_repository::CustomerRepository,
    service::{ customer_service::CustomerService, customer_service_impl::CustomerServiceImpl },
    utils::{ redis::RedisUtil, session::Session, token::Token },
};

use infrastructure::{
    client::{
        builder::ClientBuilder,
        database::{ DatabaseClient, DatabaseClientExt },
        email::EmailClient,
        redis::RedisClient,
    },
    config::AppConfig,
    domain::utils::{ redis_impl::RedisUtilImpl, session_impl::SessionImpl, token_impl::TokenImpl },
    persistence::customer_repository_impl::CustomerRepositoryImpl,
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
    pub customer_use_case: Arc<dyn CustomerUseCase>,
    // util
    pub redis_util: Arc<dyn RedisUtil>,
    pub session: Arc<dyn Session>,
    pub token: Arc<dyn Token>,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let redis = Arc::new(RedisClient::build_from_config(&config)?);
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let email = Arc::new(EmailClient::build_from_config(&config)?);

        // DI ，这里使用Arc来共享数据，避免数据的复制和所有权的转移，clone()本质上是增加引用计数
        // util
        let redis_util = Arc::new(RedisUtilImpl::new(redis.clone()));
        let session = Arc::new(SessionImpl::new(redis.clone()));
        let customer_repository = Arc::new(CustomerRepositoryImpl::new(db.clone()));
        let token = Arc::new(TokenImpl::new());

        let customer_service = Arc::new(
            CustomerServiceImpl::new(
                customer_repository.clone(),
                redis_util.clone(),
                session.clone(),
                token.clone()
            )
        );
        let customer_use_case = Arc::new(
            CustomerUseCaseImpl::new(
                db.clone(),
                customer_service.clone(),
                customer_repository.clone(),
                session.clone()
            )
        );

        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
            email,
            customer_repository,
            customer_service,
            customer_use_case,
            redis_util,
            session,
            token,
        })
    }
}
