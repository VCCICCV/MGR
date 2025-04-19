use axum::{http::Method, routing::get, Router};
use server_api::admin::SysOrganizationApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysOrganizationRouter;

impl SysOrganizationRouter {
    pub async fn init_organization_router() -> Router {
        let base_path = "/org";
        let service_name = "SysOrganizationApi";

        let routes = vec![RouteInfo::new(
            base_path,
            Method::GET,
            service_name,
            "获取组织列表",
        )];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new().route("/", get(SysOrganizationApi::get_paginated_organizations));

        Router::new().nest(base_path, router)
    }
}
