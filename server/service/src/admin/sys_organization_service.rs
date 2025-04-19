use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysOrganization,
        sys_organization::{Column as SysOrganizationColumn, Model as SysOrganizationModel},
    },
    input::OrganizationPageRequest,
};

use crate::helper::db_helper;

#[async_trait]
pub trait TOrganizationService {
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<SysOrganizationModel>, AppError>;
}

pub struct SysOrganizationService;

#[async_trait]
impl TOrganizationService for SysOrganizationService {
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<SysOrganizationModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysOrganization::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysOrganizationColumn::Code.contains(keywords))
                .add(SysOrganizationColumn::Name.contains(keywords))
                .add(SysOrganizationColumn::Description.contains(keywords));
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
}
