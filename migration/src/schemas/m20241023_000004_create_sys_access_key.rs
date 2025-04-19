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
                    .table(SysAccessKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysAccessKey::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysAccessKey::Domain).string().not_null())
                    .col(
                        ColumnDef::new(SysAccessKey::AccessKeyId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SysAccessKey::AccessKeySecret)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SysAccessKey::Status)
                            .enumeration(Alias::new("\"Status\""), Status::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysAccessKey::Description).string().null())
                    .col(
                        ColumnDef::new(SysAccessKey::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SysAccessKey::CreatedBy).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysAccessKey::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysAccessKey {
    Table,
    Id,
    Domain,
    AccessKeyId,
    AccessKeySecret,
    Status,
    Description,
    CreatedAt,
    CreatedBy,
}
