use std::sync::Arc;
use shared::web::{auth::User, error::AppError, page::PaginatedData, res::Res, validator::ValidatedForm};
use axum::{ extract::{ Path, Query }, Extension };
use axum_casbin::{casbin::MgmtApi, CasbinAxumLayer};
use model::admin::{request::sys_user::{CreateUserInput, UpdateUserInput, UserPageRequest}, response::sys_user::UserWithoutPassword};

use service::admin::sys_user_service::{SysUserService, TUserService};


pub struct SysUserApi;

impl SysUserApi {
    pub async fn get_all_users(Extension(service): Extension<Arc<SysUserService>>) -> Result<
        Res<Vec<UserWithoutPassword>>,
        AppError
    > {
        service.find_all().await.map(Res::new_data)
    }

    pub async fn get_paginated_users(
        Query(params): Query<UserPageRequest>,
        Extension(service): Extension<Arc<SysUserService>>,
        user: User
    ) -> Result<Res<PaginatedData<UserWithoutPassword>>, AppError> {
        print!("user is {:#?}", user);
        service.find_paginated_users(params).await.map(Res::new_data)
    }

    pub async fn remove_policies(Extension(
        mut cache_enforcer,
    ): Extension<CasbinAxumLayer>) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string()
        ];
        let _ = enforcer_write.remove_policies(vec![rule]).await;
        Res::new_data(true)
    }

    pub async fn add_policies(Extension(
        mut cache_enforcer,
    ): Extension<CasbinAxumLayer>) -> Res<bool> {
        let enforcer = cache_enforcer.get_enforcer();
        let mut enforcer_write = enforcer.write().await;
        let rule = vec![
            "1".to_string(),
            "built-in".to_string(),
            "/user/users".to_string(),
            "GET".to_string()
        ];
        let _ = enforcer_write.add_policy(rule).await;
        Res::new_data(true)
    }

    pub async fn create_user(
        Extension(service): Extension<Arc<SysUserService>>,
        ValidatedForm(input): ValidatedForm<CreateUserInput>
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.create_user(input).await.map(Res::new_data)
    }

    pub async fn get_user(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysUserService>>
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.get_user(&id).await.map(Res::new_data)
    }

    pub async fn update_user(
        Extension(service): Extension<Arc<SysUserService>>,
        ValidatedForm(input): ValidatedForm<UpdateUserInput>
    ) -> Result<Res<UserWithoutPassword>, AppError> {
        service.update_user(input).await.map(Res::new_data)
    }

    pub async fn delete_user(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<SysUserService>>
    ) -> Result<Res<()>, AppError> {
        service.delete_user(&id).await.map(Res::new_data)
    }
}
