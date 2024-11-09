// use application::{
//     dto::{ request_command::CustomerCommand, response_dto::Res },
//     use_case::customer_use_case::CustomerUseCase,
// };
// use axum::{ extract::State, response::IntoResponse };
// use infrastructure::{
//     persistence::customer_repository_impl::CustomerRepositoryImpl,
//     state::AppState,
// };
// pub async fn get_all(State(app_state): State<AppState>) -> impl IntoResponse {
//     let customer_repository_impl = CustomerRepositoryImpl::new(app_state.db.clone());
//     let use_case = CustomerUseCase::new(customer_repository_impl);
//     let customers = use_case.get_all().await;
//     match customers {
//         Ok(customers) => Res::with_data(customers),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// pub async fn create_customer(
//     State(app_state): State<AppState>,
//     command: axum::extract::Json<CustomerCommand>
// ) -> impl IntoResponse {
//     let customer_repository_impl = CustomerRepositoryImpl::new(app_state.db.clone());
//     let use_case = CustomerUseCase::new(customer_repository_impl);
//     let customer = use_case.create(command.0).await;
//     match customer {
//         Ok(msg) => Res::<String>::with_msg(&msg),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// pub async fn send_mail(
//     State(app_state): State<AppState>,
//     email: axum::extract::Path<String>
// ) -> impl IntoResponse {
//     let customer_repository_impl = CustomerRepositoryImpl::new(app_state.db.clone());
//     let use_case = CustomerUseCase::new(customer_repository_impl);
//     let code = use_case.send_mail(email).await;
//     match code {
//         Ok(msg) => Res::<String>::with_msg(&msg),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// pub async fn register_customer(
//     State(app_state): State<AppState>,
//     command: axum::extract::Json<CustomerCommand>
// ) {}
// DI：我们把查询用户的trait和命令用户的trait注入到handler中

// pub async fn register_handler(
//     State(state): State<AppState>,
//     user_name: Path<String>,
//     password: Path<String>
// ) -> impl IntoResponse {
//     let user_command = UserRegisterCommand::new(user_name, password);
//     // 创建用例实例
//     let user_use_case = user_use_case::UserUseCase::new(state.user_repository);

//     let result = UserUseCase.execute(user_command).await;
//     match result {
//         Ok(user) => Res::with_data(user),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// Res::with_data("register".to_string())
// pub async fn list_users() -> impl IntoResponse {
//     let use_case = UserUseCase::new();
//     let users = use_case.list_users().await;
//     match users {
//         Ok(users) => Res::with_data(users),
//         Err(err) => {
//             return Res::with_err(&err.to_string());
//         }
//     }
// }

// pub async fn create_user(user: axum::extract::Json<RegisterUserDTO>) -> impl IntoResponse {
//     let use_case = UserUseCase::new();
//     let result = use_case.create_user(user.0).await;
//     match result {
//         Ok(new_user) => Res::with_data(new_user),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// pub async fn update_user(user: axum::extract::Json<User>) -> impl IntoResponse {
//     let use_case = UserUseCase::new();
//     let result = use_case.update_user(user.0).await;
//     match result {
//         Ok(new_user) => Res::with_data(new_user),
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
// pub async fn delete_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
//     let use_case = UserUseCase::new();
//     let result = use_case.delete_user(*id).await;
//     match result {
//         Ok(deleted) => {
//             if deleted {
//                 Res::with_data("User deleted successfully")
//             } else {
//                 Res::with_err("User not found or deletion failed")
//             }
//         }
//         Err(err) => Res::with_err(&err.to_string()),
//     }
// }
