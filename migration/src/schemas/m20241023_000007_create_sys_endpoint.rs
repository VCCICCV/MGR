use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysEndpoint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysEndpoint::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysEndpoint::Path).string().not_null())
                    .col(ColumnDef::new(SysEndpoint::Method).string().not_null())
                    .col(ColumnDef::new(SysEndpoint::Action).string().not_null())
                    .col(ColumnDef::new(SysEndpoint::Resource).string().not_null())
                    .col(ColumnDef::new(SysEndpoint::Controller).string().not_null())
                    .col(ColumnDef::new(SysEndpoint::Summary).string().null())
                    .col(
                        ColumnDef::new(SysEndpoint::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SysEndpoint::UpdatedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysEndpoint::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysEndpoint {
    Table,
    Id,
    Path,
    Method,
    Action,
    Resource,
    Controller,
    Summary,
    CreatedAt,
    UpdatedAt,
}
