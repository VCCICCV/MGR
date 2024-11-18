// use domain::repositories::customer_repository::CustomerRepository;
// use infrastructure::{
//     config::{ env::get_env_source, AppConfig },
//     constant::ENV_PREFIX,
//     state::AppState,
//     persistence::customer_repository_impl::CustomerRepositoryImpl,
// };

// #[tokio::test]
// async fn test_find_all() {
//     // 获取 AppState
//     let config = AppConfig::read(get_env_source(ENV_PREFIX)).unwrap();
//     let state_result = AppState::new(config).await;
//     let state = match state_result {
//         Ok(s) => s,
//         Err(e) => {
//             eprintln!("创建AppState失败: {:?}", e);
//             return;
//         }
//     };
//     // 创建CustomerRepository
//     let repository = CustomerRepositoryImpl::new(state.db, state.redis);

//     let result = repository.find_all().await;
//     match result {
//         Ok(customers) => {
//             println!("Customers: {:?}", customers);
//         }
//         Err(err) => panic!("Error: {:?}", err),
//     }

//     // let result  = repository.find_by_email("asdasdsa".to_string()).await;
//     // match result {
//     //     Ok(customer) => {
//     //         println!("Customer: {:?}", customer);
//     //     }
//     //     Err(err) => panic!("Error: {:?}", err),
//     // }
// }
// #[tokio::test]
// async fn test_find_code_by_email() {
//     // 获取 AppState
//     let config = AppConfig::read(get_env_source(ENV_PREFIX)).unwrap();
//     let state_result = AppState::new(config).await;
//     let state = match state_result {
//         Ok(s) => s,
//         Err(e) => {
//             eprintln!("创建AppState失败: {:?}", e);
//             return;
//         }
//     };
//     // 创建CustomerRepository
//     let repository = CustomerRepositoryImpl::new(state.db, state.redis);

//     let code = repository.find_code_by_email("email").await;
//     println!("{:?}", code);
// }
