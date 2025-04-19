use axum::{http::Method, routing::get, Router};
use server_api::admin::SysOperationLogApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysOperationLogRouter;

impl SysOperationLogRouter {
    pub async fn init_operation_log_router() -> Router {
        let base_path = "/operation-log";
        let service_name = "SysOperationLogApi";

        let routes = vec![RouteInfo::new(
            base_path,
            Method::GET,
            service_name,
            "获取操作日志列表",
        )];

        for route in routes {
            add_route(route).await;
        }

        let router =
            Router::new().route("/", get(SysOperationLogApi::get_paginated_operation_logs));

        Router::new().nest(base_path, router)
    }
}
