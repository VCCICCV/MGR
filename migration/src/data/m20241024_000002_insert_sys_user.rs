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
            INSERT INTO sys_user (id, username, password, domain, built_in, avatar, email, phone_number, nick_name, status, created_at, created_by, updated_at, updated_by)
            VALUES
            ('1', 'Mgr', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '111@gmail.com', '18511111111', 'Soybean', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('2', 'Administrator', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '222@gmail.com', '18522222222', 'Admin', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('3', 'GeneralUser', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '333@gmail.com', '18533333333', 'User', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string(),
        );

        db.execute(insert_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
