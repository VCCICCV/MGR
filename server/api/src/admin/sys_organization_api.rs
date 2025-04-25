use std::sync::Arc;
use model::{
    admin::request::sys_organization::OrganizationPageRequest,
    entities::sys_organization,
};
use service::admin::sys_organization_service::{ SysOrganizationService, TOrganizationService };
use shared::web::{ error::AppError, page::PaginatedData, res::Res };
use axum::{ extract::Query, Extension };

pub struct SysOrganizationApi;

impl SysOrganizationApi {
    pub async fn get_paginated_organizations(
        Query(params): Query<OrganizationPageRequest>,
        Extension(service): Extension<Arc<SysOrganizationService>>
    ) -> Result<Res<PaginatedData<sys_organization::Model>>, AppError> {
        service.find_paginated_organizations(params).await.map(Res::new_data)
    }
}
