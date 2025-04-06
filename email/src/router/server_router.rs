use axum::routing::get;


use crate::{api::server_handler::health_check, server::state::AppState};


pub fn setup_server_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/auth/health_check", get(health_check))
}