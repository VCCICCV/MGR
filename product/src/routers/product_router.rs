use axum::Router;
use axum::routing::{get, post};

use crate::interface::api::product_handler::list_categories;
use crate::state::AppState;
pub async fn setup_routers(router: Router<AppState>) -> Router<AppState> {
    router.route("/api/product/categories", get(list_categories))
}
// .route("/api/product/spu/:id", get_spu)
// .route("/api/product/page", get_spu_page)
// .route("/api/stock/verify", verify_stock)
// .route("/api/product/tock/lock", lock_stock)
// .route("/api/product/stock/unlock", unlock_stock)
