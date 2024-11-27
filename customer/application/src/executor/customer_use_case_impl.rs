use axum::async_trait;
use domain::{
    model::{ aggregate::customer::CustomerBuilder, reponse::error::AppResult },
    service::customer_service::CustomerService, utils::password
};
use infrastructure::{client::database::DatabaseClient, utils};
use crate::dto::command::{ ActiveCommand, SignUpCommand };
use tracing::info;
use uuid::Uuid;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use super::customer_use_case::CustomerUseCase;
pub struct CustomerUseCaseImpl {
    db: Arc<DatabaseClient>,
    customer_service: Arc<dyn CustomerService>,
}
impl CustomerUseCaseImpl {
    pub fn new(db: Arc<DatabaseClient>, customer_service: Arc<dyn CustomerService>) -> Self {
        Self {
            db,
            customer_service,
        }
    }
}
#[async_trait]
impl CustomerUseCase for CustomerUseCaseImpl {
     async fn login_command_handler(&self, sign_up_command: SignUpCommand) -> AppResult {
        info!("注册用户请求: {sign_up_command:?}.");
        // // 转bo
        let customer = CustomerBuilder::new()
            .username(sign_up_command.username)
            .email(sign_up_command.email)
            .password(sign_up_command.password)
            .build();
        // 调用领域服务登录验证
        let token = self.customer_service.login(customer).await?;
        // 生成sessionid并保存到redis
        // 生成token
        // 发送消息
        // 返回token
        todo!();
    }
    async fn active_command_handler(&self, active_command: ActiveCommand) -> AppResult {
        info!("激活用户请求: {active_command:?}.");
        // 开启事务
        let tx = self.db.begin().await?;
        // 转bo
        let customer = CustomerBuilder::new()
            .user_id(active_command.user_id)
            .verify_code(Some(active_command.verify_code))
            .is_deleted(0)
            .build();
        // 调用领域服务激活
        self.customer_service.active(&tx, customer).await?;
        // 发送消息
        // 提交事务
        tx.commit().await?;
        Ok(())
        // // 查询用户
        // let customer = self.state.customer_repository
        //     .find_by_user_id(&active_command.user_id).await?
        //     .ok_or(AppError::UserNotActiveError("未找到对应的用户记录".to_string()))?;
        // // 冲redis查询用户验证码
        // // let code = self.state.redis.get(&active_command.user_id.to_string()).await?;
        // // 转bo
        // let customer = CustomerBuilder::new()
        //   .user_id(customer.user_id)
        //   .is_deleted(customer.is_deleted)
        //   .is2fa(customer.is2fa)
        //   .verify_code(Some(active_command.verify_code))
        //   .build();
        // // 通过领域服务激活
        // self.state.customer_service.active(customer, code).await?;
        // // 发送消息

        // // 更新状态
        // let user_id = self.state.customer_repository.save(&tx, customer).await?;
        // // 提交事务
        // tx.commit().await?;
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
    }
    async fn sign_up_command_handler(&self, signup_command: SignUpCommand) -> AppResult<Uuid> {
        info!("注册用户请求: {signup_command:?}.");
        // 开启事务
        let tx = self.db.begin().await?;
        // hash密码
        let hash_password = password::hash(signup_command.password).await?;
        // 生成user_id
        let user_id = Uuid::new_v4();
        //转bo
        let customer = CustomerBuilder::new()
            .user_id(user_id)
            .username(signup_command.username)
            .email(signup_command.email)
            .is_deleted(1)
            .password(hash_password)
            .build();
        self.customer_service.sign_up(&tx, customer).await?;
        // 设置角色
        tx.commit().await?;
        // 使用kafka通知激活发送
        Ok(user_id)
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
