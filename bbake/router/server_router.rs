use axum::routing::get;

use crate::{api::serve::ServerApi, server::state::AppState};


pub fn add_server_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/health_check", get(ServerApi::health_check))
        .route("/api/service_state", get(ServerApi::service_state))
}
