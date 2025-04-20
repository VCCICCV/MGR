use async_trait::async_trait;
use model::admin::request::sys_organization::OrganizationPageRequest;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use shared::web::{error::AppError, page::PaginatedData};
use model::entities::prelude::SysOrganization;
use model::entities::sys_organization;

use crate::helper::db_helper;

#[async_trait]
pub trait TOrganizationService {
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<sys_organization::Model>, AppError>;
}

pub struct SysOrganizationService;

#[async_trait]
impl TOrganizationService for SysOrganizationService {
    async fn find_paginated_organizations(
        &self,
        params: OrganizationPageRequest,
    ) -> Result<PaginatedData<sys_organization::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysOrganization::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(sys_organization::Column::Code.contains(keywords))
                .add(sys_organization::Column::Name.contains(keywords))
                .add(sys_organization::Column::Description.contains(keywords));
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
