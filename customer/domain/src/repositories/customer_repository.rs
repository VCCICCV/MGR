use shared::error::InfraError;
use crate::model::{aggregate::customer::Customer, vo::customer_id::CustomerId};

// // use crate::model::{ dto::Customer_dto::RegisterCustomerDTO, Customer::Customer };
// // use common::error::InfraError;
// // pub trait CustomerRepository {
// //     async fn find_all(&self) -> Result<Vec<Customer>, InfraError>;
// //     async fn find_by_id(&self, id: i32) -> Result<Option<Customer>, InfraError>;
// //     async fn find_by_email(&self, email: String) -> Result<Option<Customer>, InfraError>;
// //     async fn Customer_exists(&self, email: &str) -> Result<bool, InfraError>;
// //     async fn create(&self, Customer: RegisterCustomerDTO) -> Result<bool, InfraError>;
// //     async fn update(&self, Customer: Customer) -> Result<bool, InfraError>;
// //     async fn delete(&self, id: i32) -> Result<bool, InfraError>;
// //     async fn generate_jwt(&self, Customer: Customer) -> Result<String, InfraError>;
// //     async fn generate_refresh_jwt(&self, Customer: Customer) -> Result<String, InfraError>;
// // }
pub trait CustomerRepository {
    async fn find_all(&self) -> Result<Vec<Customer>, InfraError>;
    async fn find_by_email(&self, email: String) -> Result<Option<Customer>, InfraError>;
    async fn save(&self, customer: Customer) -> Result<(), InfraError>;
    async fn find_by_id(&self, id: CustomerId) -> Result<Option<Customer>, InfraError>;
    async fn send_email(&self, email:String) -> Result<(), InfraError>;
    // async fn save(&self, Customer: Customer) -> Result<(), InfraError>;
    // async fn update(&self, Customer: Customer) -> Result<(), InfraError>;
    // async fn delete(&self, id: i32) -> Result<(), InfraError>;
}
