use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use shared::global::{add_route, RouteInfo};
pub struct SysUserRouter;

impl SysUserRouter {
    pub async fn init_user_router() -> Router {
        let base_path = "/user";
        let service_name = "SysUserApi";

        let routes = vec![
            RouteInfo::new(
                &format!("{}/users", base_path),
                Method::GET,
                service_name,
                "获取所有用户",
            ),
            RouteInfo::new(base_path, Method::GET, service_name, "获取用户列表"),
            RouteInfo::new(base_path, Method::POST, service_name, "创建用户"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::GET,
                service_name,
                "获取用户详情",
            ),
            RouteInfo::new(base_path, Method::PUT, service_name, "更新用户"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::DELETE,
                service_name,
                "删除用户",
            ),
            RouteInfo::new(
                &format!("{}/add_policies", base_path),
                Method::GET,
                service_name,
                "添加用户策略",
            ),
            RouteInfo::new(
                &format!("{}/remove_policies", base_path),
                Method::GET,
                service_name,
                "删除用户策略",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/users", get(SysUserApi::get_all_users))
            .route("/", get(SysUserApi::get_paginated_users))
            .route("/", post(SysUserApi::create_user))
            .route("/{id}", get(SysUserApi::get_user))
            .route("/", put(SysUserApi::update_user))
            .route("/{id}", delete(SysUserApi::delete_user))
            .route("/add_policies", get(SysUserApi::add_policies))
            .route("/remove_policies", get(SysUserApi::remove_policies));

        Router::new().nest(base_path, router)
    }
}
