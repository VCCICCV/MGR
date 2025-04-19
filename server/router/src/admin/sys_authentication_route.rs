use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use server_api::admin::SysAuthenticationApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysAuthenticationRouter;

impl SysAuthenticationRouter {
    pub async fn init_authentication_router() -> Router {
        let router = Router::new().route("/login", post(SysAuthenticationApi::login_handler));
        Router::new().nest("/auth", router)
    }

    pub async fn init_protected_router() -> Router {
        let router = Router::new()
            .route("/getUserInfo", get(SysAuthenticationApi::get_user_info))
            .route("/getUserRoutes", get(SysAuthenticationApi::get_user_routes));

        Router::new().nest("/auth", router)
    }

    pub async fn init_authorization_router() -> Router {
        let base_path = "/authorization";
        let service_name = "SysAuthorizationApi";

        let routes = vec![
            RouteInfo::new(
                &format!("{}/assign-permission", base_path),
                Method::POST,
                service_name,
                "分配权限",
            ),
            RouteInfo::new(
                &format!("{}/assign-routes", base_path),
                Method::POST,
                service_name,
                "分配路由",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let authorization_router = Router::new()
            .route("/getUserRoutes", get(SysAuthenticationApi::get_user_routes))
            .route(
                "/assign-permission",
                post(SysAuthenticationApi::assign_permission),
            )
            .route("/assign-routes", post(SysAuthenticationApi::assign_routes));

        Router::new().nest(base_path, authorization_router)
    }
}
