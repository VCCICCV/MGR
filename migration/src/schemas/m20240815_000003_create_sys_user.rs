use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::m20240815_000001_create_enum_status::Status;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建表
        manager.create_table(
            Table::create()
                .table(SysUser::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysUser::Id).string().not_null().primary_key())
                .col(
                    ColumnDef::new(SysUser::Username)
                        .string()
                        .not_null()
                        .unique_key()
                        .comment("用户名")
                )
                .col(ColumnDef::new(SysUser::Password).string().not_null().comment("密码"))
                .col(ColumnDef::new(SysUser::Domain).string().not_null().comment("域"))
                .col(ColumnDef::new(SysUser::BuiltIn).boolean().not_null().comment("是否内置"))
                .col(ColumnDef::new(SysUser::Avatar).string().comment("头像"))
                .col(ColumnDef::new(SysUser::Email).string().unique_key().comment("邮箱"))
                .col(ColumnDef::new(SysUser::PhoneNumber).string().unique_key().comment("手机号"))
                .col(ColumnDef::new(SysUser::NickName).string().not_null().comment("昵称"))
                .col(
                    ColumnDef::new(SysUser::Status)
                        .enumeration(Alias::new("status"), Status::iter())
                        .not_null()
                        .comment("用户状态")
                )
                .col(
                    ColumnDef::new(SysUser::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                        .comment("创建时间")
                )
                .col(ColumnDef::new(SysUser::CreatedBy).string().not_null())
                .col(ColumnDef::new(SysUser::UpdatedAt).timestamp())
                .col(ColumnDef::new(SysUser::UpdatedBy).string())
                .to_owned()
        ).await?;

        // 创建索引
        manager.create_index(
            Index::create()
                .table(SysUser::Table)
                .name("idx_sys_user_domain_id")
                .col(SysUser::Domain)
                .to_owned()
        ).await?;

        // 索引
        manager.create_index(
            Index::create()
                .table(SysUser::Table)
                .name("idx_sys_user_username")
                .col(SysUser::Username)
                .unique()
                .to_owned()
        ).await?;
        // 外键
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_sys_user_domain")
                .from(SysUser::Table, SysUser::Domain)
                .to(Alias::new("sys_domain"), Alias::new("code"))
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SysUser::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum SysUser {
    Table,
    Id,
    Username,
    Password,
    Domain,
    BuiltIn,
    Avatar,
    Email,
    PhoneNumber,
    NickName,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}
