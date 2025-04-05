use axum::routing::get;

use crate::{api::server_handler::health_check, server::state::AppState};
pub fn setup_auth_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/register", get(health_check))
        .route("/api/activate", get(health_check))
        .route("/api/jjj", get(health_check))
}