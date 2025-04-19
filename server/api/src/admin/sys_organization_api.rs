use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    OrganizationPageRequest, SysOrganizationModel, SysOrganizationService, TOrganizationService,
};

pub struct SysOrganizationApi;

impl SysOrganizationApi {
    pub async fn get_paginated_organizations(
        Query(params): Query<OrganizationPageRequest>,
        Extension(service): Extension<Arc<SysOrganizationService>>,
    ) -> Result<Res<PaginatedData<SysOrganizationModel>>, AppError> {
        service
            .find_paginated_organizations(params)
            .await
            .map(Res::new_data)
    }
}
