use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysOperationLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysOperationLog::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysOperationLog::UserId).string().not_null())
                    .col(
                        ColumnDef::new(SysOperationLog::Username)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysOperationLog::Domain).string().not_null())
                    .col(
                        ColumnDef::new(SysOperationLog::ModuleName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::RequestId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysOperationLog::Method).string().not_null())
                    .col(ColumnDef::new(SysOperationLog::Url).string().not_null())
                    .col(ColumnDef::new(SysOperationLog::Ip).string().not_null())
                    .col(ColumnDef::new(SysOperationLog::UserAgent).string().null())
                    .col(ColumnDef::new(SysOperationLog::Params).json_binary().null())
                    .col(ColumnDef::new(SysOperationLog::Body).json_binary().null())
                    .col(
                        ColumnDef::new(SysOperationLog::Response)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::StartTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::EndTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::Duration)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SysOperationLog::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysOperationLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysOperationLog {
    Table,
    Id,
    UserId,
    Username,
    Domain,
    ModuleName,
    Description,
    RequestId,
    Method,
    Url,
    Ip,
    UserAgent,
    Params,
    Body,
    Response,
    StartTime,
    EndTime,
    Duration,
    CreatedAt,
}
