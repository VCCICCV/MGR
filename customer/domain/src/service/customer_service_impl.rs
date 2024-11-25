use std::sync::Arc;
use axum::async_trait;
use sea_orm::DatabaseTransaction;
use tracing::info;
use crate::{
    constant::{ CODE_LEN, EXPIRE_ACTIVE_CODE_SECS },
    interface::customer_service::CustomerService,
    model::{ aggregate::customer::Customer, reponse::error::AppResult },
    repositories::{ customer_repository::CustomerRepository, redis_repository::RedisRepository },
    utils::{ self, random },
};
/// 动态分发
/// 编译器无法知道具体要调用的是 CustomerRepositoryImpl 这个类型所实现的对应方法，因为类型是不确定的
/// 当一个类型实现trait时，编译器会生成一个虚表（vtable）并用一个指针指向这个虚表，其中虚表包含了该类型所实现的所有方法的函数指针
/// Arc包含了这两个指针，一个指向虚表的指针和一个指向数据的指针，当调用一个方法时，编译器会通过trait指向的虚表中的函数指针来确定具体要调用的方法
pub struct CustomerServiceImpl {
    customer_repository: Arc<dyn CustomerRepository>,
    redis_repository: Arc<dyn RedisRepository>,
}
impl CustomerServiceImpl {
    pub fn new(
        customer_repository: Arc<dyn CustomerRepository>,
        redis_repository: Arc<dyn RedisRepository>
    ) -> Self {
        Self {
            customer_repository,
            redis_repository,
        }
    }
}
// 这里是领域能力
#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn sign_up(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult {
        info!("Customer sign up: {customer:?}");
        // 检查唯一性
        self.customer_repository.check_unique_by_username(tx, &customer.username()).await?;
        self.customer_repository.check_unique_by_email(tx, &customer.email()).await?;
        // 生成激活验证码
        let code = random::generate_random_string(CODE_LEN);
        // 保存激活验证码到redis
        self.redis_repository.set(&customer.user_id().to_string(), &code, EXPIRE_ACTIVE_CODE_SECS).await?;
        // 保存用户
        self.customer_repository.insert(tx, customer.clone()).await?;
        Ok(())
    }
    async fn login(&self, customer: &Customer) -> AppResult {
        todo!()
    }
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult {
        // 检查是否已激活，1未激活，0已激活
        if let Some(user) = self.customer_repository.find_by_user_id(tx, &customer.user_id()).await?{
            if *user.is_deleted() == 1 {
                return Ok(());
            }
        }
        // 检查验证码是否正确
        let code = self.redis_repository.get(&customer.user_id().to_string()).await?;
        info!("code: {code:?}");
        customer.checkout_valid_code(code)?;
        // if code.is_none() || *code != customer.active_code() {
        //     return Err(utils::create_error("验证码错误"));
        // }
        // 更新用户状态
        info!("********************Customer active: {customer:?}");
        self.customer_repository.update_status(tx, customer).await?;
        Ok(())
    }
    // async fn login(&self, customer: &Customer) -> AppResult {

    // }
    // async fn active(&self, customer: &Customer, code: &str) -> AppResult {
    //     if customer.is_deleted() == 0 {
    //         Ok(())
    //     } else {
    //         // 校验验证码
    //         customer.checkout_valid_code(code).await?;
    //         // 激活
    //         customer.is2fa(1)
    //     }
    // }
    // async fn login(&self, customer: Customer) -> AppResult<LoginResponse> {
    // // 判断用户是否激活，修正if let语句的格式，去掉多余的换行和括号，使其符合语法规范
    // if
    //     let Some(result) = self.customer_repository.find_by_username_and_status(
    //         &customer.user_name
    //     ).await?
    // {
    //     // 判断密码是否正确
    //     password::verify(customer.password(), result.password().to_string()).await?;
    //     // 2fa之后才能登录
    //     if *result.is_2fa() == 1 {
    //         // 这里原代码中 user_id 未定义，假设从 result 中获取用户ID，你可根据实际情况调整
    //         let user_id = result.user_id();
    //         let key = LoginKey { user_id };
    //         // 检查ttl是否过期
    //         let ttl = self.redis_repository.get_ttl(&key).await?;
    //         if ttl > 0 {
    //             return Ok(LoginResponse::Code {
    //                 expire_in: ttl as u64,
    //                 message: CHECK_EMAIL_MESSAGE.to_string(),
    //             });
    //         }
    //         // 生成验证码
    //         let login_code = utils::generate_code(CODE_LEN);
    //         // 保存验证码到redis
    //         self.redis_repository.set(&key, &login_code).await?;
    //     } else {
    //         // 返回验证相关响应
    //         return Ok(LoginResponse::Code {
    //             expire_in: EXPIRE_TWO_FACTOR_CODE_SECS.as_secs(),
    //             message: CHECK_EMAIL_MESSAGE.to_string(),
    //         });
    //     }
    // } else {
    //     // 返回错误信息，这里调整了错误创建的格式，使其更规范一些（原代码创建错误的格式不太对）
    //     return Err(
    //         AppError::new(
    //             ErrorCode::InvalidCredentials,
    //             INVALID_CREDENTIALS_MESSAGE.to_string()
    //         )
    //     );
    // }

    // // 生成sessionid并保存到redis
    // let session_id = self.session_repository.set(customer.user_id()).await?;
    // // 返回token
    // let resp = self.token_repository.generate_token(customer.user_id()).await?;
    // Ok(LoginResponse::Token(resp))
    // }
}

// 泛型注入方式
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
//
