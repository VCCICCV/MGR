// use anyhow::{Context, Result};
// use axum::response::IntoResponse;
// use reqwest::StatusCode;
// use uuid::Uuid;
// use tracing::info;
// use crate::{constant::{ACCESS_TOKEN_DECODE_KEY, ACCESS_TOKEN_ENCODE_KEY, EXPIRE_BEARER_TOKEN_SECS, EXPIRE_REFRESH_TOKEN_SECS, REFRESH_TOKEN_DECODE_KEY, REFRESH_TOKEN_ENCODE_KEY}, model::dto::{request::{RefreshTokenRequest, TokenInfoRequest}, response::TokenResponse}, repository::auth_repository, server::state::AppState, utils::{self, claim::Role}};

// use super::claim::UserClaims;

// // 为 anyhow::Error 实现 IntoResponse 以便 Axum 使用
// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> axum::response::Response {
//         let status = if self.to_string().contains("PermissionDenied") {
//             StatusCode::FORBIDDEN
//         } else if self.to_string().contains("Invalid token") {
//             StatusCode::UNAUTHORIZED
//         } else {
//             StatusCode::INTERNAL_SERVER_ERROR
//         };
        
//         (status, self.to_string()).into_response()
//     }
// }

// pub async fn info(
//     state: &AppState,
//     user: UserClaims,
//     req: TokenInfoRequest,
// ) -> Result<UserClaims> {
//     info!("Get token info by user_id: {}", user.uid);
    
//     if user.role != Role::System {
//         anyhow::bail!("PermissionDenied: This user does not have permission to use this resource");
//     }

//     let token_data = UserClaims::decode(&req.token, &ACCESS_TOKEN_DECODE_KEY)
//         .context("Invalid token")?;
        
//     utils::session::check(&state.redis, &token_data.claims)
//         .await
//         .context("Session validation failed")?;
        
//     Ok(token_data.claims)
// }

// pub async fn refresh(
//     state: &AppState, 
//     req: RefreshTokenRequest
// ) -> Result<TokenResponse> {
//     let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)
//         .context("Invalid refresh token")?
//         .claims;
        
//     info!("Refresh token: {user_claims:?}");
    
//     let user_id = utils::session::check(&state.redis, &user_claims)
//         .await
//         .context("Session check failed")?;
        
//     let user = auth_repository::find_by_id(&*state.db, user_id)
//         .await
//         .context("User not found")?;
        
//     let session_id = utils::session::set(&state.redis, user.unwrap().user_id)
//         .await
//         .context("Failed to create new session")?;
        
//     info!("Set new session for user: {}", user.unwrap().user_id);
    
//     generate_tokens(user.unwrap().user_id, user.role, session_id)
//         .context("Failed to generate tokens")
// }

// pub fn generate_tokens(
//     user_id: Uuid,
//     role: Role,
//     session_id: Uuid,
// ) -> Result<TokenResponse> {
//     let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id, role)
//         .encode(&ACCESS_TOKEN_ENCODE_KEY)
//         .context("Failed to encode access token")?;
        
//     let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id, role)
//         .encode(&REFRESH_TOKEN_ENCODE_KEY)
//         .context("Failed to encode refresh token")?;
        
//     Ok(TokenResponse::new(
//         access_token,
//         refresh_token,
//         EXPIRE_BEARER_TOKEN_SECS.as_secs(),
//     ))
// }