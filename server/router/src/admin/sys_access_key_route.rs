use axum::{
    http::Method,
    routing::{delete, get, post},
    Router,
};


pub struct SysAccessKeyRouter;

impl SysAccessKeyRouter {
    pub async fn init_access_key_router() -> Router {
        let base_path = "/access-key";
        let service_name = "SysAccessKeyApi";

        let routes = vec![
            RouteInfo::new(base_path, Method::GET, service_name, "获取访问密钥列表"),
            RouteInfo::new(base_path, Method::POST, service_name, "创建访问密钥"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::DELETE,
                service_name,
                "删除访问密钥",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/", get(SysAccessKeyApi::get_paginated_access_keys))
            .route("/", post(SysAccessKeyApi::create_access_key))
            .route("/{id}", delete(SysAccessKeyApi::delete_access_key));

        Router::new().nest(base_path, router)
    }
}
