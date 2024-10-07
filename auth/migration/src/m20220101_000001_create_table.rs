use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建用户表
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Role).integer().not_null())
                    .col(ColumnDef::new(User::Salt).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(ColumnDef::new(User::IsDeleted).boolean().not_null())
                    .col(ColumnDef::new(User::CreateTime).date_time().not_null())
                    .col(ColumnDef::new(User::UpdateTime).date_time())
                    .to_owned(),
            )
            .await?;
        // 创建角色表
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::RoleId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::RoleName).string().not_null())
                    .col(ColumnDef::new(Role::RoleDescription).string())
                    .to_owned(),
            )
            .await?;
        // 创建权限表
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::PermissionId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::PermissionName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Permission::PermissionDescription).string())
                    .col(
                        ColumnDef::new(Permission::PermissionType)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        // 创建用户角色关联表
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRole::UserId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRole::RoleId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        // 创建角色权限关联表
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermission::RoleId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除用户表
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        // 删除角色表
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        // 删除权限表
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;
        // 删除用户角色关联表
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
    Username,
    Email,
    Password,
    Role,
    Salt,
    Avatar,
    IsDeleted,
    CreateTime,
    UpdateTime,
}
#[derive(DeriveIden)]
enum Role {
    Table,
    RoleId,
    RoleName,
    RoleDescription,
}
#[derive(DeriveIden)]
enum Permission {
    Table,
    PermissionId,
    PermissionName,
    PermissionDescription,
    PermissionType,
}
#[derive(DeriveIden)]
enum UserRole {
    Table,
    UserId,
    RoleId,
}
#[derive(DeriveIden)]
enum RolePermission {
    Table,
    RoleId,
    PermissionId,
}
