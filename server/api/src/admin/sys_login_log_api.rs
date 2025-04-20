use std::sync::Arc;
use model::{admin::request::sys_login_log::LoginLogPageRequest, entities::sys_login_log};
use service::admin::sys_login_log_service::{SysLoginLogService, TLoginLogService};
use shared::web::{error::AppError, page::PaginatedData, res::Res};
use axum::{ extract::Query, Extension };


pub struct SysLoginLogApi;

impl SysLoginLogApi {
    pub async fn get_paginated_login_logs(
        Query(params): Query<LoginLogPageRequest>,
        Extension(service): Extension<Arc<SysLoginLogService>>,
    ) -> Result<Res<PaginatedData<sys_login_log::Model>>, AppError> {
        service
            .find_paginated_login_logs(params)
            .await
            .map(Res::new_data)
    }
}
