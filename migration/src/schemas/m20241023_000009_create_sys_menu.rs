use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::m20240815_000001_create_enum_status::{MenuType, Status};



#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysMenu::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SysMenu::MenuType)
                            .enumeration(Alias::new("menu_type"), MenuType::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysMenu::MenuName).string().not_null())
                    .col(ColumnDef::new(SysMenu::IconType).integer().null())
                    .col(ColumnDef::new(SysMenu::Icon).string().null())
                    .col(
                        ColumnDef::new(SysMenu::RouteName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SysMenu::RoutePath).string().not_null())
                    .col(ColumnDef::new(SysMenu::Component).string().not_null())
                    .col(ColumnDef::new(SysMenu::PathParam).string().null())
                    .col(
                        ColumnDef::new(SysMenu::Status)
                            .enumeration(Alias::new("status"), Status::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysMenu::ActiveMenu).string().null())
                    .col(ColumnDef::new(SysMenu::HideInMenu).boolean().null())
                    .col(ColumnDef::new(SysMenu::Pid).string().not_null())
                    .col(ColumnDef::new(SysMenu::Sequence).integer().not_null())
                    .col(ColumnDef::new(SysMenu::I18nKey).string().null())
                    .col(ColumnDef::new(SysMenu::KeepAlive).boolean().null())
                    .col(ColumnDef::new(SysMenu::Constant).boolean().not_null())
                    .col(ColumnDef::new(SysMenu::Href).string().null())
                    .col(ColumnDef::new(SysMenu::MultiTab).boolean().null())
                    .col(
                        ColumnDef::new(SysMenu::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SysMenu::CreatedBy).string().not_null())
                    .col(ColumnDef::new(SysMenu::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(SysMenu::UpdatedBy).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysMenu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysMenu {
    Table,
    Id,
    MenuType,
    MenuName,
    IconType,
    Icon,
    RouteName,
    RoutePath,
    Component,
    PathParam,
    Status,
    ActiveMenu,
    HideInMenu,
    Pid,
    Sequence,
    I18nKey,
    KeepAlive,
    Constant,
    Href,
    MultiTab,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}
