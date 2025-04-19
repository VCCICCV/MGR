use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
// 商品表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 product_attribute 表
        manager.create_table(
            Table::create()
                .table(ProductAttribute::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductAttribute::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .unique_key()
                        .comment("自增 ID")
                )
                .col(
                    ColumnDef::new(ProductAttribute::ProductAttributeCategoryId)
                        .big_integer()
                        .comment("商品属性分类 ID")
                )
                .col(ColumnDef::new(ProductAttribute::Name).string().comment("名称"))
                .col(
                    ColumnDef::new(ProductAttribute::OptionStatus)
                        .tiny_integer()
                        .comment("可选状态 0：手动录入 1：代入可选集合")
                )
                .col(ColumnDef::new(ProductAttribute::OptionList).string().comment("可选值集合"))
                .col(ColumnDef::new(ProductAttribute::Sort).integer().comment("排序"))
                .col(
                    ColumnDef::new(ProductAttribute::Type)
                        .tiny_integer()
                        .comment("类型 0：规则 1：参数")
                )
                .col(ColumnDef::new(ProductAttribute::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductAttribute::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductAttribute::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        // 创建 product_attribute_category 表
        manager.create_table(
            Table::create()
                .table(ProductAttributeCategory::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductAttributeCategory::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(ColumnDef::new(ProductAttributeCategory::Name).string().comment("分类名称"))
                .col(
                    ColumnDef::new(ProductAttributeCategory::CreateTime)
                        .timestamp()
                        .comment("创建时间")
                )
                .col(
                    ColumnDef::new(ProductAttributeCategory::UpdateTime)
                        .timestamp()
                        .comment("修改时间")
                )
                .col(
                    ColumnDef::new(ProductAttributeCategory::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;

        // 创建 product_attribute_value 表
        manager.create_table(
            Table::create()
                .table(ProductAttributeValue::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductAttributeValue::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(ColumnDef::new(ProductAttributeValue::ProductId).uuid().comment("商品 ID"))
                .col(
                    ColumnDef::new(ProductAttributeValue::ProductAttributeId)
                        .big_integer()
                        .comment("商品属性 ID")
                )
                .col(
                    ColumnDef::new(ProductAttributeValue::AttributeValue)
                        .string()
                        .comment("商品属性值")
                )
                .col(
                    ColumnDef::new(ProductAttributeValue::CreateTime)
                        .timestamp()
                        .comment("创建时间")
                )
                .col(
                    ColumnDef::new(ProductAttributeValue::UpdateTime)
                        .timestamp()
                        .comment("修改时间")
                )
                .col(
                    ColumnDef::new(ProductAttributeValue::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        // 创建 product_brand 表
        manager.create_table(
            Table::create()
                .table(ProductBrand::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductBrand::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(ColumnDef::new(ProductBrand::Name).string().comment("品牌名称"))
                .col(ColumnDef::new(ProductBrand::Desc).string().comment("品牌介绍"))
                .col(ColumnDef::new(ProductBrand::Pic).string().comment("品牌图"))
                .col(ColumnDef::new(ProductBrand::Sort).integer().comment("排序"))
                .col(ColumnDef::new(ProductBrand::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductBrand::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductBrand::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        // 创建 product_category 表
        manager.create_table(
            Table::create()
                .table(ProductCategory::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductCategory::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(ColumnDef::new(ProductCategory::Name).string().comment("分类名称"))
                .col(ColumnDef::new(ProductCategory::ParentId).big_integer().comment("父级 ID"))
                .col(ColumnDef::new(ProductCategory::Level).tiny_integer().comment("层级"))
                .col(ColumnDef::new(ProductCategory::IconUrl).string().comment("图标 URL"))
                .col(ColumnDef::new(ProductCategory::Sort).integer().comment("排序"))
                .col(ColumnDef::new(ProductCategory::Url).string().comment("跳转地址"))
                .col(
                    ColumnDef::new(ProductCategory::Status)
                        .tiny_integer()
                        .comment("状态 0：展示 1：隐藏")
                )
                .col(ColumnDef::new(ProductCategory::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductCategory::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductCategory::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        // 创建 product_sku

        manager.create_table(
            Table::create()
                .table(ProductComment::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductComment::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("自增ID")
                )
                .col(
                    ColumnDef::new(ProductComment::ParentId)
                        .big_integer()
                        .default(0)
                        .comment("上级 ID，一级评论为 0")
                )
                .col(ColumnDef::new(ProductComment::ProductId).uuid().not_null().comment("商品 ID"))
                .col(
                    ColumnDef::new(ProductComment::ProductSkuId)
                        .uuid()
                        .not_null()
                        .comment("商品 SKU ID")
                )
                .col(ColumnDef::new(ProductComment::UserId).uuid().not_null().comment("用户 ID"))
                .col(
                    ColumnDef::new(ProductComment::LikeCount).integer().default(0).comment("点赞数")
                )
                .col(
                    ColumnDef::new(ProductComment::ReplyCount)
                        .integer()
                        .default(0)
                        .comment("回复数")
                )
                .col(ColumnDef::new(ProductComment::Star).integer().comment("评分"))
                .col(ColumnDef::new(ProductComment::Content).string().comment("评论"))
                .col(
                    ColumnDef::new(ProductComment::CommentFlag)
                        .tiny_integer()
                        .comment("回复标识 0：用户 1：店家")
                )
                .col(
                    ColumnDef::new(ProductComment::HideFlag)
                        .tiny_integer()
                        .comment("匿名标识 0：匿名 1：不匿名")
                )
                .col(
                    ColumnDef::new(ProductComment::AppendFlag)
                        .tiny_integer()
                        .default(0)
                        .comment("追加标识 0：否 1：是")
                )
                .col(
                    ColumnDef::new(ProductComment::Resource)
                        .string()
                        .comment("评论图片/视频，JSON 格式")
                )
                .col(ColumnDef::new(ProductComment::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductComment::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductComment::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        manager.create_table(
            Table::create()
                .table(ProductSku::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductSku::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(ColumnDef::new(ProductSku::CategoryId).big_integer().comment("商品类型 ID"))
                .col(
                    ColumnDef::new(ProductSku::BrandId)
                        .string_len(36)
                        .not_null()
                        .comment("商品品牌 ID")
                )
                .col(
                    ColumnDef::new(ProductSku::ProductId)
                        .string_len(36)
                        .not_null()
                        .comment("商品 ID")
                )
                .col(ColumnDef::new(ProductSku::Price).decimal().comment("价格"))
                .col(ColumnDef::new(ProductSku::Stock).integer().default(0).comment("库存"))
                .col(ColumnDef::new(ProductSku::LockStock).integer().default(0).comment("锁定库存"))
                .col(ColumnDef::new(ProductSku::Pic).string().comment("图片"))
                .col(ColumnDef::new(ProductSku::Attribute).string().comment("属性，JSON 格式"))
                .col(ColumnDef::new(ProductSku::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductSku::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductSku::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;
        // 创建 product_spu
        manager.create_table(
            Table::create()
                .table(ProductSpu::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductSpu::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("ID")
                )
                .col(
                    ColumnDef::new(ProductSpu::CategoryId)
                        .big_integer()
                        .not_null()
                        .comment("商品类型 ID")
                )
                .col(
                    ColumnDef::new(ProductSpu::BrandId)
                        .string_len(36)
                        .not_null()
                        .comment("商品品牌 ID")
                )
                .col(ColumnDef::new(ProductSpu::Name).string().comment("商品名称"))
                .col(ColumnDef::new(ProductSpu::ProductSn).string().comment("商品编码"))
                .col(ColumnDef::new(ProductSpu::Pic).string().comment("商品主图"))
                .col(ColumnDef::new(ProductSpu::PhotoAlbum).text().comment("商品图集"))
                .col(ColumnDef::new(ProductSpu::Price).decimal().comment("商品价格"))
                .col(ColumnDef::new(ProductSpu::PromotionPrice).decimal().comment("促销价格"))
                .col(
                    ColumnDef::new(ProductSpu::PromotionStartTime)
                        .timestamp()
                        .comment("促销开始时间")
                )
                .col(
                    ColumnDef::new(ProductSpu::PromotionEndTime).timestamp().comment("促销结束时间")
                )
                .col(ColumnDef::new(ProductSpu::SubTitle).string().comment("副标题"))
                .col(ColumnDef::new(ProductSpu::Sales).integer().default(0).comment("销量"))
                .col(ColumnDef::new(ProductSpu::Unit).string().comment("单位"))
                .col(ColumnDef::new(ProductSpu::Detail).text().comment("商品详情"))
                .col(
                    ColumnDef::new(ProductSpu::PublishStatus)
                        .tiny_integer()
                        .comment("发布状态 0：发布 1：未发布")
                )
                .col(
                    ColumnDef::new(ProductSpu::NewStatus)
                        .tiny_integer()
                        .comment("新品状态 0：新品 1：非新品")
                )
                .col(
                    ColumnDef::new(ProductSpu::RecommandStatus)
                        .tiny_integer()
                        .comment("推荐状态 0：推荐 1：非推荐")
                )
                .col(ColumnDef::new(ProductSpu::CreateTime).timestamp().comment("创建时间"))
                .col(ColumnDef::new(ProductSpu::UpdateTime).timestamp().comment("修改时间"))
                .col(
                    ColumnDef::new(ProductSpu::IsDeleted)
                        .tiny_integer()
                        .comment("删除标记 0：未删除 1：删除")
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除 product_attribute 表
        manager.drop_table(Table::drop().table(ProductAttribute::Table).to_owned()).await?;

        // 删除 product_attribute_category 表
        manager.drop_table(Table::drop().table(ProductAttributeCategory::Table).to_owned()).await?;

        // 删除 product_attribute_value 表
        manager.drop_table(Table::drop().table(ProductAttributeValue::Table).to_owned()).await?;

        // 删除 product_brand 表
        manager.drop_table(Table::drop().table(ProductBrand::Table).to_owned()).await?;

        // 删除 product_category 表
        manager.drop_table(Table::drop().table(ProductCategory::Table).to_owned()).await?;

        // 删除 product_comment表
        manager.drop_table(Table::drop().table(ProductComment::Table).to_owned()).await?;
        // 删除 product_sku_
        manager.drop_table(Table::drop().table(ProductSku::Table).to_owned()).await?;
        // 删除 product_spu 表
        manager.drop_table(Table::drop().table(ProductSpu::Table).to_owned()).await?;
        Ok(())
    }
}
// 产品属性枚举
#[derive(DeriveIden)]
enum ProductAttribute {
    Table,
    Id,
    ProductAttributeCategoryId,
    Name,
    OptionStatus,
    OptionList,
    Sort,
    Type,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
// 产品属性分类枚举
#[derive(DeriveIden)]
enum ProductAttributeCategory {
    Table,
    Id,
    Name,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
// 产品属性值枚举
#[derive(DeriveIden)]
enum ProductAttributeValue {
    Table,
    Id,
    ProductId,
    ProductAttributeId,
    AttributeValue,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
// 产品品牌枚举
#[derive(DeriveIden)]
enum ProductBrand {
    Table,
    Id,
    Name,
    Desc,
    Pic,
    Sort,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
// 产品分类枚举
#[derive(DeriveIden)]
enum ProductCategory {
    Table,
    Id,
    Name,
    ParentId,
    Level,
    IconUrl,
    Sort,
    Url,
    Status,
    CreateTime,
    UpdateTime,
    IsDeleted,
}

// 产品评论枚举
#[derive(DeriveIden)]
enum ProductComment {
    Table,
    Id,
    ParentId,
    ProductId,
    ProductSkuId,
    UserId,
    LikeCount,
    ReplyCount,
    Star,
    Content,
    CommentFlag,
    HideFlag,
    AppendFlag,
    Resource,
    CreateTime,
    UpdateTime,
    IsDeleted,
}

// 产品库存枚举
#[derive(DeriveIden)]
enum ProductSku {
    Table,
    Id,
    CategoryId,
    BrandId,
    ProductId,
    Price,
    Stock,
    LockStock,
    Pic,
    Attribute,
    CreateTime,
    UpdateTime,
    IsDeleted,
}

// 产品枚举
#[derive(DeriveIden)]
enum ProductSpu {
    Table,
    Id,
    CategoryId,
    BrandId,
    Name,
    ProductSn,
    Pic,
    PhotoAlbum,
    Price,
    PromotionPrice,
    PromotionStartTime,
    PromotionEndTime,
    SubTitle,
    Sales,
    Unit,
    Detail,
    PublishStatus,
    NewStatus,
    RecommandStatus,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
