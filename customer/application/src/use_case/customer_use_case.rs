use std::time::Duration;
use domain::{
    model::{
        aggregate::customer::CustomerBuilder,
        dto::{ command::{ ActiveCommand, SignUpCommand }, query::PageParams },
        entity::user::User,
    },
    utils,
};
use infrastructure::client::redis::RedisClientExt;

use sea_orm::TransactionTrait;
use shared::error::{ AppError, AppResult };
use tracing::info;
use uuid::Uuid;
use crate::state::AppState;

// pub async fn sign_up(state: AppState, signup_command: SignUpCommand) -> AppResult<String> {
//     info!("Register a new user request: {signup_command:?}.");
//     // 开启事务
//     let tx = state.db.begin().await?;
//     // 查看是否唯一的username和email
//     state.customer_repository.check_unique_by_username(&tx, &signup_command.username.clone())?;
//     state.customer_repository.check_unique_by_email(&tx, &signup_command.email.clone())?;
//     // 转bo
//     let customer = CustomerBuilder::new()
//         .user_id(Uuid::new_v4().to_string())
//         .email(signup_command.email)
//         .username(signup_command.username)
//         .build();
//     // 保存用户并返回id
//     state.customer_repository.save(&tx, customer.clone())?;
//     // 生成激活码
//     let code = utils::random::generate_random_string(6);
//     // 保存验证码到redis
//     state.redis.set("code", &code, Duration::from_secs(60)).await?;
//     // 提交事务
//     tx.commit().await?;
//     // // 发送消息通知用户激活
//     // let topic = "user_activation_topic"; // 主题名称
//     // // 消息体
//     // let payload = format!("用户 {} 的激活码是：{}，请尽快激活账号。", customer.clone().email(), code);
//     // // 消息对象
//     // let record = FutureRecord::<(), [u8]>::to(topic).payload(payload.as_bytes());
//     // // 发送并返回状态
//     // let delivery_status = state.producer.send(record, Duration::from_secs(0)).await;
//     // match delivery_status {
//     //     Ok(_) => info!("消息成功发送到Kafka主题, 通知用户激活"),
//     //     Err((e, _)) => {
//     //         return Err(AppError::MessageError(e)); // 根据实际错误情况转换为合适的 AppError 并返回错误结果
//     //     }
//     // }
//     Ok(customer.user_id().to_owned())
// }
// pub async fn list(state: AppState, param: PageParams) -> AppResult<Vec<User>> {
//     let users = state.customer_repository.find_page(param).await?;
//     Ok(users)
// }
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
    pub async fn active(&self, active_command: ActiveCommand) -> AppResult<()> {
        info!("激活用户请求: {active_command:?}.");
        // 开启事务
        let tx = self.state.db.begin().await?;
        // 检查是否已激活
        if
            let Some(mut customer) = self.state.customer_repository.find_by_user_id(
                &tx,
                active_command.user_id
            ).await?
        {
            // 获取缓存验证码
            let code = self.state.redis.get(&active_command.user_id.to_string()).await?;
            // 检查验证码正确性
            customer.checkout_valid_code(code.as_deref())?;
            // 删除缓存验证码
            self.state.redis.del(&active_command.user_id.to_string()).await?;
            // 转BO
            let customer = CustomerBuilder::new().user_id(active_command.user_id).is2fa(1).build();
            // 更新激活状态
            self.state.customer_repository.active(&tx, customer).await?;
        } else {
            return Err(AppError::UserNotActiveError("未找到对应的用户记录".to_string()));
        }
        // 提交事务
        tx.commit().await?;
        Ok(())
    }
    pub async fn sign_up(&self, signup_command: SignUpCommand) -> AppResult<Uuid> {
        info!("注册用户请求: {signup_command:?}.");
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
        let code = utils::random::generate_random_string(6);
        // 保存验证码到redis并设置120秒过期
        self.state.redis.set(&customer.email(), &code, Duration::from_secs(120)).await?;
        // 保存用户
        let user_id = self.state.customer_repository.save(&tx, customer).await?;
        // 提交事务
        tx.commit().await?;
        // 使用kafka通知激活发送右键
        Ok(user_id)
    }
    pub async fn list(&self, param: PageParams) -> AppResult<Vec<User>> {
        let users = self.state.customer_repository.find_page(param).await?;
        Ok(users)
    }
}
