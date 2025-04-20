use async_trait::async_trait;
use model::admin::request::sys_login_log::LoginLogPageRequest;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use shared::web::{error::AppError, page::PaginatedData};
use model::entities::prelude::SysLoginLog;
use model::entities::sys_login_log;
use crate::helper::db_helper;

#[async_trait]
pub trait TLoginLogService {
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<sys_login_log::Model>, AppError>;
}

pub struct SysLoginLogService;

#[async_trait]
impl TLoginLogService for SysLoginLogService {
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<sys_login_log::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysLoginLog::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(sys_login_log::Column::Domain.contains(keywords))
                .add(sys_login_log::Column::Username.contains(keywords))
                .add(sys_login_log::Column::Ip.contains(keywords))
                .add(sys_login_log::Column::Address.contains(keywords))
                .add(sys_login_log::Column::UserAgent.contains(keywords));
            query = query.filter(condition);
        }

        query = query.order_by_desc(sys_login_log::Column::CreatedAt);

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }
}
