use axum::async_trait;
use uuid::Uuid;

use crate::model::reponse::{error::AppResult, response::TokenResponse};

#[async_trait]
pub trait TokenRepository {
    async fn generate_token(&self, user_id: Uuid) -> String;
    async fn refresh(&self, token: &str) -> AppResult<TokenResponse>;
}
