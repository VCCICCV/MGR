use std::sync::Arc;

use sea_orm::DatabaseTransaction;
use tracing::info;
use uuid::Uuid;

use crate::{
    constant::{
        CHECK_EMAIL_MESSAGE,
        CODE_LEN,
        EXPIRE_ACTIVE_CODE_SECS,
        EXPIRE_TWO_FACTOR_CODE_SECS,
    },
    model::{
        aggregate::customer::Customer,
        dto::info::{ LoginKey, SessionKey },
        reponse::{ error::{ AppError, AppResult }, response::{ SignInResponse, TokenResponse } },
    },
    repositories::customer_repository::CustomerRepository,
    utils::{ claim::UserClaims, password, random, redis::RedisUtil, session::Session, token::Token },
};

use super::customer_service::CustomerService;

/// 动态分发
/// 编译器无法知道具体要调用的是 CustomerRepositoryImpl 这个类型所实现的对应方法，因为类型是不确定的
/// 当一个类型实现trait时，编译器会生成一个虚表（vtable）并用一个指针指向这个虚表，其中虚表包含了该类型所实现的所有方法的函数指针
/// Arc包含了这两个指针，一个指向虚表的指针和一个指向数据的指针，当调用一个方法时，编译器会通过trait指向的虚表中的函数指针来确定具体要调用的方法
#[async_trait]
pub struct CustomerServiceImpl {
    customer_repository: Arc<dyn CustomerRepository>,
    redis_util: Arc<dyn RedisUtil>,
    session: Arc<dyn Session>,
    token: Arc<dyn Token>,
}
impl CustomerServiceImpl {
    pub fn new(
        customer_repository: Arc<dyn CustomerRepository>,
        redis_util: Arc<dyn RedisUtil>,
        session: Arc<dyn Session>,
        token: Arc<dyn Token>
    ) -> Self {
        Self {
            customer_repository,
            redis_util,
            session,
            token,
        }
    }
}
// 这里是领域能力
#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn refresh(&self, user_claims: &UserClaims) -> AppResult<TokenResponse> {
        // 检查session是否存在
        let user_id = self.session.check(&user_claims).await?;
        // 查询用户
        if let Some(user) = self.customer_repository.find_by_user_id(&user_id).await? {
            // 生成session并保存到redis
            let session = self.session.set(*user.user_id()).await?;
            // 生成token
            let resp = self.token.generate_token(
                *user.user_id(),
                user.role().clone(),
                session
            ).await?;
            Ok(resp)
        } else {
            Err(AppError::PermissionDeniedError("User not found".to_string()))
        }
    }
    async fn logout(&self, user_id: &Uuid) -> AppResult {
        // 清除session
        let key = SessionKey {
            user_id: *user_id,
        };
        self.redis_util.del(&key.to_string()).await?;
        Ok(())
    }
    async fn sign_in_2fa(&self, customer: Customer) -> AppResult<SignInResponse> {
        // 获取登录key
        let key = LoginKey {
            user_id: *customer.user_id(),
        };
        info!("key: {key:?}");
        // 从redis中获取验证码
        let code = self.redis_util.get(&key.to_string()).await?;
        info!("code: {code:?}");
        // 判断验证码是否正确
        customer.checkout_valid_code(code)?;
        // 根据user_id查询用户
        if let Some(result) = self.customer_repository.find_by_user_id(&customer.user_id()).await? {
            // 生成session
            let session = self.session.set(*result.user_id()).await?;
            // 生成token
            let resp = self.token.generate_token(
                *customer.user_id(),
                customer.role().clone(),
                session
            ).await?;
            // 返回token
            Ok(SignInResponse::Token(resp))
        } else {
            Ok(SignInResponse::Code {
                expire_in: EXPIRE_TWO_FACTOR_CODE_SECS.as_secs(),
                message: CHECK_EMAIL_MESSAGE.to_string(),
            })
        }
    }
    async fn sign_up(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult {
        info!("Customer sign up: {customer:?}");
        // 检查唯一性
        self.customer_repository.check_unique_by_username(tx, &customer.username()).await?;
        self.customer_repository.check_unique_by_email(tx, &customer.email()).await?;
        // 生成激活验证码
        let code = random::generate_random_string(CODE_LEN);
        // 保存激活验证码到redis
        self.redis_util.set(&customer.user_id().to_string(), &code, EXPIRE_ACTIVE_CODE_SECS).await?;
        // 保存用户
        self.customer_repository.insert(tx, customer.clone()).await?;
        Ok(())
    }
    async fn sign_in(&self, customer: Customer) -> AppResult<SignInResponse> {
        // 检查用户是否已激活
        if
            let Some(result) = self.customer_repository.find_by_email_and_status(
                &customer.email(),
                0
            ).await?
        {
            // 验证密码
            password::verify(customer.password().to_string(), result.password().to_string()).await?;
        }
        // 检查是否需要2fa
        if *customer.is2fa() == 0 {
            let key = LoginKey {
                user_id: *customer.user_id(),
            };
            let ttl = self.redis_util.ttl(&key.to_string()).await?;
            if ttl > 0 {
                return Ok(SignInResponse::Code {
                    expire_in: ttl as u64,
                    message: CHECK_EMAIL_MESSAGE.to_string(),
                });
            }
            // 生成验证码并保存到redis
            let login_code = random::generate_random_string(CODE_LEN);
            self.redis_util.set(&key.to_string(), &login_code, EXPIRE_ACTIVE_CODE_SECS).await?;
            // 返回验证
            return Ok(SignInResponse::Code {
                expire_in: EXPIRE_TWO_FACTOR_CODE_SECS.as_secs(),
                message: CHECK_EMAIL_MESSAGE.to_string(),
            });
        }
        // 已经二次验证，直接登录
        // 生成session key和session_id
        let session = self.session.set(*customer.user_id()).await?;
        // 生成token
        let resp = self.token.generate_token(
            *customer.user_id(),
            customer.role().clone(),
            session
        ).await?;
        // 返回token
        Ok(SignInResponse::Token(resp))
    }
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult {
        // 检查是否已激活，1未激活，0已激活
        if let Some(user) = self.customer_repository.find_by_user_id(&customer.user_id()).await? {
            if *user.is_deleted() == 1 {
                return Ok(());
            }
        }
        // 检查验证码是否正确
        let code = self.redis_util.get(&customer.user_id().to_string()).await?;
        info!("code: {code:?}");
        customer.checkout_valid_code(code)?;
        // 更新用户状态
        self.customer_repository.update_status(tx, customer).await?;
        Ok(())
    }
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
