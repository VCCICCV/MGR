use sea_orm_migration::{ prelude::*, sea_orm::Statement };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 执行架构迁移
        sea_orm_adapter::up(db).await?;

        let insert_casbin_rules_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5)
            VALUES
            ('p', 'ROLE_SUPER', 'built-in', '/domain', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/domain', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/domain/:id', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/domain', 'PUT', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/domain/:id', 'DELETE', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/route', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route/:id', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route', 'PUT', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route/:id', 'DELETE', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route/tree', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/route/auth-route/:roleId', 'GET', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/role', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/role', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/role/:id', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/role', 'PUT', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/role/:id', 'DELETE', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/user/users', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/user', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/user', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/user/:id', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/user', 'PUT', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/user/:id', 'DELETE', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/api-endpoint', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/api-endpoint/auth-api-endpoint/:roleCode', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/api-endpoint/tree', 'GET', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/access-key', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/access-key', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/access-key/:id', 'DELETE', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/login-log', 'GET', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/operation-log', 'GET', '', ''),

            ('p', 'ROLE_SUPER', 'built-in', '/authorization/assign-permission', 'POST', '', ''),
            ('p', 'ROLE_SUPER', 'built-in', '/authorization/assign-routes', 'POST', '', '')
        "#.to_string()
        );

        db.execute(insert_casbin_rules_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 清空表
        let truncate_stmt = Statement::from_string(
            manager.get_database_backend(),
            "TRUNCATE TABLE casbin_rule".to_string()
        );
        db.execute(truncate_stmt).await?;

        // 重置序列
        let reset_seq_stmt = Statement::from_string(
            manager.get_database_backend(),
            "ALTER SEQUENCE casbin_rule_id_seq RESTART WITH 1".to_string()
        );
        db.execute(reset_seq_stmt).await?;
        Ok(())
    }
}
