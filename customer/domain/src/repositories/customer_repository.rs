use sea_orm::DatabaseTransaction;
use shared::error::AppResult;
use crate::model::{ aggregate::customer::Customer, dp::customer_id::CustomerId };
pub trait CustomerRepository: Send + Sync {
    fn find_all(&self) -> AppResult<Vec<Customer>>;
    fn find_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<Option<Customer>>;
    fn save(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<String>;
    fn find_by_id(&self, tx: &DatabaseTransaction, id: CustomerId) -> AppResult<Option<Customer>>;
    fn send_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<()>;
    fn find_code_by_email(
        &self,
        tx: &DatabaseTransaction,
        email: &str
    ) -> AppResult<Option<String>>;
    fn check_unique_by_username(&self, tx: &DatabaseTransaction, username: &str) -> AppResult<bool>;
    fn check_unique_by_email(&self, tx: &DatabaseTransaction, email: &str) -> AppResult<bool>;
}
