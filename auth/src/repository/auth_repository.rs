use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{ NotSet, Set },
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
};
use uuid::Uuid;
use anyhow::{ Context, Result };
use crate::{ model::entities::user, utils };
#[tracing::instrument(skip(tx))]
pub async fn save(
    tx: &DatabaseTransaction,
    username: String,
    password: String,
    email: String
) -> Result<Uuid> {
    let user = (user::ActiveModel {
        id: NotSet,
        user_id: Set(Uuid::new_v4()),
        username: Set(username),
        email: Set(email),
        password: Set(utils::password::hash(password).await?),
        avatar: NotSet,
        is_deleted: Set(0),
        is2fa: Set(0),
        create_time: Set(Utc::now().naive_utc()),
        update_time: Set(Some(Utc::now().naive_utc())),
    }).insert(tx).await?;
    // 
    Ok(user.user_id)
}
#[tracing::instrument(skip(tx))]
pub async fn check_unique_by_email(tx: &DatabaseTransaction, email: &str) -> Result<()> {
    // 构建类型安全查询
    let existing = user::Entity
        ::find()
        .filter(user::Column::Email.eq(email))
        .one(tx).await
        .context("数据库查询失败")?;

    // 唯一性校验逻辑
    if existing.is_some() {
        anyhow::bail!("邮箱 '{}' 已被注册", email);
    }
    Ok(())
}
#[tracing::instrument]
pub async fn check_unique_by_username(tx: &DatabaseTransaction, username: &str) -> Result<()> {
    // 使用类型安全查询构建器[1,6](@ref)
    let existing = user::Entity
        ::find()
        .filter(user::Column::Email.eq(username))
        .one(tx).await
        .context("数据库查询失败")?; 

    // 显式校验唯一性逻辑[4](@ref)
    if existing.is_some() {
        anyhow::bail!("用户名 '{}' 已被注册", username); // 使用anyhow返回业务错误[5](@ref)
    }

    Ok(())
}
