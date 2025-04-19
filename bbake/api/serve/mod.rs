use axum::{ extract::State, Json };
use crate::{
    client::redis::RedisClientExt,
    server::state::AppState,
    shared::{ error::AppError, res::{ MessageResponse, Res, ServiceStatusResponse } },
};
pub struct ServerApi;
impl ServerApi {
    pub async fn health_check(State(_state): State<AppState>) -> Result<
        Json<Res<MessageResponse>>,
        AppError
    > {
        Ok(Json(Res::with_success(MessageResponse::new("Ok"))))
    }
    pub async fn service_state(State(state): State<AppState>) -> Result<
        Json<Res<ServiceStatusResponse>>,
        AppError
    > {
        let db_status = state.db.ping().await.is_ok();
        let redis_status = state.redis.ping().await.is_ok();

        let resp = ServiceStatusResponse {
            db: db_status,
            redis: redis_status,
        };

        Ok(Json(Res::with_success(resp)))
    }
}
