use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
// 购物车表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建订单表
        manager.create_table(
            Table::create()
                .table(CartItem::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(CartItem::Id)
                        .big_integer()
                        .not_null()
                        .unique_key()
                        .auto_increment()
                        .primary_key()
                        .comment("自增id")
                )
                .col(
                    ColumnDef::new(CartItem::ProductSpuId)
                        .uuid().not_null()
                        .comment("商品SPU ID")
                )
                .col(
                    ColumnDef::new(CartItem::ProductSkuId)
                        .uuid().not_null()
                        .comment("商品SKU ID")
                )
                .col(ColumnDef::new(CartItem::UserId).uuid().not_null().comment("用户ID"))
                .col(
                    ColumnDef::new(CartItem::ProductPicture).string().not_null().comment("商品图片")
                )
                .col(ColumnDef::new(CartItem::ProductName).string().not_null().comment("商品名称"))
                .col(ColumnDef::new(CartItem::ProductBrand).string().not_null().comment("商品品牌"))
                .col(
                    ColumnDef::new(CartItem::ProductPrice).decimal().not_null().comment("商品价格")
                )
                .col(
                    ColumnDef::new(CartItem::ProductQuantity)
                        .integer()
                        .not_null()
                        .comment("加购物车数量")
                )
                .col(
                    ColumnDef::new(CartItem::ProductAttribute)
                        .string()
                        .not_null()
                        .comment("商品规格；JSON格式")
                )
                .col(
                    ColumnDef::new(CartItem::Selected)
                        .tiny_integer()
                        .not_null()
                        .comment("选中标志；1-选中；0-未选中")
                )
                .col(
                    ColumnDef::new(CartItem::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标志；1-已删除；0-未删除")
                )
                .col(
                    ColumnDef::new(CartItem::CreateTime).date_time().not_null().comment("创建时间")
                )
                .col(ColumnDef::new(CartItem::UpdateTime).date_time().comment("更新时间"))
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除产品表
        manager.drop_table(Table::drop().table(CartItem::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum CartItem {
    Table,
    Id,
    ProductSpuId,
    ProductSkuId,
    UserId,
    ProductPicture,
    ProductName,
    ProductBrand,
    ProductPrice,
    ProductQuantity,
    ProductAttribute,
    Selected,
    IsDeleted,
    CreateTime,
    UpdateTime,
}
