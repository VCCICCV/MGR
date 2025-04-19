use async_trait::async_trait;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use server_core::web::{auth::User, error::AppError};
use server_model::admin::{
    entities::{
        prelude::{SysMenu, SysRoleMenu},
        sea_orm_active_enums::Status,
        sys_menu::{
            ActiveModel as SysMenuActiveModel, Column as SysMenuColumn, Model as SysMenuModel,
        },
        sys_role_menu::Column as SysRoleMenuColumn,
    },
    input::{CreateMenuInput, UpdateMenuInput},
    output::{MenuRoute, MenuTree, RouteMeta},
};
use server_utils::TreeBuilder;

use crate::{admin::sys_menu_error::MenuError, helper::db_helper};

#[async_trait]
pub trait TMenuService {
    async fn tree_menu(&self) -> Result<Vec<MenuTree>, AppError>;

    async fn get_menu_list(&self) -> Result<Vec<MenuTree>, AppError>;

    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, AppError>;

    async fn create_menu(
        &self,
        input: CreateMenuInput,
        user: User,
    ) -> Result<SysMenuModel, AppError>;
    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, AppError>;
    async fn update_menu(
        &self,
        input: UpdateMenuInput,
        user: User,
    ) -> Result<SysMenuModel, AppError>;
    async fn delete_menu(&self, id: i32, user: User) -> Result<(), AppError>;
    async fn get_menu_ids_by_role_id(
        &self,
        role_id: String,
        domain: String,
    ) -> Result<Vec<i32>, AppError>;
}

#[derive(Clone)]
pub struct SysMenuService;

impl SysMenuService {
    fn build_menu_tree(menu: &SysMenuModel) -> MenuTree {
        MenuTree {
            id: menu.id,
            pid: menu.pid.clone(),
            menu_type: menu.menu_type.clone(),
            menu_name: menu.menu_name.clone(),
            icon_type: menu.icon_type.clone(),
            icon: menu.icon.clone(),
            route_name: menu.route_name.clone(),
            route_path: menu.route_path.clone(),
            component: menu.component.clone(),
            path_param: menu.path_param.clone(),
            status: menu.status.clone(),
            active_menu: menu.active_menu.clone(),
            hide_in_menu: menu.hide_in_menu,
            sequence: menu.sequence,
            i18n_key: menu.i18n_key.clone(),
            keep_alive: menu.keep_alive,
            constant: menu.constant,
            href: menu.href.clone(),
            multi_tab: menu.multi_tab,
            created_at: menu.created_at,
            created_by: menu.created_by.clone(),
            updated_at: menu.updated_at,
            updated_by: menu.updated_by.clone(),
            children: None,
        }
    }

    fn build_tree_structure(menu_trees: Vec<MenuTree>) -> Vec<MenuTree> {
        TreeBuilder::build(
            menu_trees,
            |node| node.id,
            |node| {
                if node.pid == "0" {
                    None
                } else {
                    Some(node.pid.parse::<i32>().unwrap_or(-1))
                }
            },
            |node| node.sequence,
            |node, children| node.children = Some(children),
        )
    }

    async fn check_menu_exists(&self, id: Option<i32>, route_name: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let route_name_exists = SysMenu::find()
            .filter(SysMenuColumn::RouteName.eq(route_name))
            .filter(SysMenuColumn::Id.ne(id.unwrap_or(-1)))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .is_some();

        if route_name_exists {
            return Err(MenuError::DuplicateRouteName.into());
        }

        Ok(())
    }
}

#[async_trait]
impl TMenuService for SysMenuService {
    async fn tree_menu(&self) -> Result<Vec<MenuTree>, AppError> {
        let db = db_helper::get_db_connection().await?;

        let menus = SysMenu::find()
            .filter(SysMenuColumn::Constant.eq(false))
            .filter(SysMenuColumn::Status.eq(Status::ENABLED))
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let menu_trees: Vec<MenuTree> = menus.iter().map(Self::build_menu_tree).collect();
        Ok(Self::build_tree_structure(menu_trees))
    }

