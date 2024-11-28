use std::time::Duration;
use chrono::Utc;
use once_cell::sync::Lazy;
use jsonwebtoken::{ Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation };
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;
use crate::model::{ dp::role::Role, reponse::error::{ AppError, AppResult } };
// 常量
pub static DECODE_HEADER: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));
// token claims
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, ToSchema)]
pub struct UserClaims {
    // 签发时间
    pub iat: i64,
    // 过期时间
    pub exp: i64,
    // user id
    pub uid: Uuid,
    // session id
    pub sid: Uuid,
    // role user
    pub rol: Role,
}
impl UserClaims {
    // 创建claims
    pub fn new(duration: Duration, user_id: Uuid, session_id: Uuid, role: Role) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: user_id,
            sid: session_id,
            rol: role,
        }
    }
    //加密
    pub fn decode(
        token: &str,
        key: &DecodingKey
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<UserClaims>(token, key, &DECODE_HEADER)
    }
    // 解密
    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key)
    }
}

// 从header提取claims方法
pub trait UserClaimsRequest {
    fn get_user_id(&self) -> AppResult<Uuid>;
    fn get_user_claims(&self) -> AppResult<UserClaims>;
}
// 实现提取claims方法
impl UserClaimsRequest for axum::extract::Request {
    fn get_user_id(&self) -> AppResult<Uuid> {
        self.extensions()
            .get::<UserClaims>()
            .map(|u| u.uid)
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }

    fn get_user_claims(&self) -> AppResult<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }
}
