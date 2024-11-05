use domain::repositories::customer_repository::CustomerRepository;
use infrastructure::{
    config::{ env::get_env_source, AppConfig },
    constant::ENV_PREFIX,
    state::AppState,
    persistence::customer_repository_impl::CustomerRepositoryImpl,
};

#[tokio::test]
async fn test_find_all() {
    // 获取 AppState
    let config = AppConfig::read(get_env_source(ENV_PREFIX)).unwrap();
    let state = AppState::new(config).await;
    // 创建CustomerRepository
    let repository = CustomerRepositoryImpl::new(state.unwrap().db);

    let result = repository.find_all().await;
    match result {
        Ok(customers) => {
            println!("Customers: {:?}", customers);
        }
        Err(err) => panic!("Error: {:?}", err),
    }
}