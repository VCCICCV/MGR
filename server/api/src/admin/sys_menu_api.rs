use std::sync::Arc;

use axum::{extract::Path, Extension};
use server_core::web::{auth::User, error::AppError, res::Res, validator::ValidatedForm};
use server_service::admin::{
    CreateMenuInput, MenuRoute, MenuTree, SysMenuModel, SysMenuService, TMenuService,
    UpdateMenuInput,
};

pub struct SysMenuApi;

impl SysMenuApi {
    pub async fn tree_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Res<Vec<MenuTree>>, AppError> {
        service.tree_menu().await.map(Res::new_data)
    }

    pub async fn get_menu_list(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Res<Vec<MenuTree>>, AppError> {
        service.get_menu_list().await.map(Res::new_data)
    }

    pub async fn get_constant_routes(
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Res<Vec<MenuRoute>>, AppError> {
        service.get_constant_routes().await.map(Res::new_data)
    }

    pub async fn create_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Extension(user): Extension<User>,
        ValidatedForm(input): ValidatedForm<CreateMenuInput>,
    ) -> Result<Res<SysMenuModel>, AppError> {
        service.create_menu(input, user).await.map(Res::new_data)
    }

    pub async fn get_menu(
        Path(id): Path<i32>,
        Extension(service): Extension<Arc<SysMenuService>>,
    ) -> Result<Res<SysMenuModel>, AppError> {
        service.get_menu(id).await.map(Res::new_data)
    }

    pub async fn update_menu(
        Extension(service): Extension<Arc<SysMenuService>>,
        Extension(user): Extension<User>,
        ValidatedForm(input): ValidatedForm<UpdateMenuInput>,
    ) -> Result<Res<SysMenuModel>, AppError> {
        service.update_menu(input, user).await.map(Res::new_data)
    }

    pub async fn delete_menu(
        Path(id): Path<i32>,
        Extension(service): Extension<Arc<SysMenuService>>,
        Extension(user): Extension<User>,
    ) -> Result<Res<()>, AppError> {
        print!("user is {:#?}", user);
        service.delete_menu(id, user).await.map(Res::new_data)
    }

    pub async fn get_auth_routes(
        Path(role_id): Path<String>,
        Extension(service): Extension<Arc<SysMenuService>>,
        Extension(user): Extension<User>,
    ) -> Result<Res<Vec<i32>>, AppError> {
        service
            .get_menu_ids_by_role_id(role_id, user.domain())
            .await
            .map(Res::new_data)
    }
}
