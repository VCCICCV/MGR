use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysUserRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SysUserRole::UserId).string().not_null())
                    .col(ColumnDef::new(SysUserRole::RoleId).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(SysUserRole::UserId)
                            .col(SysUserRole::RoleId),
                    )
                    .to_owned(),
            )
            .await?;

        // Add foreign key constraints
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_sys_user_role_user_id")
                    .from(SysUserRole::Table, SysUserRole::UserId)
                    .to(Alias::new("sys_user"), Alias::new("id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_sys_user_role_role_id")
                    .from(SysUserRole::Table, SysUserRole::RoleId)
                    .to(Alias::new("sys_role"), Alias::new("id"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysUserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysUserRole {
    Table,
    UserId,
    RoleId,
}
