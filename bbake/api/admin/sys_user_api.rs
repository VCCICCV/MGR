use axum::{ extract::{ Query, State } };

use crate::{
    model::admin::response::UserWithoutPassword,
    server::state::AppState,
    shared::{ error::AppError, res::{ PageRequest, PageResponse, Res } },
};

pub struct SysUserApi;
impl SysUserApi {
    pub async fn find_all(State(state): State<AppState>) -> Result<
        Res<Vec<UserWithoutPassword>>,
        AppError
    > {
        state.sys_user_service.get_all().await.map(Res::with_success)
    }
    pub async fn find_page(
        State(state): State<AppState>,
        Query(req): Query<PageRequest>
    ) -> Result<Res<PageResponse<UserWithoutPassword>>, AppError> {
        state.sys_user_service.get_page(req).await.map(Res::with_success)
    }

    pub async fn create_user(
        State(state): State<AppState>,
        Query(req): Query<PageRequest>
    ) -> Result<Res<PageResponse<UserWithoutPassword>>, AppError> {
        // state.sys_user_service.post(req).await.map(Res::with_success)
        todo!()
    }
}
