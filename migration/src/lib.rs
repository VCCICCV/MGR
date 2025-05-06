mod schemas;
mod data;
use data::{
    m20241023_000001_insert_sys_domain,
    m20241024_000002_insert_sys_user,
    m20241024_000005_insert_sys_user_role,
    m20241024_000006_insert_sys_role_menu,
    m20241024_000003_insert_sys_role,
    m20241024_000004_insert_sys_menu,
    m20241024_000007_insert_casbin_rule,
};
use schemas::{
    m20240815_000001_create_enum_status,
    m20240815_000003_create_sys_user,
    m20241023_000005_create_sys_role,
    m20241023_000004_create_sys_access_key,
    m20241023_000002_create_sys_domain,
    m20241023_000007_create_sys_endpoint,
    m20241023_000008_create_sys_login_log,
    m20241023_000009_create_sys_menu,
    m20241023_000010_create_sys_operation_log,
    m20241023_000011_create_sys_organization,
    m20241023_000012_create_sys_role_menu,
    m20241023_000013_create_sys_tokens,
    m20241023_000014_create_sys_user_role,
    // m20241023_000015_create_event_streams,
    m20241023_000016_create_order,
    m20241023_000017_create_receive_address,
    m20241023_000018_create_cart_item,
    m20241023_000019_create_product,
};
pub use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 架构迁移
            Box::new(m20240815_000001_create_enum_status::Migration),
            Box::new(m20241023_000002_create_sys_domain::Migration),
            Box::new(m20240815_000003_create_sys_user::Migration),
            Box::new(m20241023_000004_create_sys_access_key::Migration),
            Box::new(m20241023_000005_create_sys_role::Migration),
            Box::new(m20241023_000007_create_sys_endpoint::Migration),
            Box::new(m20241023_000008_create_sys_login_log::Migration),
            Box::new(m20241023_000009_create_sys_menu::Migration),
            Box::new(m20241023_000010_create_sys_operation_log::Migration),
            Box::new(m20241023_000011_create_sys_organization::Migration),
            Box::new(m20241023_000012_create_sys_role_menu::Migration),
            Box::new(m20241023_000013_create_sys_tokens::Migration),
            Box::new(m20241023_000014_create_sys_user_role::Migration),
            // Box::new(m20241023_000015_create_event_streams::Migration),
            Box::new(m20241023_000016_create_order::Migration),
            Box::new(m20241023_000017_create_receive_address::Migration),
            Box::new(m20241023_000018_create_cart_item::Migration),
            Box::new(m20241023_000019_create_product::Migration),
            // 数据迁移
            Box::new(m20241023_000001_insert_sys_domain::Migration),
            Box::new(m20241024_000002_insert_sys_user::Migration),
            Box::new(m20241024_000003_insert_sys_role::Migration),
            Box::new(m20241024_000004_insert_sys_menu::Migration),
            Box::new(m20241024_000005_insert_sys_user_role::Migration),
            Box::new(m20241024_000006_insert_sys_role_menu::Migration),
            Box::new(m20241024_000007_insert_casbin_rule::Migration)
        ]
    }
}
