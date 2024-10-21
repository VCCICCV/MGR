use crate::{ adapter::user_handler::register_handler, infrastructure::state::AppState };
use axum::{routing::post, Router};
pub async fn setup_user_routes() -> Router<AppState> {
    Router::new().route("/register", post(register_handler))
    // .route("/:id", get(get_user))
    // .route("/", post(create_user))
    // .route("/", put(update_user))
    // .route("/:id", delete(delete_user))
}
