use std::{ sync::LazyLock, time::Duration };

use axum_extra::{ TypedHeader, headers::{ Authorization, authorization::Bearer } };

use axum::{http::Response, response::IntoResponse, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Utc;
use reqwest::StatusCode;
use super::session;
use jsonwebtoken::Header;
use jsonwebtoken::{ Algorithm, DecodingKey, EncodingKey, TokenData, Validation };
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use anyhow::{ Context, Result };

use crate::{ constant::ACCESS_TOKEN_DECODE_KEY, server::state::AppState };

pub static DECODE_HEADER: LazyLock<Validation> = LazyLock::new(||
    Validation::new(Algorithm::RS256)
);
pub static ENCODE_HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Role {
    Admin,
    User,
    System,
    // 添加其他角色...
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserClaims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // user id
    pub uid: Uuid,
    // session id
    pub sid: Uuid,
    // role user
    pub role: Role,
}

impl UserClaims {
      pub fn new(duration: Duration, user_id: Uuid, session_id: Uuid, role: Role) -> Self {
    let now = Utc::now().timestamp();
    Self {
      iat: now,
      exp: now + (duration.as_secs() as i64),
      uid: user_id,
      sid: session_id,
      role: role,
    }
  }
    pub fn decode(token: &str, key: &DecodingKey) -> Result<TokenData<Self>> {
        jsonwebtoken
            ::decode::<UserClaims>(token, key, &DECODE_HEADER)
            .context("Failed to decode JWT token")
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key).context("Failed to encode JWT token")
    }
}
impl FromRequestParts<AppState> for UserClaims {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Missing auth header".into()))?;

        let claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?
            .claims;

        session::check(&state.redis, &claims)
            .await
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Session check failed: {}", e)))?;

        Ok(claims)
    }
}

pub trait UserClaimsRequest {
    fn get_user_id(&self) -> Result<Uuid>;
    fn get_user_claims(&self) -> Result<UserClaims>;
}

impl UserClaimsRequest for axum::extract::Request {
    fn get_user_id(&self) -> Result<Uuid> {
        self.extensions()
            .get::<UserClaims>()
            .map(|u| u.uid)
            .ok_or_else(|| anyhow::anyhow!("User must be logged in"))
    }

    fn get_user_claims(&self) -> Result<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("User must be logged in"))
    }
}
