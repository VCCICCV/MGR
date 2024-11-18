use std::time::Duration;
use domain::{ model::aggregate::customer::CustomerBuilder, utils };
use infrastructure::client::redis::RedisClientExt;
use rdkafka::producer::FutureRecord;
use sea_orm::TransactionTrait;
use shared::error::{AppError, AppResult};
use tracing::info;
use uuid::Uuid;
use crate::{ dto::command::SignUpCommand, state::AppState };

pub async fn sign_up(state: AppState, signup_command: SignUpCommand) -> AppResult<Uuid> {
    info!("Register a new user request: {signup_command:?}.");
    // 开启事务
    let tx = state.db.begin().await?;
    // 查看是否唯一的username和email
    state.customer_repository.check_unique_by_username(&tx, &signup_command.username.clone())?;
    state.customer_repository.check_unique_by_email(&tx, &signup_command.email.clone())?;
    // 转bo
    let customer = CustomerBuilder::new()
        .user_id(Uuid::new_v4().to_string())
        .email(signup_command.email)
        .username(signup_command.username)
        .build();
    // 保存用户并返回id
    let user_id = state.customer_repository.save(&tx, customer)?;
    // 生成激活码
    let code = utils::random::generate_random_string(6);
    // 保存验证码到redis
    state.redis.set("code", &code, Duration::from_secs(60)).await?;
    // 提交事务
    tx.commit().await?;
    // 发送消息通知用户激活
    let topic = "user_activation_topic"; // 主题名称
    // 消息体
    let payload = format!("用户 {} 的激活码是：{}，请尽快激活账号。", customer.email(), code);
    // 
    let record = FutureRecord::to(topic).payload(payload.as_bytes());
    // 发送状态
    let delivery_status = state.producer.send(record, Duration::from_secs(0)).await;
    match delivery_status {
        Ok(_) => info!("消息成功发送到Kafka主题，通知用户激活"),
        Err((e,_)) => {
            return Err(AppError::MessageError(e)); // 根据实际错误情况转换为合适的 AppError 并返回错误结果
        }
    }
    Ok(user_id)
}
