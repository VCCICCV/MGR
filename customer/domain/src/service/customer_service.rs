use axum::async_trait;
use sea_orm::DatabaseTransaction;
use uuid::Uuid;

use crate::{model::{
    aggregate::customer::Customer,reponse::{ error::AppResult, response::{SignInResponse, TokenResponse} }
}, utils::claim::UserClaims};

#[async_trait]
pub trait CustomerService: Send + Sync {
    async fn refresh(&self, user_claims: &UserClaims) -> AppResult<TokenResponse>;
    async fn logout(&self, user_id: &Uuid) -> AppResult;
    async fn sign_in_2fa(&self, customer: Customer) -> AppResult<SignInResponse>;
    async fn sign_up(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
    async fn sign_in(&self, customer: Customer) -> AppResult<SignInResponse>;
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
}
