use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::m20240815_000001_create_enum_status::Status;



#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysOrganization::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysOrganization::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SysOrganization::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SysOrganization::Name).string().not_null())
                    .col(ColumnDef::new(SysOrganization::Description).string().null())
                    .col(ColumnDef::new(SysOrganization::Pid).string().not_null())
                    .col(
                        ColumnDef::new(SysOrganization::Status)
                            .enumeration(Alias::new("\"Status\""), Status::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOrganization::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SysOrganization::CreatedBy)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOrganization::UpdatedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(ColumnDef::new(SysOrganization::UpdatedBy).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysOrganization::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysOrganization {
    Table,
    Id,
    Code,
    Name,
    Description,
    Pid,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}
