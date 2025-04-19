use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysLoginLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysLoginLog::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysLoginLog::UserId).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::Username).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::Domain).string().not_null())
                    .col(
                        ColumnDef::new(SysLoginLog::LoginTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SysLoginLog::Ip).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::Port).integer().null())
                    .col(ColumnDef::new(SysLoginLog::Address).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::UserAgent).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::RequestId).string().not_null())
                    .col(ColumnDef::new(SysLoginLog::Type).string().not_null())
                    .col(
                        ColumnDef::new(SysLoginLog::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SysLoginLog::CreatedBy).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysLoginLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysLoginLog {
    Table,
    Id,
    UserId,
    Username,
    Domain,
    LoginTime,
    Ip,
    Port,
    Address,
    UserAgent,
    RequestId,
    Type,
    CreatedAt,
    CreatedBy,
}
