use axum::async_trait;
use sea_orm::DatabaseTransaction;

use uuid::Uuid;
use crate::{
    model::{
        aggregate::customer::Customer,
        dp::role::Role,
        dto::query::PageParams,
        reponse::{ error::AppResult, response::TokenResponse },
    },
    query_model::user::User,
};
//在编译阶段进行转换使其符合对象安全，以dyn Trait的形式访问
// 命令以事务实现，查询以非事务实现
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_username_and_status(
        &self,
        email: &str,
        is_delete: i16
    ) -> AppResult<Option<Customer>>;
    async fn generate_token(
        &self,
        user_id: Uuid,
        role: Role,
        session_id: Uuid
    ) -> AppResult<TokenResponse>;
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<()>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> AppResult<Option<Customer>>;

    async fn save(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<Uuid>;
    async fn check_unique_by_username(
        &self,
        tx: &DatabaseTransaction,
        username: &str
    ) -> AppResult<bool>;
    async fn check_unique_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<bool>;
    // 查询仓储，不经过业务
    async fn find_page(&self, param: PageParams) -> AppResult<Vec<User>>;
}
