use axum::Router;
use server_router::add_server_routers;
use crate::server::state::AppState;
pub mod server_router;
pub fn setup_routers(state: AppState) -> Router {
  let router = Router::new();
  let router = add_server_routers(router);
  router.with_state(state)
}