    async fn get_menu_list(&self) -> Result<Vec<MenuTree>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let menus = SysMenu::find()
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let menu_trees: Vec<MenuTree> = menus.iter().map(Self::build_menu_tree).collect();
        Ok(Self::build_tree_structure(menu_trees))
    }

    async fn get_constant_routes(&self) -> Result<Vec<MenuRoute>, AppError> {
        let db = db_helper::get_db_connection().await?;

        let menus: Vec<SysMenuModel> = SysMenu::find()
            .filter(SysMenuColumn::Constant.eq(true))
            .filter(SysMenuColumn::Status.eq(Status::ENABLED))
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let result = menus
            .into_iter()
            .map(|menu| MenuRoute {
                id: menu.id,
                pid: menu.pid,
                name: menu.route_name,
                path: menu.route_path,
                component: menu.component,
                meta: RouteMeta {
                    title: menu.menu_name,
                    i18n_key: menu.i18n_key,
                    keep_alive: menu.keep_alive,
                    constant: menu.constant,
                    icon: menu.icon,
                    order: menu.sequence,
                    href: menu.href,
                    hide_in_menu: menu.hide_in_menu,
                    active_menu: menu.active_menu,
                    multi_tab: menu.multi_tab,
                },
                children: vec![].into(),
            })
            .collect();

        Ok(result)
    }

    async fn create_menu(
        &self,
        input: CreateMenuInput,
        user: User,
    ) -> Result<SysMenuModel, AppError> {
        self.check_menu_exists(None, &input.route_name).await?;

        let db = db_helper::get_db_connection().await?;

        let menu = SysMenuActiveModel {
            menu_type: Set(input.menu_type),
            menu_name: Set(input.menu_name),
            icon_type: Set(input.icon_type),
            icon: Set(input.icon),
            route_name: Set(input.route_name),
            route_path: Set(input.route_path),
            component: Set(input.component),
            path_param: Set(input.path_param),
            status: Set(input.status),
            active_menu: Set(input.active_menu),
            hide_in_menu: Set(input.hide_in_menu),
            pid: Set(input.pid),
            sequence: Set(input.sequence),
            i18n_key: Set(input.i18n_key),
            keep_alive: Set(input.keep_alive),
            constant: Set(input.constant),
            href: Set(input.href),
            multi_tab: Set(input.multi_tab),

            created_by: Set(user.user_id()),
            ..Default::default()
        };

        let result = menu.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(result)
    }

    async fn get_menu(&self, id: i32) -> Result<SysMenuModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| MenuError::MenuNotFound.into())
    }

    async fn update_menu(
        &self,
        input: UpdateMenuInput,
        user: User,
    ) -> Result<SysMenuModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_menu = self.get_menu(input.id).await?;

        self.check_menu_exists(Some(input.id), &input.menu.route_name)
            .await?;

        let mut menu: SysMenuActiveModel = existing_menu.into();
        menu.menu_type = Set(input.menu.menu_type);
        menu.menu_name = Set(input.menu.menu_name);
        menu.icon_type = Set(input.menu.icon_type);
        menu.icon = Set(input.menu.icon);
        menu.route_name = Set(input.menu.route_name);
        menu.route_path = Set(input.menu.route_path);
        menu.component = Set(input.menu.component);
        menu.path_param = Set(input.menu.path_param);
        menu.status = Set(input.menu.status);
        menu.active_menu = Set(input.menu.active_menu);
        menu.hide_in_menu = Set(input.menu.hide_in_menu);
        menu.pid = Set(input.menu.pid);
        menu.sequence = Set(input.menu.sequence);
        menu.i18n_key = Set(input.menu.i18n_key);
        menu.keep_alive = Set(input.menu.keep_alive);
        menu.constant = Set(input.menu.constant);
        menu.href = Set(input.menu.href);
        menu.multi_tab = Set(input.menu.multi_tab);

        menu.updated_at = Set(Some(Local::now().naive_local()));
        menu.updated_by = Set(Some(user.user_id()));

        let updated_menu = menu.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(updated_menu)
    }

    async fn delete_menu(&self, id: i32, _user: User) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        SysMenu::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn get_menu_ids_by_role_id(
        &self,
        role_id: String,
        domain: String,
    ) -> Result<Vec<i32>, AppError> {
        let db = db_helper::get_db_connection().await?;

        let role_menus = SysRoleMenu::find()
            .filter(
                Condition::all()
                    .add(SysRoleMenuColumn::RoleId.eq(role_id))
                    .add(SysRoleMenuColumn::Domain.eq(domain)),
            )
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let menu_ids: Vec<i32> = role_menus.iter().map(|rm| rm.menu_id).collect();

        if menu_ids.is_empty() {
            return Ok(vec![]);
        }

        let menus = SysMenu::find()
            .filter(
                Condition::all()
                    .add(SysMenuColumn::Id.is_in(menu_ids))
                    .add(SysMenuColumn::Status.eq(Status::ENABLED))
                    .add(SysMenuColumn::Constant.eq(false)),
            )
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        Ok(menus.iter().map(|menu| menu.id).collect())
    }
}
