// use axum::async_trait;
// use uuid::Uuid;

// use crate::model::{dp::claims::UserClaims, reponse::{error::AppResult, response::TokenResponse}};

// #[async_trait]
// pub trait SessionRepository {
//     async fn check(&self, claims: &UserClaims) -> String;
//     async fn set(&self, user_id: Uuid) -> AppResult<TokenResponse>;
// }
