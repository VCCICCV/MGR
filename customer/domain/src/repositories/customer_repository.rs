use axum::async_trait;
use sea_orm::DatabaseTransaction;

use uuid::Uuid;
use crate::model::{
    aggregate::customer::Customer, dp::role::Role, dto::query::PageParams, entity::user::User, vo::{error::AppResult, response::TokenResponse}
};
// 默认是同步执行，标记后可以异步执行，在编译阶段进行转换使其符合对象安全，以dyn Trait的形式访问
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn generate_token(&self, user_id: Uuid, role: Role, session_id: Uuid)->AppResult<TokenResponse>;
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<()>;
    async fn find_by_user_id(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid
    ) -> AppResult<Option<Customer>>;
    async fn find_page(&self, param: PageParams) -> AppResult<Vec<User>>;
    async fn save(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<Uuid>;
    async fn check_unique_by_username(
        &self,
        tx: &DatabaseTransaction,
        username: &str
    ) -> AppResult<bool>;
    async fn check_unique_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<bool>;
}
