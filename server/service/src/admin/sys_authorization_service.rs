use std::sync::Arc;

use async_trait::async_trait;

use model::entities::{
        prelude::{ SysDomain, SysEndpoint, SysMenu, SysRole, SysRoleMenu, SysUser, SysUserRole },
        sys_domain,
        sys_endpoint,
        sys_menu,
        sys_role,
        sys_role_menu,
        sys_user_role,
    };
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter, TransactionTrait };
use shared::web::error::AppError;
use thiserror::Error;
use tokio::sync::RwLock;
use axum_casbin::casbin::{ CoreApi, MgmtApi, RbacApi };
use crate::helper::db_helper;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("Domain not found")]
    DomainNotFound,
    #[error("Role not found")]
    RoleNotFound,
    #[error("One or more permissions not found")]
    PermissionsNotFound,
    #[error("One or more routes not found")]
    RoutesNotFound,
    #[error("One or more users not found")]
    UsersNotFound,
}

impl From<AuthorizationError> for AppError {
    fn from(error: AuthorizationError) -> Self {
        AppError {
            code: 400,
            message: error.to_string(),
        }
    }
}

#[async_trait]
pub trait TAuthorizationService: Send + Sync {
    /// 为角色分配权限
    async fn assign_permission(
        &self,
        domain: String,
        role_id: String,
        permissions: Vec<String>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync>>
    ) -> Result<(), AppError>;

    /// 为角色分配路由
    async fn assign_routes(
        &self,
        domain: String,
        role_id: String,
        route_ids: Vec<i32>
    ) -> Result<(), AppError>;

    /// 为角色分配用户
    async fn assign_users(&self, role_id: String, user_ids: Vec<String>) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysAuthorizationService;

impl SysAuthorizationService {
    async fn check_domain_and_role(
        &self,
        domain_code: &str,
        role_id: &str
    ) -> Result<(String, String, String), AppError> {
        let db = db_helper::get_db_connection().await?;

        let domain = SysDomain
            ::find()
            .filter(sys_domain::Column::Code.eq(domain_code))
            .one(db.as_ref()).await
            .map_err(AppError::from)?;

        let domain = domain.ok_or_else(|| AuthorizationError::DomainNotFound)?;

        let role = SysRole
            ::find()
            .filter(sys_role::Column::Id.eq(role_id))
            .one(db.as_ref()).await
            .map_err(AppError::from)?;

        let role = role.ok_or_else(|| AuthorizationError::RoleNotFound)?;

        Ok((domain.code, role_id.to_string(), role.code))
    }

    async fn check_role(&self, role_id: &str) -> Result<String, AppError> {
        let db = db_helper::get_db_connection().await?;

        let role = SysRole
            ::find()
            .filter(sys_role::Column::Id.eq(role_id))
            .one(db.as_ref()).await
            .map_err(AppError::from)?;

        let role = role.ok_or_else(|| AuthorizationError::RoleNotFound)?;

        Ok(role.code)
    }

    /// 同步角色权限
    async fn sync_role_permissions(
        &self,
        role_code: &str,
        domain: &str,
        new_permissions: Vec<model::entities::sys_endpoint::Model>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync>>
    ) -> Result<(), AppError> {
        let mut enforcer_write = enforcer.write().await;
        let existing_permissions = enforcer_write.get_filtered_policy(
            0,
            vec![role_code.to_string(), domain.to_string()]
        );

        println!("existing_permissions: {:?}", existing_permissions);

        let new_policies: Vec<Vec<String>> = new_permissions
            .iter()
            .map(|perm| {
                vec![
                    role_code.to_string(),
                    domain.to_string(),
                    perm.path.clone(),
                    perm.method.clone()
                ]
            })
            .collect();

        println!("new_policies: {:?}", new_policies);

        let existing_policies: Vec<Vec<String>> = existing_permissions
            .iter()
            .map(|perm| {
                vec![perm[0].clone(), perm[1].clone(), perm[2].clone(), perm[3].clone()]
            })
            .collect();

        println!("existing_policies: {:?}", existing_policies);

        let policies_to_remove: Vec<Vec<String>> = existing_policies
            .iter()
            .filter(|policy| !new_policies.contains(policy))
            .cloned()
            .collect();

        let policies_to_add: Vec<Vec<String>> = new_policies
            .iter()
            .filter(|policy| !existing_policies.contains(policy))
            .cloned()
            .collect();

        if !policies_to_remove.is_empty() {
            let _ = enforcer_write.remove_policies(policies_to_remove).await.map_err(|e| AppError {
                code: 500,
                message: e.to_string(),
            })?;
        }

        if !policies_to_add.is_empty() {
            let _ = enforcer_write.add_policies(policies_to_add).await.map_err(|e| AppError {
                code: 500,
                message: e.to_string(),
            })?;
        }

        Ok(())
    }
}

#[async_trait]
impl TAuthorizationService for SysAuthorizationService {
    async fn assign_permission(
        &self,
        domain: String,
        role_id: String,
        permissions: Vec<String>,
        enforcer: Arc<RwLock<impl CoreApi + MgmtApi + RbacApi + Send + Sync>>
    ) -> Result<(), AppError> {
        let (domain_code, _, role_code) = self.check_domain_and_role(&domain, &role_id).await?;

        let db = db_helper::get_db_connection().await?;
        let permissions = SysEndpoint
            ::find()
            .filter(sys_endpoint::Column::Id.is_in(permissions))
            .all(db.as_ref()).await
            .map_err(AppError::from)?;

        if permissions.is_empty() {
            return Err(AuthorizationError::PermissionsNotFound.into());
        }

        self.sync_role_permissions(&role_code, &domain_code, permissions, enforcer).await?;

        Ok(())
    }

