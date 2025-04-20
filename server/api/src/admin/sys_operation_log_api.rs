use std::sync::Arc;
use model::{admin::request::sys_operation_log::OperationLogPageRequest, entities::sys_operation_log};
use service::admin::sys_operation_log_service::{SysOperationLogService, TOperationLogService};
use shared::web::{auth::User, error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use axum::{ extract::{ Path, Query }, Extension };

use axum::extract::{Extension, Query};


pub struct SysOperationLogApi;

impl SysOperationLogApi {
    pub async fn get_paginated_operation_logs(
        Query(params): Query<OperationLogPageRequest>,
        Extension(service): Extension<Arc<SysOperationLogService>>,
    ) -> Result<Res<PaginatedData<sys_operation_log::Model>>, AppError> {
        service
            .find_paginated_operation_logs(params)
            .await
            .map(Res::new_data)
    }
}
