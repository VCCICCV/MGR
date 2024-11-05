use axum::{routing::post, Router};
use infrastructure::state::AppState;
use crate::adapter::user_handler::get_all_customer;
pub async fn setup_user_routes() -> Router<AppState> {
    Router::new().route("/get_all", post(get_all_customer))
    // .route("/:id", get(get_user))
    // .route("/", post(create_user))
    // .route("/", put(update_user))
    // .route("/:id", delete(delete_user))
}