    async fn assign_routes(
        &self,
        domain: String,
        role_id: String,
        route_ids: Vec<i32>
    ) -> Result<(), AppError> {
        let (domain_code, role_id, _) = self.check_domain_and_role(&domain, &role_id).await?;

        let db = db_helper::get_db_connection().await?;
        let routes = SysMenu
            ::find()
            .filter(sys_menu::Column::Id.is_in(route_ids.clone()))
            .all(db.as_ref()).await
            .map_err(AppError::from)?;

        if routes.is_empty() {
            return Err(AuthorizationError::RoutesNotFound.into());
        }

        let existing_routes = SysRoleMenu
            ::find()
            .filter(
                sys_role_menu::Column::RoleId
                    .eq(&role_id)
                    .and(sys_role_menu::Column::Domain.eq(&domain_code))
            )
            .all(db.as_ref()).await
            .map_err(AppError::from)?;

        let existing_route_ids: Vec<i32> = existing_routes
            .iter()
            .map(|r| r.menu_id)
            .collect();

        let new_route_ids: Vec<i32> = route_ids
            .iter()
            .filter(|id| !existing_route_ids.contains(id))
            .cloned()
            .collect();

        let route_ids_to_delete: Vec<i32> = existing_route_ids
            .iter()
            .filter(|id| !route_ids.contains(id))
            .cloned()
            .collect();

        let txn = db.begin().await.map_err(AppError::from)?;

        if !new_route_ids.is_empty() {
            let role_menus: Vec<sys_role_menu::ActiveModel> = new_route_ids
                .iter()
                .map(|route_id| sys_role_menu::ActiveModel {
                    role_id: sea_orm::Set(role_id.clone()),
                    menu_id: sea_orm::Set(*route_id),
                    domain: sea_orm::Set(domain_code.clone()),
                    ..Default::default()
                })
                .collect();

            SysRoleMenu
                ::insert_many(role_menus)
                .exec(&txn).await
                .map_err(AppError::from)?;
        }

        if !route_ids_to_delete.is_empty() {
             SysRoleMenu
                ::delete_many()
                .filter(
                    sys_role_menu::Column::RoleId
                        .eq(&role_id)
                        .and(sys_role_menu::Column::Domain.eq(&domain_code))
                        .and(sys_role_menu::Column::MenuId.is_in(route_ids_to_delete))
                )
                .exec(&txn).await
                .map_err(AppError::from)?;
        }

        txn.commit().await.map_err(AppError::from)?;

        Ok(())
    }

    async fn assign_users(&self, role_id: String, user_ids: Vec<String>) -> Result<(), AppError> {
        let _ = self.check_role(&role_id).await?;

        let db = db_helper::get_db_connection().await?;
        let users = SysUser
            ::find()
            .filter(model::entities::sys_user::Column::Id.is_in(user_ids.clone()))
            .all(db.as_ref()).await
            .map_err(AppError::from)?;

        if users.is_empty() {
            return Err(AuthorizationError::UsersNotFound.into());
        }

        let existing_user_roles = SysUserRole::find()
            .filter(sys_user_role::Column::RoleId.eq(&role_id))
            .all(db.as_ref()).await
            .map_err(AppError::from)?;

        let existing_user_ids: Vec<String> = existing_user_roles
            .iter()
            .map(|r| r.user_id.clone())
            .collect();

        let new_user_ids: Vec<String> = user_ids
            .iter()
            .filter(|id| !existing_user_ids.contains(id))
            .cloned()
            .collect();

        let user_ids_to_delete: Vec<String> = existing_user_ids
            .iter()
            .filter(|id| !user_ids.contains(id))
            .cloned()
            .collect();

        let txn = db.begin().await.map_err(AppError::from)?;

        if !new_user_ids.is_empty() {
            let user_roles: Vec<sys_user_role::ActiveModel> = new_user_ids
                .iter()
                .map(|user_id| sys_user_role::ActiveModel {
                    role_id: sea_orm::Set(role_id.clone()),
                    user_id: sea_orm::Set(user_id.clone()),
                    ..Default::default()
                })
                .collect();

            SysUserRole::insert_many(user_roles).exec(&txn).await.map_err(AppError::from)?;
        }

        if !user_ids_to_delete.is_empty() {
            SysUserRole::delete_many()
                .filter(
                    sys_user_role::Column::RoleId
                        .eq(&role_id)
                        .and(sys_user_role::Column::UserId.is_in(user_ids_to_delete))
                )
                .exec(&txn).await
                .map_err(AppError::from)?;
        }

        txn.commit().await.map_err(AppError::from)?;

        Ok(())
    }
}
