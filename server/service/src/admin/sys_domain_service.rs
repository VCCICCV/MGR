use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysDomain,
        sea_orm_active_enums::Status,
        sys_domain::{
            ActiveModel as SysDomainActiveModel, Column as SysDomainColumn, Model as SysDomainModel,
        },
    },
    input::{CreateDomainInput, DomainPageRequest, UpdateDomainInput},
};
use ulid::Ulid;

use crate::{admin::sys_domain_error::DomainError, helper::db_helper};

#[async_trait]
pub trait TDomainService {
    async fn find_paginated_domains(
        &self,
        params: DomainPageRequest,
    ) -> Result<PaginatedData<SysDomainModel>, AppError>;

    async fn create_domain(&self, input: CreateDomainInput) -> Result<SysDomainModel, AppError>;
    async fn get_domain(&self, id: &str) -> Result<SysDomainModel, AppError>;
    async fn update_domain(&self, input: UpdateDomainInput) -> Result<SysDomainModel, AppError>;
    async fn delete_domain(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysDomainService;

impl SysDomainService {
    async fn check_domain_exists(
        &self,
        id: Option<&str>,
        code: &str,
        name: &str,
    ) -> Result<(), AppError> {
        let id_str = id.unwrap_or("-1");
        let db = db_helper::get_db_connection().await?;

        let code_exists = SysDomain::find()
            .filter(SysDomainColumn::Code.eq(code))
            .filter(SysDomainColumn::Id.ne(id_str))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if code_exists {
            return Err(DomainError::DuplicateCode.into());
        }

        let name_exists = SysDomain::find()
            .filter(SysDomainColumn::Name.eq(name))
            .filter(SysDomainColumn::Id.ne(id_str))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if name_exists {
            return Err(DomainError::DuplicateName.into());
        }

        Ok(())
    }
}

#[async_trait]
impl TDomainService for SysDomainService {
    async fn find_paginated_domains(
        &self,
        params: DomainPageRequest,
    ) -> Result<PaginatedData<SysDomainModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysDomain::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysDomainColumn::Name.contains(keywords));
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

    async fn create_domain(&self, input: CreateDomainInput) -> Result<SysDomainModel, AppError> {
        self.check_domain_exists(None, &input.code, &input.name)
            .await?;

        let db = db_helper::get_db_connection().await?;

        let domain = SysDomainActiveModel {
            id: Set(Ulid::new().to_string()),
            code: Set(input.code),
            name: Set(input.name),
            description: Set(input.description),
            status: Set(Status::ENABLED),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let result = domain.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_domain(&self, id: &str) -> Result<SysDomainModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysDomain::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| DomainError::DomainNotFound.into())
    }

    async fn update_domain(&self, input: UpdateDomainInput) -> Result<SysDomainModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_domain = self.get_domain(&input.id).await?;

        if existing_domain.code == "built-in" {
            return Err(DomainError::BuiltInDomain.into());
        }

        self.check_domain_exists(Some(&input.id), &input.domain.code, &input.domain.name)
            .await?;

        let mut domain: SysDomainActiveModel = existing_domain.into();
        domain.code = Set(input.domain.code);
        domain.name = Set(input.domain.name);
        domain.description = Set(input.domain.description);

        let updated_domain = domain.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_domain)
    }

    async fn delete_domain(&self, id: &str) -> Result<(), AppError> {
        let domain = self.get_domain(id).await?;

        if domain.code == "built-in" {
            return Err(DomainError::BuiltInDomain.into());
        }

        let db = db_helper::get_db_connection().await?;
        SysDomain::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
