use axum::async_trait;
use sea_orm::DatabaseTransaction;
use shared::error::AppResult;
use uuid::Uuid;
use crate::model::{
    aggregate::customer::Customer,
    dto::query::PageParams,
    entity::user::User,
};
// 默认是同步执行，标记后可以异步执行，在编译阶段进行转换使其符合对象安全，以dyn Trait的形式访问
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn active(&self, tx: &DatabaseTransaction,customer:Customer) -> AppResult<()>;
    async fn find_by_user_id(&self,tx: &DatabaseTransaction, user_id: Uuid) -> AppResult<Option<Customer>>;
    async fn find_page(&self, param: PageParams) -> AppResult<Vec<User>>;
    // async fn find_all(&self) -> AppResult<Vec<Customer>>;
    // async fn find_by_email(
    //     &self,
    //     tx: &DatabaseTransaction,
    //     email: &str
    // ) -> AppResult<Option<Customer>>;
    async fn save(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<Uuid>;
    // async fn find_by_id(
    //     &self,
    //     tx: &DatabaseTransaction,
    //     id: CustomerId
    // ) -> AppResult<Option<Customer>>;
    // async fn send_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<()>;
    // async fn find_code_by_email(
    //     &self,
    //     tx: &DatabaseTransaction,
    //     email: &str
    // ) -> AppResult<Option<String>>;
    async fn check_unique_by_username(
        &self,
        tx: &DatabaseTransaction,
        username: &str
    ) -> AppResult<bool>;
    async fn check_unique_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<bool>;
}
