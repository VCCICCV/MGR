use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{
        prelude::SysLoginLog,
        sys_login_log::{Column as SysLoginLogColumn, Model as SysLoginLogModel},
    },
    input::LoginLogPageRequest,
};

use crate::helper::db_helper;

#[async_trait]
pub trait TLoginLogService {
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<SysLoginLogModel>, AppError>;
}

pub struct SysLoginLogService;

#[async_trait]
impl TLoginLogService for SysLoginLogService {
    async fn find_paginated_login_logs(
        &self,
        params: LoginLogPageRequest,
    ) -> Result<PaginatedData<SysLoginLogModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysLoginLog::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysLoginLogColumn::Domain.contains(keywords))
                .add(SysLoginLogColumn::Username.contains(keywords))
                .add(SysLoginLogColumn::Ip.contains(keywords))
                .add(SysLoginLogColumn::Address.contains(keywords))
                .add(SysLoginLogColumn::UserAgent.contains(keywords));
            query = query.filter(condition);
        }

        query = query.order_by_desc(SysLoginLogColumn::CreatedAt);

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
