use axum::async_trait;
use sea_orm::DatabaseTransaction;
use uuid::Uuid;

use crate::model::{aggregate::customer::Customer, reponse::error::AppResult};
//在编译阶段进行转换使其符合对象安全，以dyn Trait的形式访问
// 命令以事务实现，查询以非事务实现
// 查询、删除传入等简单类型使用&，更新添加等复杂类直接传类型获得所有权，避免不必要的复制
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_user_id(
        &self,
        user_id: &Uuid
    ) -> AppResult<Option<Customer>>;
    async fn update_status(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
    async fn update(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult;
    async fn delete(&self, tx: &DatabaseTransaction, user_id: &Uuid) -> AppResult;
    async fn insert(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<Uuid>;
    async fn find_by_email_and_status(
        &self,
        email: &str,
        is_deleted: i16
    ) -> AppResult<Option<Customer>>;
    async fn check_unique_by_username(
        &self,
        tx: &DatabaseTransaction,
        username: &str
    ) -> AppResult<bool>;
    async fn check_unique_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<bool>;
}
