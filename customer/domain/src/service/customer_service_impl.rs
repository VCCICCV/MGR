use std::sync::Arc;
use crate::repositories::{customer_repository::CustomerRepository, customer_service::CustomerService};
/// 动态分发
/// 编译器无法知道具体要调用的是 CustomerRepositoryImpl 这个类型所实现的对应方法，因为类型是不确定的
/// 当一个类型实现trait时，编译器会生成一个虚表（vtable）并用一个指针指向这个虚表，其中虚表包含了该类型所实现的所有方法的函数指针
/// Arc包含了这两个指针，一个指向虚表的指针和一个指向数据的指针，当调用一个方法时，编译器会通过trait指向的虚表中的函数指针来确定具体要调用的方法
pub struct CustomerServiceImpl {
    customer_repository: Arc<dyn CustomerRepository>,
}
impl CustomerServiceImpl {
    pub fn new(customer_repository: Arc<dyn CustomerRepository>) -> Self {
        Self {
            customer_repository,
        }
    }
}
// 这里是领域能力
impl CustomerService for CustomerServiceImpl {
}





















// 使用以上where子句更合适
// pub struct CustomerService<T: CustomerRepository> {
//     customer_repository: T,
// }
// impl<T: CustomerRepository> CustomerService<T> {
//     pub fn new(customer_repository: T) -> Self {
//         Self {
//             customer_repository,
//         }
//     }
//     pub async fn find_by_email(&self, email: &str) -> Result<Option<Customer>, InfraError> {
//         self.customer_repository.find_by_email(email.to_string()).await
//     }
// }
