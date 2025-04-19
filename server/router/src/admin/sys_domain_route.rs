use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysDomainApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysDomainRouter;

impl SysDomainRouter {
    pub async fn init_domain_router() -> Router {
        let base_path = "/domain";
        let service_name = "SysDomainApi";

        let routes = vec![
            RouteInfo::new(base_path, Method::GET, service_name, "获取域名列表"),
            RouteInfo::new(base_path, Method::POST, service_name, "创建域名"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::GET,
                service_name,
                "获取域名详情",
            ),
            RouteInfo::new(base_path, Method::PUT, service_name, "更新域名"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::DELETE,
                service_name,
                "删除域名",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/", get(SysDomainApi::get_paginated_domains))
            .route("/", post(SysDomainApi::create_domain))
            .route("/{id}", get(SysDomainApi::get_domain))
            .route("/", put(SysDomainApi::update_domain))
            .route("/{id}", delete(SysDomainApi::delete_domain));

        Router::new().nest(base_path, router)
    }
}
