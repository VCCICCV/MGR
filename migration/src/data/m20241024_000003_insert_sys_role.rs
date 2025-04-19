use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let insert_role_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_role (id, code, name, description, pid, status, created_at, created_by, updated_at, updated_by)
            VALUES
            ('1', 'ROLE_SUPER', '超级管理员', '超级管理员', 0, 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('2', 'ROLE_ADMIN', '管理员', '管理员', 1, 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('3', 'ROLE_USER', '用户', '用户', 1, 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string(),
        );

        db.execute(insert_role_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
