use application::dto::{ command::RefreshTokenCommand, query::TokenInfoQuery };
use axum::{ extract::State, Json };
use domain::{model::reponse::{ error::{ AppResponseError, AppResult }, response::{ Res, TokenResponse } }, utils::claim::UserClaims};
use garde::Validate;
use tracing::{ info, warn };
use crate::state::AppState;

/// 刷新token
#[utoipa::path(
    post,
    path = "/api/token/refresh",
    responses(
        (
            status = 200,
            description = "Success get new access token and refresh token",
            body = [TokenResponse],
        ),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(refresh_token_command): Json<RefreshTokenCommand>
) -> AppResult<Res<TokenResponse>> {
    info!("Refresh token with request: {refresh_token_command:?}.");
    match state.customer_use_case.refresh_command_handler(refresh_token_command).await {
        Ok(resp) => {
            info!("Success refresh token user response: {resp:?}.");
            Ok(Res::with_data(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully refresh token error: {e:?}.");
            Err(e)
        }
    }
}

/// 获取token详情
#[utoipa::path(
    post,
    path = "/api/token/info",
    request_body = TokenInfoQuery,
    responses(
        (status = 200, description = "Success get token information", body = [UserClaims]),
        (status = 400, description = "Invalid token", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
    // security("jwt" = [])
)]
pub async fn info(
    State(state): State<AppState>,
    user_claims: UserClaims,
    Json(token_info_query): Json<TokenInfoQuery>
) -> AppResult<Res<UserClaims>> {
    // 校验请求
    token_info_query.validate()?;
    info!("Get token information by user_id: {}.", user_claims.uid);
    match state.customer_use_case.info_query_handler(user_claims, token_info_query).await {
        Ok(resp) => {
            info!("Success get token information response: {resp:?}.");
            Ok(Res::with_data(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully get token information error: {e:?}.");
            Err(e)
        }
    }
}
