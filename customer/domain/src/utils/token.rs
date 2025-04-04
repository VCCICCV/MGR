use axum::async_trait;
use uuid::Uuid;
use crate::model::{dp::role::Role, reponse::{ error::AppResult, response::TokenResponse }};
// 有IO操作所以抽象
#[async_trait]
pub trait Token: Send + Sync {
    async fn generate_token(
        &self,
        user_id: Uuid,
        role: Role,
        session_id: Uuid
    ) -> AppResult<TokenResponse>;
}
