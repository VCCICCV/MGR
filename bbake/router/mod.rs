use std::time::Duration;

use admin::sys_user_router::add_admin_user_routers;
use axum::Router;
use http::{ header, HeaderValue, Method };
use server_router::add_server_routers;
use tower_http::cors::CorsLayer;
use crate::server::state::AppState;
pub mod server_router;
pub mod admin;
pub fn setup_routers(state: AppState) -> Router {
    let cors = CorsLayer::new()
        // 明确指定允许的源（开发环境）
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        // 或者允许多个指定源
        // .allow_origin(["http://localhost:3000", "https://yourdomain.com"])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true) // 与前端保持一致
        .max_age(Duration::from_secs(60 * 60));
    let router = Router::new();
    let router = add_server_routers(router);
    let router = add_admin_user_routers(router);
    router.with_state(state).layer(cors)
}
