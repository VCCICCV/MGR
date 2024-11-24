use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
// 订单表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建订单表
        manager.create_table(
            Table::create()
                .table(Message::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Message::Id)
                        .big_integer()
                        .not_null()
                        .unique_key()
                        .auto_increment()
                        .primary_key()
                        .comment("自增id")
                )
                .col(ColumnDef::new(Message::Kind).string().not_null().comment("消息类型"))
                .col(ColumnDef::new(Message::Content).string().not_null().comment("消息内容"))
                .col(
                    ColumnDef::new(Message::Status)
                        .tiny_integer()
                        .not_null()
                        .comment("消息状态；0：Pending，1：Sending，2：Success，3：Failed")
                )
                .col(ColumnDef::new(Message::UserId).uuid().not_null().comment("用户id"))
                .col(ColumnDef::new(Message::CreateTime).date_time().not_null().comment("创建时间"))
                .col(ColumnDef::new(Message::UpdateTime).date_time().comment("更新时间"))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除订单表
        manager.drop_table(Table::drop().table(Message::Table).to_owned()).await?;
        Ok(())
    }
}
// 消息表
#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    Kind,
    Content,
    Status,
    UserId,
    CreateTime,
    UpdateTime,
}
