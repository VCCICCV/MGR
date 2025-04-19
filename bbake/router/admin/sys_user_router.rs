use axum::routing::get;

use crate::{ api::admin::sys_user_api::SysUserApi, server::state::AppState };
pub fn add_admin_user_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/admin/users", get(SysUserApi::find_all))
        .route("/api/admin/user/", get(SysUserApi::find_page))
        // .route("/api/admin/", post(SysUserApi::create_user))
    // .route("/api/admin/user/", put(SysUserApi::update))
    // .route("/api/admin/user/{id}", put(SysUserApi::delete))
    // .route("/api/admin/user/{id}", get(SysUserApi::find_by_id))
}
