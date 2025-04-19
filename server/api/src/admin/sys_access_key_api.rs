use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use shared::{error::AppError, res::{PageData, Res}, utils::validator::ValidatedForm};


pub struct SysAccessKeyApi;

impl SysAccessKeyApi {
    pub async fn get_paginated_access_keys(
        Query(params): Query<AccessKeyPageRequest>,
        Extension(service): Extension<Arc<SysAccessKeyService>>,
    ) -> Result<Res<PageData<SysAccessKeyModel>>, AppError> {
        service
            .find_paginated_access_keys(params)
            .await
            .map(Res::with_success(""))
    }

    pub async fn create_access_key(
        Extension(service): Extension<Arc<SysAccessKeyService>>,
        ValidatedForm(input): ValidatedForm<CreateAccessKeyInput>,
    ) -> Result<Res<SysAccessKeyModel>, AppError> {
        service.create_access_key(input).await.map(Res::new_data)
    }

    pub async fn delete_access_key(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysAccessKeyService>>,
    ) -> Result<Res<()>, AppError> {
        service.delete_access_key(&id).await.map(Res::new_data)
    }
}
