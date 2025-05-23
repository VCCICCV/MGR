//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product_attribute")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub product_attribute_category_id: Option<i64>,
    pub name: Option<String>,
    pub option_status: Option<i16>,
    pub option_list: Option<String>,
    pub sort: Option<i32>,
    pub r#type: Option<i16>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub is_deleted: Option<i16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
