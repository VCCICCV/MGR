use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysMenuApi;
use server_core::web::operation_log::OperationLogLayer;
use server_global::global::{add_route, RouteInfo};

pub struct SysMenuRouter;

impl SysMenuRouter {
    pub async fn init_menu_router() -> Router {
        let router = Router::new().route(
            "/getConstantRoutes",
            get(SysMenuApi::get_constant_routes).layer(OperationLogLayer::new(true)),
        );
        Router::new().nest("/route", router)
    }

    pub async fn init_protected_menu_router() -> Router {
        let base_path = "/route";
        let service_name = "SysMenuApi";

        let routes = vec![
            RouteInfo::new(
                &format!("{}/tree", base_path),
                Method::GET,
                service_name,
                "获取菜单树",
            ),
            RouteInfo::new(base_path, Method::GET, service_name, "获取菜单列表"),
            RouteInfo::new(base_path, Method::POST, service_name, "创建菜单"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::GET,
                service_name,
                "获取菜单详情",
            ),
            RouteInfo::new(base_path, Method::PUT, service_name, "更新菜单"),
            RouteInfo::new(
                &format!("{}/:id", base_path),
                Method::DELETE,
                service_name,
                "删除菜单",
            ),
            RouteInfo::new(
                &format!("{}/auth-route/:roleId", base_path),
                Method::GET,
                service_name,
                "获取角色菜单",
            ),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/tree", get(SysMenuApi::tree_menu))
            .route("/", get(SysMenuApi::get_menu_list))
            .route("/", post(SysMenuApi::create_menu))
            .route("/{id}", get(SysMenuApi::get_menu))
            .route("/", put(SysMenuApi::update_menu))
            .route("/{id}", delete(SysMenuApi::delete_menu))
            .route("/auth-route/{roleId}", get(SysMenuApi::get_auth_routes));

        Router::new().nest(base_path, router)
    }
}
