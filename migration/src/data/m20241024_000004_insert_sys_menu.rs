use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let insert_menu_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_menu (id, menu_type, menu_name, icon_type, icon, route_name, route_path, component, path_param, status, active_menu, hide_in_menu, pid, sequence, i18n_key, keep_alive, constant, href, multi_tab, created_at, created_by, updated_at, updated_by)
            VALUES
            ('1', 'menu', 'login', '1', '', 'login', '/login/:module(pwd-login|code-login|register|reset-pwd|bind-wechat)?', 'layout.blank$view.login', '', 'ENABLED', '', 'true', '0', '0', 'route.login', 'false', 'true', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('2', 'menu', '403', '1', '', '403', '/403', 'layout.blank$view.403', '', 'ENABLED', '', 'true', '0', '0', 'route.403', 'false', 'true', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('3', 'menu', '404', '1', '', '404', '/404', 'layout.blank$view.404', '', 'ENABLED', '', 'true', '0', '0', 'route.404', 'false', 'true', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('4', 'menu', '500', '1', '', '500', '/500', 'layout.blank$view.500', '', 'ENABLED', '', 'true', '0', '0', 'route.500', 'false', 'true', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('5', 'menu', 'iframe-page', '1', '', 'iframe-page', '/iframe-page/:url', 'layout.base$view.iframe-page', '', 'ENABLED', '', 'true', '0', '0', 'route.iframe-page', 'false', 'true', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('62', 'menu', 'manage_menu', '1', 'material-symbols:route', 'manage_menu', '/manage/menu', 'view.manage_menu', '', 'ENABLED', '', 'false', '54', '2', 'route.manage_menu', 'true', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('65', 'menu', 'manage_user-detail', '1', '', 'manage_user-detail', '/manage/user-detail/:id', 'view.manage_user-detail', '', 'ENABLED', 'manage_user', 'true', '54', '3', 'route.manage_user-detail', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('50', 'menu', 'home', '1', 'mdi:monitor-dashboard', 'home', '/home', 'layout.base$view.home', '', 'ENABLED', '', 'false', '0', '0', 'route.home', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('54', 'directory', 'manage', '1', 'carbon:cloud-service-management', 'manage', '/manage', 'layout.base', '', 'ENABLED', '', 'false', '0', '4', 'route.manage', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('64', 'menu', 'manage_user', '1', 'ic:round-manage-accounts', 'manage_user', '/manage/user', 'view.manage_user', '', 'ENABLED', '', 'false', '54', '0', 'route.manage_user', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('63', 'menu', 'manage_role', '1', 'carbon:user-role', 'manage_role', '/manage/role', 'view.manage_role', '', 'ENABLED', '', 'false', '54', '1', 'route.manage_role', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('71', 'menu', 'log_login', '1', 'carbon:login', 'log_login', '/log/login', 'view.log_login', '', 'ENABLED', '', 'false', '52', '0', 'route.log_login', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('72', 'menu', 'log_operation', '1', 'carbon:operations-record', 'log_operation', '/log/operation', 'view.log_operation', '', 'ENABLED', '', 'false', '52', '0', 'route.log_operation', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('52', 'directory', 'log', '1', 'carbon:cloud-logging', 'log', '/log', 'layout.base', '', 'ENABLED', '', 'false', '0', '0', 'route.log', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('51', 'menu', 'access-key', '1', 'carbon:document-signed', 'access-key', '/access-key', 'layout.base$view.access-key', '', 'ENABLED', '', 'false', '0', '0', 'route.access-key', 'false', 'false', '', 'false', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string(),
        );

        db.execute(insert_menu_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
