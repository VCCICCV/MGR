use std::sync::Arc;

use axum::extract::{Extension, Query};
use server_core::web::{error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    OperationLogPageRequest, SysOperationLogModel, SysOperationLogService, TOperationLogService,
};

pub struct SysOperationLogApi;

impl SysOperationLogApi {
    pub async fn get_paginated_operation_logs(
        Query(params): Query<OperationLogPageRequest>,
        Extension(service): Extension<Arc<SysOperationLogService>>,
    ) -> Result<Res<PaginatedData<SysOperationLogModel>>, AppError> {
        service
            .find_paginated_operation_logs(params)
            .await
            .map(Res::new_data)
    }
}
