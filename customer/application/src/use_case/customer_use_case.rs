use std::time::Duration;
use domain::{
    constant::CODE_LEN,
    model::{
        aggregate::customer::{ Customer, CustomerBuilder },
        dto::{ command::{ ActiveCommand, LoginCommand, SignUpCommand }, query::PageParams },
        reponse::{ error::{ AppError, AppResult }, response::LoginResponse },
    },
    service,
    utils::{ self, password },
};
use infrastructure::{ client::redis::RedisClientExt, utils::token::generate_tokens };
use sea_orm::TransactionTrait;
use tracing::info;
use uuid::Uuid;
use crate::state::AppState;

use std::sync::Arc;
pub struct CustomerUseCase {
    state: Arc<AppState>,
}
impl CustomerUseCase {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
        }
    }
    //     // pub async fn login(&self, login_command: LoginCommand) -> AppResult<()> {
    //         // info!("登录用户请求: {login_command:?}.");
    //         // // 事务控制
    //         // // 调用领域服务
    //         // let token = self.state.customer_service.login(login_command).await?;
    //         // 发送消息

    //         // // 判断用户是否被删除
    //         // if
    //         //     let Some(customer) = self.state.customer_repository.find_by_username_and_status(
    //         //         &login_command.email,
    //         //         0
    //         //     ).await?
    //         // {
    //         //     // 判断密码是否正确
    //         //     password::verify(login_command.password, customer.password().to_string()).await?;
    //         //     if *customer.is2fa() == 0 {
    //         //         // 检查ttl是否过期
    //         //         let key = Loginkey { user_id };
    //         //         let ttl = self.state.redis.ttl(&key).await?;
    //         //         if ttl > 0 {
    //         //             return Ok(LoginResponse::Code {
    //         //                 expire_in: ttl as u64,
    //         //                 message: CHECK_EMAIL_MESSAGE.to_string(),
    //         //             });
    //         //         }
    //         //         let login_code = utils::random_code(CODE_LEN)?;
    //         //         // 保存消息到kafka
    //         //         // todo
    //         //         // 保存登陆验证码到redis
    //         //         &self.state.redis.set(&key, &login_code).await?;
    //         //     }
    //         // }
    //         // // 生成sessionid并保存到redis
    //         // let session_id = service::session::set(user_id).await?;
    //         // // 返回token
    //         // let token = generate_tokens(user.user_id(), user.role(), session_id)?;
    //         // Ok(LoginResponse::Token(resp))
    //     // }
    pub async fn active(&self, active_command: ActiveCommand) -> AppResult<()> {
        info!("激活用户请求: {active_command:?}.");
        // 开启事务
        let tx = self.state.db.begin().await?;
<<<<<<< HEAD
        // 检查是否已激活
        if
            let Some(mut customer) = self.state.customer_repository.find_by_user_id(
                active_command.user_id
            ).await?
        {
            // 更新BO
            customer = CustomerBuilder::new()
                .user_id(active_command.user_id)
                .is_deleted(0)
                .verify_code(Some(active_command.verify_code))
                .build();
            // 获取缓存验证码
            let code = self.state.redis.get(&active_command.user_id.to_string()).await?;
            // 传入缓存验证码检查验证码正确性
            customer.checkout_valid_code(code.as_deref())?;
            // 删除缓存验证码
            self.state.redis.del(&active_command.user_id.to_string()).await?;

            // 更新激活状态
            self.state.customer_repository.active(&tx, customer).await?;
        } else {
            return Err(AppError::UserNotActiveError("未找到对应的用户记录".to_string()));
        }
=======
        // 查询用户
        let customer = self.state.customer_repository
            .find_by_user_id(&active_command.user_id).await?
            .ok_or(AppError::UserNotActiveError("未找到对应的用户记录".to_string()))?;
        // 冲redis查询用户验证码
        let code = self.state.redis.get(&active_command.user_id.to_string()).await?;
        // 转bo
        let customer = CustomerBuilder::new()
          .user_id(customer.user_id)
          .is_deleted(customer.is_deleted)
          .is2fa(customer.is2fa)
          .verify_code(Some(active_command.verify_code))
          .build();
        // 通过领域服务激活
        self.state.customer_service.active(customer, code).await?;
        // 发送消息
        
        // 更新状态
        let user_id = self.state.customer_repository.save(&tx, customer).await?;
>>>>>>> 060613f1c423a3dd9be37fbc8d6576f837e4dca7
        // 提交事务
        tx.commit().await?;
        // 使用kafka通知激活发送
        // // 开启事务
        // let tx = self.state.db.begin().await?;
        // // 激活并发送消息
        // // 检查是否已激活
        // if
        //     let Some(mut customer) = self.state.customer_repository.find_by_user_id(
        //         active_command.user_id
        //     ).await?
        // {
        //     // 更新BO
        //     customer = CustomerBuilder::new()
        //         .user_id(active_command.user_id)
        //         .is_deleted(0)
        //         .verify_code(Some(active_command.verify_code))
        //         .build();
        //     // 获取缓存验证码
        //     let code = self.state.redis.get(&active_command.user_id.to_string()).await?;
        //     // 传入缓存验证码检查验证码正确性
        //     customer.checkout_valid_code(code.as_deref())?;
        //     // 删除缓存验证码
        //     self.state.redis.del(&active_command.user_id.to_string()).await?;
        //     // 更新激活状态
        //     self.state.customer_repository.active(&tx, customer).await?;
        // } else {
        //     return Err(AppError::UserNotActiveError("未找到对应的用户记录".to_string()));
        // }
        // // 提交事务
        // tx.commit().await?;
        Ok(())
    }
    pub async fn sign_up(&self, signup_command: SignUpCommand) -> AppResult<Uuid> {
        info!("注册用户请求: {signup_command:?}.");
        // 查询用户
        // 开启事务
        let tx = self.state.db.begin().await?;
        self.state.customer_repository.check_unique_by_username(
            &tx,
            &signup_command.username.clone()
        ).await?;
        self.state.customer_repository.check_unique_by_email(
            &tx,
            &signup_command.email.clone()
        ).await?;
        // 转bo
        let customer = CustomerBuilder::new()
            .username(signup_command.username)
            .email(signup_command.email)
            .password(signup_command.password)
            .build();
        // 生成激活码
        let code = utils::random::generate_random_string(CODE_LEN);
        // 保存用户
        let user_id = self.state.customer_repository.save(&tx, customer).await?;
        // 保存验证码到redis并设置120秒过期
        self.state.redis.set(&user_id.to_string(), &code, Duration::from_secs(120)).await?;
        // 提交事务
        tx.commit().await?;
        // 使用kafka通知激活发送
        Ok(user_id)
    }
    //     // pub async fn list(&self, param: PageParams) -> AppResult<Vec<User>> {
    //     //     let users = self.state.customer_repository.find_page(param).await?;
    //     //     Ok(users)
    //     // }
}
