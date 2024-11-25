// use domain::model::{
//     dp::{ claims::UserClaims, role::Role },
//     reponse::{ error::AppResult, response::TokenResponse },
// };
// use uuid::Uuid;

// use crate::constant::{
//     ACCESS_TOKEN_ENCODE_KEY,
//     EXPIRE_BEARER_TOKEN_SECS,
//     EXPIRE_REFRESH_TOKEN_SECS,
//     REFRESH_TOKEN_ENCODE_KEY,
// };

// // 生成token
// pub fn generate_tokens(user_id: Uuid, role: Role, session_id: Uuid) -> AppResult<TokenResponse> {
//     let access_token = UserClaims::new(
//         EXPIRE_BEARER_TOKEN_SECS,
//         user_id,
//         session_id,
//         role.clone()
//     ).encode(&ACCESS_TOKEN_ENCODE_KEY)?;
//     let refresh_token = UserClaims::new(
//         EXPIRE_REFRESH_TOKEN_SECS,
//         user_id,
//         session_id,
//         role
//     ).encode(&REFRESH_TOKEN_ENCODE_KEY)?;
//     Ok(TokenResponse::new(access_token, refresh_token, EXPIRE_BEARER_TOKEN_SECS.as_secs()))
// }
