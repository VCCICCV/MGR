use std::sync::Arc;

use axum::{ extract::{ Path, Query }, Extension };
use model::{admin::request::sys_access_key::{AccessKeyPageRequest, CreateAccessKeyInput}, entities::sys_access_key};
use service::admin::sys_access_key_service::{SysAccessKeyService, TAccessKeyService};
use shared::web::{error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
pub struct SysAccessKeyApi;

impl SysAccessKeyApi {
    pub async fn get_paginated_access_keys(
        Query(params): Query<AccessKeyPageRequest>,
        Extension(service): Extension<Arc<SysAccessKeyService>>
    ) -> Result<Res<PaginatedData<sys_access_key::Model>>, AppError> {
        service.find_paginated_access_keys(params).await.map(Res::new_data)
    }

    pub async fn create_access_key(
        Extension(service): Extension<Arc<SysAccessKeyService>>,
        ValidatedForm(input): ValidatedForm<CreateAccessKeyInput>
    ) -> Result<Res<sys_access_key::Model>, AppError> {
        service.create_access_key(input).await.map(Res::new_data)
    }

    pub async fn delete_access_key(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysAccessKeyService>>
    ) -> Result<Res<()>, AppError> {
        service.delete_access_key(&id).await.map(Res::new_data)
    }
}
