use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

use super::m20240815_000001_create_enum_status::Status;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(SysDomain::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysDomain::Id).string().not_null().primary_key())
                .col(ColumnDef::new(SysDomain::Code).string().not_null().unique_key())
                .col(ColumnDef::new(SysDomain::Name).string().not_null())
                .col(ColumnDef::new(SysDomain::Description).string().null())
                .col(
                    ColumnDef::new(SysDomain::Status)
                        .enumeration(Alias::new("status"), Status::iter())
                        .not_null()
                )
                .col(
                    ColumnDef::new(SysDomain::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                )
                .col(ColumnDef::new(SysDomain::CreatedBy).string().not_null())
                .col(ColumnDef::new(SysDomain::UpdatedAt).timestamp().null())
                .col(ColumnDef::new(SysDomain::UpdatedBy).string().null())
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SysDomain::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum SysDomain {
    Table,
    Id,
    Code,
    Name,
    Description,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}
