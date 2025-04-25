use std::sync::Arc;
use model::{
    admin::request::sys_operation_log::OperationLogPageRequest,
    entities::sys_operation_log,
};
use service::admin::sys_operation_log_service::{ SysOperationLogService, TOperationLogService };
use shared::web::{ error::AppError, page::PaginatedData, res::Res };
use axum::{ extract::Query, Extension };

pub struct SysOperationLogApi;

impl SysOperationLogApi {
    pub async fn get_paginated_operation_logs(
        Query(params): Query<OperationLogPageRequest>,
        Extension(service): Extension<Arc<SysOperationLogService>>
    ) -> Result<Res<PaginatedData<sys_operation_log::Model>>, AppError> {
        service.find_paginated_operation_logs(params).await.map(Res::new_data)
    }
}
