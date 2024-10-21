use application::{
    dto::{ request_command::UserRegisterCommand, response_dto::Res },
    use_case::user_use_case,
};
use axum::{ extract::{Path, State}, response::IntoResponse };
use infrastructure::{ state::AppState, utils::password_util };

// DI：我们把查询用户的trait和命令用户的trait注入到handler中

pub async fn register_handler(
    State(state): State<AppState>,
    user_name: Path<String>,
    password: Path<String>
) -> impl IntoResponse {
    // let user_command = UserRegisterCommand::new(user_name, password);
    // // 创建用例实例
    // let user_use_case = user_use_case::UserUseCase::new(state.user_repository);

    // let result = UserUseCase.execute(user_command).await;
    // match result {
    //     Ok(user) => Res::with_data(user),
    //     Err(err) => Res::with_err(&err.to_string()),
    // }
}
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

// pub async fn get_user(id: axum::extract::Path<i32>) -> impl IntoResponse {
//     let use_case = UserUseCase::new();
//     let user = use_case.get_user_by_id(*id).await;
//     match user {
//         Ok(user) => Res::with_data(user),
//         Err(err) => Res::with_err(&err.to_string()),
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
