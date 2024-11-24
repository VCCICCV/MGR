// use axum::async_trait;
// use crate::constant::*;
// pub struct TokenRepositoryImpl {}
// impl TokenRepositoryImpl {
//     pub fn new() -> Self {}
// }
// #[async_trait]
// impl TokenRepository for TokenRepositoryImpl {
//     async fn generate_token(
//         &self,
//         user_id: Uuid,
//         role: Role,
//         session_id: Uuid
//     ) -> AppResult<TokenResponse> {
//         // 
//         let access_token = UserClaims::new(
//             EXPIRE_BEARER_TOKEN_SECS,
//             user_id,
//             session_id,
//             role
//         ).encode(&ACCESS_TOKEN_ENCODE_KEY)?;
//         let refresh_token = UserClaims::new(
//             EXPIRE_REFRESH_TOKEN_SECS,
//             user_id,
//             session_id,
//             role
//         ).encode(&REFRESH_TOKEN_ENCODE_KEY)?;
//         Ok(TokenResponse::new(access_token, refresh_token, EXPIRE_BEARER_TOKEN_SECS.as_secs()))
//     }
// }