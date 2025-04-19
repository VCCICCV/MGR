use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]

// 用户表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建地址表
        manager.create_table(
            Table::create()
                .table(ReceiveAddress::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ReceiveAddress::Id)
                        .unique_key()
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("自增ID")
                )
                .col(ColumnDef::new(ReceiveAddress::UserId).uuid().not_null().comment("用户ID"))
                .col(
                    ColumnDef::new(ReceiveAddress::IsDefault)
                        .tiny_integer()
                        .not_null()
                        .comment("是否默认地址；0-否；1-是")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveName)
                        .string()
                        .not_null()
                        .comment("收货人姓名")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceivePhone)
                        .string()
                        .not_null()
                        .comment("收货人电话")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveProvince)
                        .string()
                        .not_null()
                        .comment("收货省份")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveCity)
                        .string()
                        .not_null()
                        .comment("收货城市")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveRegion)
                        .string()
                        .not_null()
                        .comment("收货区域")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveDetailAddress)
                        .string()
                        .not_null()
                        .comment("收货详细地址")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标记；0-未删除；1-已删除")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::CreateTime)
                        .timestamp()
                        .not_null()
                        .comment("创建时间")
                )
                .col(ColumnDef::new(ReceiveAddress::UpdateTime).timestamp().comment("更新时间"))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除地址表
        manager.drop_table(Table::drop().table(ReceiveAddress::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum ReceiveAddress {
    Table,
    Id,
    UserId,
    IsDefault,
    ReceiveName,
    ReceivePhone,
    ReceiveProvince,
    ReceiveCity,
    ReceiveRegion,
    ReceiveDetailAddress,
    IsDeleted,
    CreateTime,
    UpdateTime,
}
