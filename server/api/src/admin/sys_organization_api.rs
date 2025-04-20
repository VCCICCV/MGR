
use std::sync::Arc;
use shared::web::{auth::User, error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use axum::{ extract::{ Path, Query }, Extension };
use axum::extract::{Extension, Query};

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
