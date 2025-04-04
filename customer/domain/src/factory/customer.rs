// 定义抽象工厂trait，用于创建Customer对象
trait CustomerAbstractFactory {
    fn create_customer(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        password: &str,
        avatar: Option<&str>,
        is_deleted: i16,
        verify_code: Option<&str>,
        is2fa: i16,
        role: Role,
        receive_address: Vec<ReceiveAddress>
    ) -> Result<Customer, AppError>;
}
// 具体工厂结构体实现抽象工厂trait
struct DefaultCustomerFactory;

impl CustomerAbstractFactory for DefaultCustomerFactory {
    fn create_customer(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        password: &str,
        avatar: Option<&str>,
        is_deleted: i16,
        verify_code: Option<&str>,
        is2fa: i16,
        role: Role,
        receive_address: Vec<ReceiveAddress>
    ) -> Result<Customer, AppError> {
        let mut builder = CustomerBuilder::new();
        builder
            .user_id(user_id)
            .username(username.to_string())
            .email(email.to_string())
            .password(password.to_string())
            .avatar(avatar.map(|s| s.to_string()))
            .is_deleted(is_deleted)
            .verify_code(verify_code.map(|s| s.to_string()))
            .is2fa(is2fa)
            .role(role)
            .receive_address(receive_address);

        Ok(builder.build())
    }
}
