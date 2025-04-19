use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysUser,
        sys_user::{
            ActiveModel as SysUserActiveModel, Column as SysUserColumn, Model as SysUserModel,
        },
    },
    input::{CreateUserInput, UpdateUserInput, UserPageRequest},
    output::UserWithoutPassword,
};
use server_utils::SecureUtil;
use ulid::Ulid;

use super::sys_user_error::UserError;
use crate::helper::db_helper;

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, AppError>;
    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, AppError>;

    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, AppError>;
    async fn get_user(&self, id: &str) -> Result<UserWithoutPassword, AppError>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, AppError>;
    async fn delete_user(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysUserService;

impl SysUserService {
    async fn check_username_unique(&self, username: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_user = SysUser::find()
            .filter(SysUserColumn::Username.eq(username))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if existing_user.is_some() {
            return Err(UserError::UsernameAlreadyExists.into());
        }
        Ok(())
    }

    async fn get_user_by_id(&self, id: String) -> Result<SysUserModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| UserError::UserNotFound.into())
    }
}

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find()
            .all(db.as_ref())
            .await
            .map(|users| users.into_iter().map(UserWithoutPassword::from).collect())
            .map_err(AppError::from)
    }

    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysUser::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysUserColumn::Username.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?
            .into_iter()
            .map(UserWithoutPassword::from)
            .collect();

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, AppError> {
        self.check_username_unique(&input.username).await?;

        let db = db_helper::get_db_connection().await?;
        let user = SysUserActiveModel {
            id: Set(Ulid::new().to_string()),
            domain: Set(input.domain),
            username: Set(input.username),
            password: Set(SecureUtil::hash_password(input.password.as_bytes()).unwrap()),
            built_in: Set(false),
            nick_name: Set(input.nick_name),
            avatar: Set(input.avatar),
            email: Set(input.email),
            phone_number: Set(input.phone_number),
            status: Set(input.status),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let user_model = user.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(UserWithoutPassword::from(user_model))
    }

    async fn get_user(&self, id: &str) -> Result<UserWithoutPassword, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .map(UserWithoutPassword::from)
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, AppError> {
        let mut user = self.get_user_by_id(input.id).await?.into_active_model();

        if input.user.username != *user.username.as_ref() {
            self.check_username_unique(&input.user.username).await?;
        }

        user.domain = Set(input.user.domain);
        user.username = Set(input.user.username);
        user.password = Set(input.user.password); // TODO: Note: In a real application, you should hash the password
        user.nick_name = Set(input.user.nick_name);
        user.avatar = Set(input.user.avatar);
        user.email = Set(input.user.email);
        user.phone_number = Set(input.user.phone_number);
        user.status = Set(input.user.status);

        let db = db_helper::get_db_connection().await?;
        let updated_user = user.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(UserWithoutPassword::from(updated_user))
    }

    async fn delete_user(&self, id: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let result = SysUser::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if result.rows_affected == 0 {
            return Err(UserError::UserNotFound.into());
        }

        Ok(())
    }
}
