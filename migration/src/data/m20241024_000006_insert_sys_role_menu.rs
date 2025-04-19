use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let insert_role_menu_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_role_menu (role_id, menu_id, domain)
            VALUES
            ('1', '50', 'built-in'),
            ('1', '54', 'built-in'),
            ('1', '62', 'built-in'),
            ('1', '63', 'built-in'),
            ('1', '64', 'built-in'),
            ('1', '65', 'built-in'),
            ('3', '50', 'built-in'),
            ('2', '50', 'built-in'),
            ('2', '62', 'built-in'),
            ('1', '51', 'built-in'),
            ('1', '52', 'built-in'),
            ('1', '71', 'built-in'),
            ('1', '72', 'built-in')
            "#
            .to_string(),
        );

        db.execute(insert_role_menu_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
