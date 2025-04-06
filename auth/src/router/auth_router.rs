use axum::routing::{get, post, put};

use crate::server::state::AppState;
use crate::api::auth_handler::register;

pub fn setup_auth_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/auth/register", post(register))
        // .route("/api/auth/activate", get(active))
        // .route("/api/auth/login", post(login))
        // .route("/api/auth/login2fa", post(login2fa))
        // .route("/api/auth/logout", get(logout))
        // .route("/api/auth/password", get(forget_password))
        // .route("/api/auth/password", put(reset_password))
        // .route("/api/auth/profile", get(get_profile))
        // .route("/api/auth/profile", get(update_profile))
}