use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let insert_user_role_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_user_role (user_id, role_id)
            VALUES
            ('1', '1'),
            ('2', '2'),
            ('3', '3')
            "#
            .to_string(),
        );

        db.execute(insert_user_role_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
