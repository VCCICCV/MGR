use axum::routing::post;

use crate::{ api::token::{info, refresh}, state::AppState };

pub async fn setup_token_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/api/token/refresh", post(refresh))
    .route("/api/token/info", post(info))
}
