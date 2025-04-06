use crate::{
    constant::CODE_LEN,
    model::{ dto::request::RegisterRequest, event::auth_event::Event },
    repository::{ auth_repository, event_repository },
    server::state::AppState,
    utils,
};
use anyhow::Result;
use sea_orm::{ DatabaseTransaction, TransactionTrait };
use tracing::info;
use uuid::Uuid;
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
    // 注册事件
    let event = Event::builder()
        .id(user_id)
        .event_type("user_registered")
        .payload(
            serde_json::json!({
            "email": req.email,
            "code": code,
        })
        )
        .build()?;
    event_repository::save(&tx, event).await?;
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
