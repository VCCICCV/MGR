use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
// 车辆信息表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建订单表
        manager.create_table(
            Table::create()
                .table(Events::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Events::Id)
                        .big_integer()
                        .not_null()
                        .unique_key()
                        .auto_increment()
                        .primary_key()
                        .comment("事件id")
                )
                .col(ColumnDef::new(Events::Source).text().not_null().comment("事件源"))
                .col(ColumnDef::new(Events::Payload).text().not_null().comment("载荷"))
                .col(ColumnDef::new(Events::Status).text().not_null().comment("事件状态"))
                .col(ColumnDef::new(Events::Type).text().not_null().comment("事件类型"))
                .col(ColumnDef::new(Events::Version).integer().not_null().comment("事件版本"))
               .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除订单表
        manager.drop_table(Table::drop().table(Events::Table).to_owned()).await?;
        Ok(())
    }
}
// 事件表
#[derive(DeriveIden)]
enum Events {
    Table,
    // 事件id
    Id,
    // 事件源
    Source,
    // 载荷、元数据
    Payload,
    // 事件状态（已处理、失败、重试中等）
    Status,
    // 事件类型
    Type,
    // 事件版本
    Version,
}