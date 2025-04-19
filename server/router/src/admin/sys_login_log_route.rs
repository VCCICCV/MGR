use axum::{http::Method, routing::get, Router};
use server_api::admin::SysLoginLogApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysLoginLogRouter;

impl SysLoginLogRouter {
    pub async fn init_login_log_router() -> Router {
        let base_path = "/login-log";
        let service_name = "SysLoginLogApi";

        let routes = vec![RouteInfo::new(
            base_path,
            Method::GET,
            service_name,
            "获取登录日志列表",
        )];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new().route("/", get(SysLoginLogApi::get_paginated_login_logs));

        Router::new().nest(base_path, router)
    }
}
