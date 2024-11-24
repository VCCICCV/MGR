use axum::async_trait;

use crate::model::{aggregate::customer::Customer, reponse::error::AppResult};

#[async_trait]
pub trait CustomerService:Send+Sync{
    async fn login(&self, customer: &Customer) -> AppResult<()>;
    async fn active(&self, customer: &Customer) -> AppResult<()>;
}