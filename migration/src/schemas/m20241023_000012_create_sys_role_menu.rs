use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysRoleMenu::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SysRoleMenu::RoleId).string().not_null())
                    .col(ColumnDef::new(SysRoleMenu::MenuId).integer().not_null())
                    .col(ColumnDef::new(SysRoleMenu::Domain).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(SysRoleMenu::RoleId)
                            .col(SysRoleMenu::MenuId)
                            .col(SysRoleMenu::Domain),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysRoleMenu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysRoleMenu {
    Table,
    RoleId,
    MenuId,
    Domain,
}
