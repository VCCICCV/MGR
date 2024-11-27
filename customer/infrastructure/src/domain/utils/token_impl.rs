use axum::async_trait;
use domain::{
    model::{
        dp::role::Role,
        entity::claims::UserClaims,
        reponse::{ error::AppResult, response::TokenResponse },
    },
    repositories::customer_repository::CustomerRepository,
    utils::{ session::Session, token::Token },
};
use tracing::info;
use uuid::Uuid;

use crate::constant::{
    ACCESS_TOKEN_ENCODE_KEY,
    EXPIRE_BEARER_TOKEN_SECS,
    EXPIRE_REFRESH_TOKEN_SECS,
    REFRESH_TOKEN_DECODE_KEY,
    REFRESH_TOKEN_ENCODE_KEY,
};

use std::sync::Arc;
pub struct TokenImpl {
    session: Arc<dyn Session>,
    customer_repository: Arc<dyn CustomerRepository>,
}
impl TokenImpl {
    pub fn new(
        session: Arc<dyn Session>,
        customer_repository: Arc<dyn CustomerRepository>
    ) -> Self {
        TokenImpl { session, customer_repository }
    }
}
#[async_trait]
impl Token for TokenImpl {
    async fn generate_token(
        &self,
        user_id: Uuid,
        role: Role,
        session_id: Uuid
    ) -> AppResult<TokenResponse> {
        let access_token = UserClaims::new(
            EXPIRE_BEARER_TOKEN_SECS,
            user_id,
            session_id,
            role.clone()
        ).encode(&ACCESS_TOKEN_ENCODE_KEY)?;
        let refresh_token = UserClaims::new(
            EXPIRE_REFRESH_TOKEN_SECS,
            user_id,
            session_id,
            role
        ).encode(&REFRESH_TOKEN_ENCODE_KEY)?;
        Ok(TokenResponse::new(access_token, refresh_token, EXPIRE_BEARER_TOKEN_SECS.as_secs()))
    }
    async fn refresh(&self, token: &str) -> AppResult<TokenResponse> {
        //  解码token
        // let user_claims = UserClaims::decode(token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
        // info!("Refresh token: {user_claims:?}");
        // //  检查session是否存在
        // let user_id = self.session.check(&user_claims).await?;
        // // 查询用户
        // let user =self.customer_repository.find_by_user_id(user_id).await?;
        // // 生成session并存入redis
        // let session_id = service::session::set(&state.redis, user.id).await?;
        // info!("Set new session for user: {}", user.id);
        // // 生成token
        // let resp = generate_tokens(user.id, user.role, session_id)?;
        // info!("Refresh token success: {user_claims:?}");
        // Ok(resp)
        todo!()
    }
}
