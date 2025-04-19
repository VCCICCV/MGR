use axum::response::IntoResponse;
use tracing::info;

use crate::{
    cmd::shutdown::shutdown_signal, configure::{ self, env::get_env_source, AppConfig }, server::state::AppState, shared::{ constant::ENV_PREFIX, error::AppError, res::{ EmptyData, Res } },
    router::setup_routers
};

pub async fn start() -> Result<(), AppError> {

}

