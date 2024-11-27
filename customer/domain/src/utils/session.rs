use axum::async_trait;
use uuid::Uuid;

use crate::model::{entity::claims::UserClaims,  reponse::{error::AppResult, response::TokenResponse}};

#[async_trait]
pub trait Session:Send + Sync  {
    async fn check(&self, claims: &UserClaims) -> AppResult<Uuid> ;
    async fn set(&self, user_id: Uuid) -> AppResult<Uuid>;
}
