use axum::extract::{ Json, State };
use domain::{
    model::reponse::{
        error::{ AppResponseError, AppResult },
        response::{ MessageResponse, Res, SignInResponse, SignUpResponse },
    },
    utils::claim::UserClaims,
};
use application::dto::command::*;
use garde::Validate;
use tracing::{ info, instrument };

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
    request_body = SignInCommand,
    path = "/api/sign_in",
    responses(
        (status = 200, description = "Success login user", body = [SignInResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
#[instrument(
    skip(state, sign_in_command),
    fields(sign_in_command = tracing::field::Empty, state = tracing::field::Empty)
)]
pub async fn sign_in(
    State(state): State<AppState>,
    Json(sign_in_command): Json<SignInCommand>
) -> AppResult<Res<SignInResponse>> {
    info!("Sign in copmmand: {:?}", sign_in_command);
    match state.customer_use_case.sign_in_command_handler(sign_in_command).await {
        Ok(token_resp) => {
            info!("Success login: {token_resp:?}");
            Ok(Res::with_data(token_resp))
        }
        Err(e) => Err(e),
    }
}
pub async fn sign_in_2fa(
    State(state): State<AppState>,
    Json(sign_in_2fa_command): Json<SignIn2FaCommand>
) -> AppResult<Res<SignInResponse>> {
    info!("Sign in 2fa command: {:?}", sign_in_2fa_command);
    match state.customer_use_case.sign_in_2fa_command_handler(sign_in_2fa_command).await {
        Ok(resp) => {
            info!("Success login 2fa.");
            Ok(Res::with_data(resp))
        }
        Err(e) => Err(e),
    }
}
/// 用户登出，由于实现了
pub async fn logout(
    State(state): State<AppState>,
    claims: UserClaims
) -> AppResult<Res<MessageResponse>> {
    info!("Logout user: {:?}", claims);
    match state.customer_use_case.logout_command_handler(claims.uid).await {
        Ok(_) => {
            info!("Success logout.");
            Ok(Res::with_msg("Success logout."))
        }
        Err(e) => Err(e),
    }
}
