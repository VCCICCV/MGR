use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let insert_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_domain (id, code, name, description, status, created_at, created_by, updated_at, updated_by)
            VALUES ('1', 'built-in', 'built-in', '内置域,请勿进行任何操作', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string(),
        );

        db.execute(insert_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // 当表被删除时，这些数据自然会被删除，所以这里不需要额外的操作

        // 如果需要回滚，可以在这里删除插入的默认数据
        // let db = manager.get_connection();

        // let delete_stmt = Statement::from_string(
        //     manager.get_database_backend(),
        //     "DELETE FROM sys_domain WHERE id = '1'".to_string(),
        // );

        // db.execute(delete_stmt).await?;
        Ok(())
    }
}
