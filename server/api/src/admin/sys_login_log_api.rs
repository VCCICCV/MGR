use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    LoginLogPageRequest, SysLoginLogModel, SysLoginLogService, TLoginLogService,
};

pub struct SysLoginLogApi;

impl SysLoginLogApi {
    pub async fn get_paginated_login_logs(
        Query(params): Query<LoginLogPageRequest>,
        Extension(service): Extension<Arc<SysLoginLogService>>,
    ) -> Result<Res<PaginatedData<SysLoginLogModel>>, AppError> {
        service
            .find_paginated_login_logs(params)
            .await
            .map(Res::new_data)
    }
}
