use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysTokens::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysTokens::AccessToken).string().not_null())
                    .col(ColumnDef::new(SysTokens::RefreshToken).string().not_null())
                    .col(ColumnDef::new(SysTokens::Status).string().not_null())
                    .col(ColumnDef::new(SysTokens::UserId).string().not_null())
                    .col(ColumnDef::new(SysTokens::Username).string().not_null())
                    .col(ColumnDef::new(SysTokens::Domain).string().not_null())
                    .col(ColumnDef::new(SysTokens::LoginTime).timestamp().not_null())
                    .col(ColumnDef::new(SysTokens::Ip).string().not_null())
                    .col(ColumnDef::new(SysTokens::Port).integer().null())
                    .col(ColumnDef::new(SysTokens::Address).string().not_null())
                    .col(ColumnDef::new(SysTokens::UserAgent).string().not_null())
                    .col(ColumnDef::new(SysTokens::RequestId).string().not_null())
                    .col(ColumnDef::new(SysTokens::Type).string().not_null())
                    .col(
                        ColumnDef::new(SysTokens::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SysTokens::CreatedBy).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysTokens {
    Table,
    Id,
    AccessToken,
    RefreshToken,
    Status,
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
