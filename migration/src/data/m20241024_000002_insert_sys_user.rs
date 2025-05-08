use sea_orm_migration::{ prelude::*, sea_orm::Statement };

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
            ('1', 'MgrAdmin', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '111@gmail.com', '17671933600', 'MgrAdmin', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('2', 'Administrator', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '222@gmail.com', '18522222222', 'Admin', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('3', 'GeneralUser', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-in', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', 'ww33@gmail.com', '18533323333', 'User', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('4', 'GeneralUser1', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33w3@gmail.com', '18533343333', 'User1', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('5', 'GeneralUser2', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '333ddfdas@gmail.com', '18533533333', 'User2', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('6', 'GeneralUser3', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33dfdfd3sa@gmail.com', '18533633333', 'User3', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('7', 'GeneralUser4', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33asdfdfsa3@gmail.com', '18573333333', 'User4', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('8', 'GeneralUser5', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33sdfdfas3@gmail.com', '18535333333', 'User5', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('9', 'GeneralUser6', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '3assdfsas3@gmail.com', '18535323333', 'User6', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('10', 'GeneralUser7', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33aasfsas3@gmail.com', '17533333333', 'User7', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('11', 'GeneralUser8', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '333asdsa@gmail.com', '18533833333', 'User8', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('12', 'GeneralUser9', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '333dfdsfsd@gmail.com', '18593333333', 'User9', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('13', 'GeneralUser10', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '333sdfsd@gmail.com', '18533033333', 'User10', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL),
            ('14', 'GeneralUser11', '$argon2id$v=19$m=19456,t=2,p=1$8TC8kz2KUf0ytBWeFn5CZA$UgL+qvhpeNyijDBfL4A90KjdXOJ7tNP77RrufQhOkgg', 'built-out', true, 'https://minio.bytebytebrew.com/default/Ugly%20Avatar%20Face.png', '33sdfsdf3@gmail.com', '17536333333', 'User11', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string()
        );

        db.execute(insert_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
