use std::any::Any;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_global::{global::OperationLogContext, project_error};
use server_model::admin::{
    entities::{
        prelude::SysOperationLog,
        sys_operation_log::{
            ActiveModel as SysOperationLogActiveModel, Column as SysOperationLogColumn,
            Model as SysOperationLogModel,
        },
    },
    input::OperationLogPageRequest,
};
use tracing::instrument;
use ulid::Ulid;

use crate::helper::db_helper;

#[async_trait]
pub trait TOperationLogService {
    async fn find_paginated_operation_logs(
        &self,
        params: OperationLogPageRequest,
    ) -> Result<PaginatedData<SysOperationLogModel>, AppError>;

    async fn handle_operation_log_event(event: &OperationLogContext) -> Result<(), AppError>;
}

pub struct SysOperationLogService;

#[async_trait]
impl TOperationLogService for SysOperationLogService {
    async fn find_paginated_operation_logs(
        &self,
        params: OperationLogPageRequest,
    ) -> Result<PaginatedData<SysOperationLogModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysOperationLog::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any()
                .add(SysOperationLogColumn::Domain.contains(keywords))
                .add(SysOperationLogColumn::Username.contains(keywords))
                .add(SysOperationLogColumn::Ip.contains(keywords))
                .add(SysOperationLogColumn::UserAgent.contains(keywords));
            query = query.filter(condition);
        }

        query = query.order_by_desc(SysOperationLogColumn::CreatedAt);

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

    async fn handle_operation_log_event(event: &OperationLogContext) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        SysOperationLogActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(event.user_id.clone().unwrap_or_default()),
            username: Set(event.username.clone().unwrap_or_default()),
            domain: Set(event.domain.clone().unwrap_or_default()),
            module_name: Set(event.module_name.clone()),
            description: Set(event.description.clone()),
            request_id: Set(event.request_id.clone()),
            method: Set(event.method.clone()),
            url: Set(event.url.clone()),
            ip: Set(event.ip.clone()),
            user_agent: Set(event.user_agent.clone()),
            params: Set(event.params.clone()),
            body: Set(event.body.clone()),
            response: Set(event.response.clone()),
            start_time: Set(event.start_time),
            end_time: Set(event.end_time),
            duration: Set(event.duration),
            created_at: Set(event.created_at),
        }
        .insert(db.as_ref())
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

#[instrument(skip(rx))]
pub async fn sys_operation_log_listener(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Any + Send>>,
) {
    while let Some(event) = rx.recv().await {
        if let Some(operation_log_context) = event.downcast_ref::<OperationLogContext>() {
            if let Err(e) =
                SysOperationLogService::handle_operation_log_event(operation_log_context).await
            {
                project_error!("Failed to handle operation log event: {:?}", e);
            }
        } else {
            project_error!("Received unknown event type in operation log listener");
        }
    }
}
