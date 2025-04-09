use std::time::Duration;

use crate::{
    constant::{ CHECK_EMAIL_MESSAGE, CODE_LEN, EXPIRE_TWO_FACTOR_CODE_SECS },
    model::{
        dto::{ request::{LoginRequest, RegisterRequest}, response::LoginResponse },
        event::auth_event::AuthEvent,
    },
    repository::auth_repository,
    server::state::AppState,
    utils::{ self, redis::{ LoginKey, RegisterKey } },
};
use anyhow::{ Context, Result };
use rdkafka::producer::{ FutureRecord, Producer };
use sea_orm::{ DatabaseTransaction, TransactionTrait };
use tracing::info;
use uuid::Uuid;
use crate::client::redis::RedisClientExt;
use crate::constant::{ AUTH_TOPIC, EXPIRE_REGISTER_CODE_SECS };
use crate::model::dto::request::ActiveRequest;
pub async fn login(state: AppState, req: LoginRequest) -> Result<LoginResponse> {
    info!("User login request :{req:?}.");
    todo!()
    // let user = auth_repository
    //     ::find_by_email_and_status(&state.db, &req.email, 0).await?
    //     .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    // utils::password::verify(req.password.clone(), user.password.clone()).await?;
    // // 已经二次验证
    // if user.is2fa == 1 {
    //     // 校验用户是否过期
    //     let key = LoginKey { user_id: user.user_id };
    //     let ttl = utils::redis::get_ttl(&state.redis, &key).await?;
    //     //
    //     if ttl > 0 {
    //         return Ok(LoginResponse::Code {
    //             expire_in: ttl as u64,
    //             message: CHECK_EMAIL_MESSAGE.to_string(),
    //         });
    //     }
    //     let login_code = utils::random::generate_random_string(CODE_LEN);
    //     // 存redis
    //     utils::redis::set(&state.redis, (&key, &login_code)).await?;
    //     return Ok(LoginResponse::Code {
    //         expire_in: EXPIRE_TWO_FACTOR_CODE_SECS.as_secs(),
    //         message: CHECK_EMAIL_MESSAGE.to_string(),
    //     });
    // }
    // let session_id = utils::session::set(&state.redis, user.user_id).await?;
    // let res = utils::token
    // Ok(LoginResponse::Token(resp))
}
pub async fn register(state: AppState, req: RegisterRequest) -> Result<Uuid> {
    info!("Register a new user request: {req:?}.");
    let tx = state.db.begin().await?;

    // 唯一性校验
    check_unique_username_or_email(&tx, &req.username, &req.email).await?;
    // 保存用户
    let user_id = auth_repository::save(
        &tx,
        req.username.clone(),
        req.password.clone(),
        req.email.clone()
    ).await?;
    // 验证码
    let code = generate_active_code();
    // 存redis
    let key = RegisterKey { user_id: user_id };
    utils::redis::set(&state.redis, (&key, &code)).await?;
    // 构建事件对象
    let event = AuthEvent {
        user_id: user_id.to_string(), // 转换UUID为字符串
        email: req.email.clone(),
        code: code.clone(),
    };
    // 序列化事件
    let payload = serde_json::to_string(&event).map_err(|e| anyhow::anyhow!("序列化失败: {}", e))?;
    // 构建Kafka记录
    let record = FutureRecord::to(AUTH_TOPIC).key(&event.user_id).payload(&payload); // 直接使用序列化后的字符串
    // 发送事件
    state.kafka_producer
        .send(record, Duration::from_secs(5)).await
        .map_err(|(e, _)| anyhow::anyhow!("发送消息失败: {}", e))?;
    // 提交事务
    tx.commit().await?;
    Ok(user_id)
}
pub fn generate_active_code() -> String {
    utils::random::generate_random_string(CODE_LEN)
}
pub async fn check_unique_username_or_email(
    tx: &DatabaseTransaction,
    username: &str,
    email: &str
) -> Result<()> {
    auth_repository::check_unique_by_username(tx, username).await?;
    auth_repository::check_unique_by_email(tx, email).await
}
pub async fn active(state: AppState, req: ActiveRequest) -> Result<()> {
    info!("Active a new user request: {req:?}.");
    let tx = state.db.begin().await?;
    // 查用户
    let user = crate::repository::auth_repository
        ::find_by_id(&tx, req.user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    // 判断激活状态
    if user.is_deleted == 0 {
        return Ok(());
    }
    // 校验验证码
    let key = RegisterKey { user_id: req.user_id };
    let redis_code = utils::redis
        ::get(&state.redis, &key).await?
        .ok_or_else(|| anyhow::anyhow!("验证码不存在或已过期"))?;
    if redis_code != req.code {
        return Err(anyhow::anyhow!("验证码错误"));
    }
    // 激活
    auth_repository::active(&tx, user).await?;
    tx.commit().await?;
    Ok(())
}
