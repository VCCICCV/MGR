// use crate::{model::dto::command::TokenInfoCommand, service::session};

// pub async fn info(
//     state: &AppState,
//     user: UserClaims,
//     req: TokenInfoCommand
// ) -> AppResult<UserClaims> {
//     info!("Get token info by user_id: {}", user.uid);
//     if user.rol != Role::System {
//         return Err(
//             AppError::PermissionDeniedError(
//                 "This user does not have permission to use this resource.".to_string()
//             )
//         );
//     }
//     let token_data = UserClaims::decode(&req.token, &ACCESS_TOKEN_DECODE_KEY)?;
//     // Check token
//     session::check(&state.redis, &token_data.claims).await?;
//     Ok(token_data.claims)
// }
// // 刷新token
// pub async fn refresh(state: &AppState, req: RefreshTokenCommand) -> AppResult<TokenResponse> {
//     let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
//     info!("Refresh token success: {user_claims:?}");
//     let user_id = session::check(&state.redis, &user_claims).await?;
//     // 查询用户
//     if let Some(user) = state.customer_repository.find_by_user_id(user_id).await? {
//         let session_id = session::set(&state.redis, *user.user_id()).await?;
//         info!("Set new session for user: {}", *user.user_id());
//         let resp = generate_tokens(*user.user_id(), user.role().clone(), session_id)?;
//         Ok(resp)
//     } else {
//         Err(AppError::UserNotFound("User not found".to_string()))
//     }
// }
// use std::sync::Arc;
// use std::time::Duration;
// use serde::de::value::UsizeDeserializer;

// use crate::{
//     model::{ dp::claims::UserClaims, dto::command::TokenInfoCommand, vo::error::AppResult },
//     repositories::redis_repository::RedisRepository,
// };
// pub struct TokenService {
//     redis_repository: Arc<dyn RedisRepository>,
// }
// impl TokenService {
//     pub fn new(redis_repository: Arc<dyn RedisRepository>) -> Self {
//         TokenService { redis_repository }
//     }
// }
// impl TokenService {
//     pub async fn info(user: UserClaims, req: TokenInfoCommand) -> AppResult<UserClaims> {
//         info!("Get token info by user_id: {}", user.uid);
//         // 解码
//         let token_data = UserClaims::decode(req.token,)
//     }
//     // pub async fn get(&self, key: &str) -> AppResult<Option<String>> {
//     //     self.redis_repository.get(key).await
//     // }
//     // pub async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult<()> {
//     //     self.redis_repository.set(key, value, expire).await
//     // }
// }
