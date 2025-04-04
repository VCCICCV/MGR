//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product_sku")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub category_id: Option<i64>,
    pub brand_id: String,
    pub product_id: String,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub lock_stock: Option<i32>,
    pub pic: Option<String>,
    pub attribute: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub is_deleted: Option<i16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
