use axum::extract::{ Json, State };
use domain::model::reponse::{
    error::{ AppResponseError, AppResult },
    response::{ LoginResponse, MessageResponse, Res, SignUpResponse },
};
use application::dto::command::*;
use garde::Validate;
use tracing::info;

use crate::state::AppState;
/// 注册用户
#[utoipa::path(
    post,
    request_body = SignUpCommand,
    path = "/api/sign_up",
    responses(
        // (status = 200, description = "Success register user", body = [SignUpDto]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(signup_command): Json<SignUpCommand>
) -> AppResult<Res<SignUpResponse>> {
    info!("Register new user with request: {:?}", signup_command);
    // 数据校验
    signup_command.validate()?;
    // let use_case = CustomerUseCase::new(state.into());
    match state.customer_use_case.sign_up_command_handler(signup_command).await {
        Ok(user_id) =>
            Ok(
                Res::with_data(SignUpResponse {
                    user_id,
                })
            ),
        Err(e) => Err(e),
    }
}
/// 激活用户
#[utoipa::path(
    put,
    request_body = ActiveCommand,
    path = "/api/active",
    responses(
        (status = 200, description = "Success active user", body = [MessageResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn active(
    State(state): State<AppState>,
    Json(active_command): Json<ActiveCommand>
) -> AppResult<Res<MessageResponse>> {
    // let use_case = CustomerUseCase::new(state.clone().into());

    match state.customer_use_case.active_command_handler(active_command).await {
        Ok(_) => {
            info!("User successfully activated.");
            Ok(Res::with_msg("User successfully activated."))
        }
        Err(e) => Err(e),
    }
}
/// 用户登录
#[utoipa::path(
    post,
    request_body = LoginCommand,
    path = "/api/v1/user/login",
    responses(
        (status = 200, description = "Success login user", body = [LoginResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(login_command): Json<LoginCommand>
) -> AppResult<Res<LoginResponse>> {
    info!("用户登录请求: {:?}", login_command);
    match state.customer_use_case.login_command_handler(login_command).await {
        Ok(token) => {
            info!("Success login: {token:?}");
            Ok(Res::with_data(LoginResponse::Token(token)))
        }
        Err(e) => Err(e),
    }
}
