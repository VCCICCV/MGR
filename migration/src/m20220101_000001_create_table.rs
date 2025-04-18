use sea_orm_migration::prelude::*;
use crate::sea_orm::Statement;
use chrono::Local;
#[derive(DeriveMigrationName)]

// 用户表
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建用户表
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .unique_key()
                        .not_null()
                        .big_integer()
                        .auto_increment()
                        .comment("自增ID")
                )
                .col(ColumnDef::new(User::UserId).uuid().not_null().primary_key().comment("用户ID"))
                .col(ColumnDef::new(User::Username).string().not_null().comment("用户名"))
                .col(ColumnDef::new(User::Email).string().not_null().unique_key().comment("邮箱"))
                .col(ColumnDef::new(User::Password).string().not_null().comment("密码"))
                .col(ColumnDef::new(User::Domain).string().not_null().comment("头像地址"))
                .col(ColumnDef::new(User::Avatar).string().comment("头像地址"))
                .col(
                    ColumnDef::new(User::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标记；0-未删除；1-已删除")
                )
                .col(
                    ColumnDef::new(User::Is2fa)
                        .tiny_integer()
                        .not_null()
                        .comment("是否2FA；0-未验证；1-已验证")
                )
                .col(ColumnDef::new(User::CreateTime).date_time().not_null().comment("创建时间"))
                .col(ColumnDef::new(User::UpdateTime).date_time().comment("更新时间"))
                .to_owned()
        ).await?;
        // 创建地址表
        manager.create_table(
            Table::create()
                .table(ReceiveAddress::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ReceiveAddress::Id)
                        .unique_key()
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("自增ID")
                )
                .col(ColumnDef::new(ReceiveAddress::UserId).uuid().not_null().comment("用户ID"))
                .col(
                    ColumnDef::new(ReceiveAddress::IsDefault)
                        .tiny_integer()
                        .not_null()
                        .comment("是否默认地址；0-否；1-是")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveName)
                        .string()
                        .not_null()
                        .comment("收货人姓名")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceivePhone)
                        .string()
                        .not_null()
                        .comment("收货人电话")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveProvince)
                        .string()
                        .not_null()
                        .comment("收货省份")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveCity)
                        .string()
                        .not_null()
                        .comment("收货城市")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveRegion)
                        .string()
                        .not_null()
                        .comment("收货区域")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::ReceiveDetailAddress)
                        .string()
                        .not_null()
                        .comment("收货详细地址")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标记；0-未删除；1-已删除")
                )
                .col(
                    ColumnDef::new(ReceiveAddress::CreateTime)
                        .date_time()
                        .not_null()
                        .comment("创建时间")
                )
                .col(ColumnDef::new(ReceiveAddress::UpdateTime).date_time().comment("更新时间"))
                .to_owned()
        ).await?;
        // 创建角色表
        manager.create_table(
            Table::create()
                .table(Role::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Role::RoleId)
                        .integer()
                        .not_null()
                        .primary_key()
                        .comment("角色ID")
                )
                .col(ColumnDef::new(Role::RoleName).string().not_null().comment("角色名称"))
                .col(ColumnDef::new(Role::RoleDescription).string().comment("角色描述"))
                .col(ColumnDef::new(Role::CreateTime).date_time().not_null().comment("创建时间"))
                .col(ColumnDef::new(Role::UpdateTime).date_time().comment("更新时间"))
                .to_owned()
        ).await?;
        // 创建权限表
        manager.create_table(
            Table::create()
                .table(Permission::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Permission::PermissionId)
                        .integer()
                        .not_null()
                        .primary_key()
                        .comment("权限ID")
                )
                .col(
                    ColumnDef::new(Permission::PermissionName)
                        .string()
                        .not_null()
                        .comment("权限名称")
                )
                .col(ColumnDef::new(Permission::PermissionDescription).string().comment("权限描述"))
                .col(
                    ColumnDef::new(Permission::PermissionType)
                        .integer()
                        .not_null()
                        .comment("权限类型")
                )
                .to_owned()
        ).await?;
        // 创建用户角色关联表
        manager.create_table(
            Table::create()
                .table(UserRole::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserRole::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("自增ID")
                )
                .col(ColumnDef::new(UserRole::UserId).uuid().not_null().comment("用户ID"))
                .col(ColumnDef::new(UserRole::RoleId).integer().not_null().comment("角色ID"))
                .col(ColumnDef::new(UserRole::CreateBy).uuid().not_null().comment("创建人ID"))
                .col(ColumnDef::new(UserRole::UpdateBy).uuid().not_null().comment("更新人ID"))
                .col(
                    ColumnDef::new(UserRole::CreateTime).date_time().not_null().comment("创建时间")
                )
                .col(
                    ColumnDef::new(UserRole::UpdateTime).date_time().not_null().comment("更新时间")
                )
                .col(
                    ColumnDef::new(UserRole::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标记")
                )
                .to_owned()
        ).await?;
        // 创建角色权限关联表
        manager.create_table(
            Table::create()
                .table(RolePermission::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(RolePermission::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .comment("自增ID")
                )
                .col(
                    ColumnDef::new(RolePermission::RoleId)
                        .big_integer()
                        .not_null()
                        .comment("角色ID")
                )
                .col(
                    ColumnDef::new(RolePermission::PermissionId)
                        .big_integer()
                        .not_null()
                        .comment("权限ID")
                )
                .col(ColumnDef::new(RolePermission::CreateBy).uuid().not_null().comment("创建人ID"))
                .col(ColumnDef::new(RolePermission::UpdateBy).uuid().not_null().comment("更新人ID"))
                .col(
                    ColumnDef::new(RolePermission::IsDeleted)
                        .tiny_integer()
                        .not_null()
                        .comment("删除标记")
                )
                .col(
                    ColumnDef::new(RolePermission::CreateTime)
                        .date_time()
                        .not_null()
                        .comment("创建时间")
                )
                .col(
                    ColumnDef::new(RolePermission::UpdateTime)
                        .date_time()
                        .not_null()
                        .comment("更新时间")
                )
                .to_owned()
        ).await?;
        // 插入默认角色
        let db = manager.get_connection();
        let now = Local::now().naive_local();

        let roles = vec![
            (1, "Admin", "系统管理员", now, Some(now)),
            (2, "User", "普通用户", now, Some(now)),
            (3, "System", "系统服务", now, Some(now))
        ];

        for (id, name, desc, create_time, update_time) in roles {
            let sql = format!(
                "INSERT INTO {} ({}, {}, {}, {}, {}) VALUES ($1, $2, $3, $4, $5)",
                Role::Table.to_string(),
                Role::RoleId.to_string(),
                Role::RoleName.to_string(),
                Role::RoleDescription.to_string(),
                Role::CreateTime.to_string(),
                Role::UpdateTime.to_string()
            );

            db.execute(
                Statement::from_sql_and_values(manager.get_database_backend(), &sql, [
                    id.into(),
                    name.into(),
                    desc.into(),
                    create_time.into(),
                    update_time.into(),
                ])
            ).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除用户表
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        // 删除地址表
        manager.drop_table(Table::drop().table(ReceiveAddress::Table).to_owned()).await?;
        // 删除角色表
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await?;
        // 删除权限表
        manager.drop_table(Table::drop().table(Permission::Table).to_owned()).await?;
        // 删除用户角色关联表
        manager.drop_table(Table::drop().table(UserRole::Table).to_owned()).await?;
        // 删除用户角色关联表
        manager.drop_table(Table::drop().table(RolePermission::Table).to_owned()).await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserId,
    Username,
    Email,
    Password,
    Domain,
    Avatar,
    IsDeleted,
    Is2fa,
    CreateTime,
    UpdateTime,
}
#[derive(DeriveIden)]
enum ReceiveAddress {
    Table,
    Id,
    UserId,
    IsDefault,
    ReceiveName,
    ReceivePhone,
    ReceiveProvince,
    ReceiveCity,
    ReceiveRegion,
    ReceiveDetailAddress,
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
    CreateTime,
    UpdateTime,
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
    Id,
    UserId,
    RoleId,
    CreateBy,
    UpdateBy,
    CreateTime,
    UpdateTime,
    IsDeleted,
}
#[derive(DeriveIden)]
enum RolePermission {
    Table,
    Id,
    RoleId,
    PermissionId,
    CreateBy,
    UpdateBy,
    IsDeleted,
    CreateTime,
    UpdateTime,
}
