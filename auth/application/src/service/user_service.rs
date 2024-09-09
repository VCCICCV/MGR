use axum::extract::{Path, Form, Query};
// use infrastructure::persistence::user_persistence::UserPersistence;
use tracing::info;
use domain::model::user::{SysUserSearchReq, UserWithDept};
// use serde_json::json;
// use crate::common::res::ResJson;
// use infrastructure::config::db_config::{DB, get_db_connection};
use crate::common::res::{ListData, PageParams, Res};
use crate::service::user_service_trait;
// pub struct UserService;
pub struct UserService {
    user_service: Box<dyn user_service_trait>,
}

impl UserService {
    /// get_user_list 获取用户列表
    /// page_params 分页参数
    // pub async fn get_users_list(
    //     Query(page_params): Query<PageParams>,
    //     Query(req): Query<SysUserSearchReq>
    // ) -> Res<ListData<UserWithDept>> {
    //     info!("get_users_list");
    //     let res = infrastructure::persistence::user_persistence::get_users_list(page_params, req).await;
    //     match res {
    //         Ok(x) => Res::with_data(x),
    //         Err(e) => Res::with_err(&e.to_string()),
    //     }
    // }
    pub fn new(user_service: Box<dyn user_service_trait>) -> Self {
        Self { user_service }
    }

    pub async fn get_users_list(&self,
                                page_params: Query<PageParams>,
                                req: Query<SysUserSearchReq>,
    ) -> Res<ListData<UserWithDept>> {
        info!("get_users_list");
        let res = self.user_service.get_users_list(page_params.into_inner(), req.into_inner()).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e),
        }
    }
    // pub async fn create_user(form: Form<(String, String)>) -> Result<(), String> {
    //     let (username, email) = form.0;
    //     info!("User created successfully");
    //     println!("User created successfully");
    //     match UserPersistence::create_user(username.as_str(), email.as_str()).await {
    //         Ok(_) => Ok(()),

    //         Err(e) => Err(format!("Failed to create user: {}", e)),
    //     }
    // }

    // pub async fn get_user_by_id(id: Path<i32>) -> Result<Option<String>, String> {
    //     let user_id = *id;
    //     match UserPersistence::get_user_by_id(user_id).await {
    //         Ok(user) => Ok(user.map(|u| format!("User: id={}, username={}, email={}", u.id, u.username, u.email))),
    //         Err(e) => Err(format!("Failed to get user: {}", e)),
    //     }
    // }
    // pub async fn get_user_by_id(id: Path<i32>) -> Result<Option<String>, String> {
    //     let user_id = *id;
    //
    //     match UserPersistence::get_user_by_id(user_id).await {
    //         Ok(user) =>
    //             Ok(
    //                 user.map(|u|
    //                     format!("User: id={}, username={}, email={}", u.id, u.username, u.email)
    //                 )
    //             ),
    //         Err(e) => Err(format!("Failed to get user: {}", e)),
    //     }
    //     // info!("user_id:{}", user_id);
    //     // Err(format!("Failed to get user"))
    // }

    // pub async fn update_user(id: Path<i32>, form: Form<(String, String)>) -> Result<(), String> {
    //     let user_id = *id;
    //     let (username, email) = form.0;
    //     match UserPersistence::update_user(user_id, username.as_str(), email.as_str()).await {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(format!("Failed to update user: {}", e)),
    //     }
    // }

    // pub async fn delete_user(id: Path<i32>) -> Result<(), String> {
    //     let user_id = *id;
    //     match UserPersistence::delete_user(user_id).await {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(format!("Failed to delete user: {}", e)),
    //     }
    // }
}
