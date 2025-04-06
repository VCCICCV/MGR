
use uuid::Uuid;

use crate::model::reponse::error::AppResult;

use super::claim::UserClaims;

pub trait Session: Send + Sync {
    async fn check(&self, claims: &UserClaims) -> AppResult<Uuid>;
    async fn set(&self, user_id: Uuid) -> AppResult<Uuid>;
}
