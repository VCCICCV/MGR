use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
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
                        .col(SysRoleMenu::Domain)
                )
                .to_owned()
        ).await?;
        // 添加外键约束：sys_role_menu.role_id -> sys_role.id
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_sys_role_menu_role_id")
                .from(SysRoleMenu::Table, SysRoleMenu::RoleId)
                .to(Alias::new("sys_role"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade) // 级联删除
                .on_update(ForeignKeyAction::Cascade) // 级联更新
                .to_owned()
        ).await?;

        // 添加外键约束：sys_role_menu.menu_id -> sys_menu.id
        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk_sys_role_menu_menu_id")
                .from(SysRoleMenu::Table, SysRoleMenu::MenuId)
                .to(Alias::new("sys_menu"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 先删除外键约束（按创建顺序反向删除）
        manager.drop_foreign_key(
            ForeignKey::drop().name("fk_sys_role_menu_menu_id").table(SysRoleMenu::Table).to_owned()
        ).await?;

        manager.drop_foreign_key(
            ForeignKey::drop().name("fk_sys_role_menu_role_id").table(SysRoleMenu::Table).to_owned()
        ).await?;

        manager.drop_table(Table::drop().table(SysRoleMenu::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum SysRoleMenu {
    Table,
    RoleId,
    MenuId,
    Domain,
}
