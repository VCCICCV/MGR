use sea_orm_migration::{ prelude::*, sea_orm::Statement };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 内置域
        let insert_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_domain (id, code, name, description, status, created_at, created_by, updated_at, updated_by)
            VALUES ('1', 'built-in', 'built-in', '内置域,请勿进行任何操作', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string()
        );
        // 外部域
        let insert_stmt2 = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_domain (id, code, name, description, status, created_at, created_by, updated_at, updated_by)
            VALUES ('2', 'built-out', 'built-out', '外部域,请勿进行任何操作', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string()
        );
        // 插入商品域
        let insert_product = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_domain (id, code, name, description, status, created_at, created_by, updated_at, updated_by)
            VALUES ('3', 'product', '商品域', '管理商品相关数据和权限', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string()
        );

        // 插入订单域
        let insert_order = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_domain (id, code, name, description, status, created_at, created_by, updated_at, updated_by)
            VALUES ('4', 'order', '订单域', '管理订单相关数据和权限', 'ENABLED', '2024-05-15 00:00:00.000', '-1', NULL, NULL)
            "#.to_string()
        );

        db.execute(insert_stmt).await?;
        db.execute(insert_stmt2).await?;
        db.execute(insert_product).await?;
        db.execute(insert_order).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
