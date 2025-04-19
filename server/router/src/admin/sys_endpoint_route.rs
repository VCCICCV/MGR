use axum::{http::Method, routing::get, Router};
use server_api::admin::SysEndpointApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysEndpointRouter;

impl SysEndpointRouter {
    pub async fn init_endpoint_router() -> Router {
        let base_path = "/api-endpoint";
        let service_name = "SysEndpointApi";

        let routes = vec![
            RouteInfo::new(base_path, Method::GET, service_name, "获取接口列表"),
            RouteInfo::new(
                &format!("{}/auth-api-endpoint/:roleCode", base_path),
                Method::GET,
                service_name,
                "获取角色API权限",
            ),
            RouteInfo::new(
                &format!("{}/tree", base_path),
                Method::GET,
                service_name,
                "获取接口树",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/", get(SysEndpointApi::get_paginated_endpoints))
            .route(
                "/auth-api-endpoint/{roleCode}",
                get(SysEndpointApi::get_auth_endpoints),
            )
            .route("/tree", get(SysEndpointApi::tree_endpoint));

        Router::new().nest(base_path, router)
    }
}
