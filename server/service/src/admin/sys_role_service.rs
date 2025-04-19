use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysRole,
        sys_role::{
            ActiveModel as SysRoleActiveModel, Column as SysRoleColumn, Model as SysRoleModel,
        },
    },
    input::{CreateRoleInput, RolePageRequest, UpdateRoleInput},
};

use super::sys_role_error::RoleError;
use crate::helper::db_helper;
use ulid::Ulid;

#[async_trait]
pub trait TRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, AppError>;

    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, AppError>;
    async fn get_role(&self, id: &str) -> Result<SysRoleModel, AppError>;
    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, AppError>;
    async fn delete_role(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysRoleService;

impl SysRoleService {
    async fn check_role_exists(&self, id: Option<&str>, code: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find().filter(SysRoleColumn::Code.eq(code));

        if let Some(id) = id {
            query = query.filter(SysRoleColumn::Id.ne(id));
        }

        let existing_role = query.one(db.as_ref()).await.map_err(AppError::from)?;

        if existing_role.is_some() {
            return Err(RoleError::DuplicateRoleCode.into());
        }

        Ok(())
    }
}

#[async_trait]
impl TRoleService for SysRoleService {
    async fn find_paginated_roles(
        &self,
        params: RolePageRequest,
    ) -> Result<PaginatedData<SysRoleModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysRole::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysRoleColumn::Name.contains(keywords));
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
            .map_err(AppError::from)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_role(&self, input: CreateRoleInput) -> Result<SysRoleModel, AppError> {
        let db = db_helper::get_db_connection().await?;

        self.check_role_exists(None, &input.code).await?;

        let role = SysRoleActiveModel {
            id: Set(Ulid::new().to_string()),
            pid: Set(input.pid),
            code: Set(input.code),
            name: Set(input.name),
            status: Set(input.status),
            description: Set(input.description),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let result = role.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_role(&self, id: &str) -> Result<SysRoleModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| RoleError::RoleNotFound.into())
    }

    async fn update_role(&self, input: UpdateRoleInput) -> Result<SysRoleModel, AppError> {
        let db = db_helper::get_db_connection().await?;

        self.check_role_exists(Some(&input.id), &input.role.code)
            .await?;

        let role: SysRoleActiveModel = SysRole::find_by_id(&input.id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(RoleError::RoleNotFound))?
            .into();

        let role = SysRoleActiveModel {
            id: Set(input.id.clone()),
            pid: Set(input.role.pid),
            code: Set(input.role.code),
            name: Set(input.role.name),
            description: Set(input.role.description),

            updated_at: Set(Some(Local::now().naive_local())),
            ..role
        };

        let updated_role = role.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_role)
    }

    async fn delete_role(&self, id: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        SysRole::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
