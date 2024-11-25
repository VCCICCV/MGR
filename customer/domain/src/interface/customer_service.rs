use axum::async_trait;
use sea_orm::DatabaseTransaction;

use crate::model::{ aggregate::customer::Customer, reponse::error::AppResult };

#[async_trait]
pub trait CustomerService: Send + Sync {
    async fn sign_up(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
    async fn login(&self, customer: &Customer) -> AppResult;
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
}
