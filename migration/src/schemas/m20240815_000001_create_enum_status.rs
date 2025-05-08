use sea_orm::EnumIter;
use sea_orm_migration::{
    prelude::{ sea_query::extension::postgres::Type, * },
    sea_orm::{ ConnectionTrait, DbBackend },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                // Create Status enum
                manager.create_type(
                    Type::create()
                        .as_enum(Alias::new("status"))
                        .values([Status::ENABLED, Status::DISABLED, Status::BANNED])
                        .to_owned()
                ).await?;

                // Create MenuType enum
                manager.create_type(
                    Type::create()
                        .as_enum(Alias::new("menu_type"))
                        .values([MenuType::DIRECTORY, MenuType::MENU])
                        .to_owned()
                ).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        match db.get_database_backend() {
            DbBackend::Postgres => {
                // Drop Status enum
                manager.drop_type(Type::drop().name(Alias::new("status")).to_owned()).await?;
                // Drop MenuType enum
                manager.drop_type(Type::drop().name(Alias::new("menu_type")).to_owned()).await?;
            }
            DbBackend::MySql | DbBackend::Sqlite => {}
        }
        Ok(())
    }
}
// 状态
#[derive(DeriveIden, EnumIter)]
pub enum Status {
    #[sea_orm(iden = "Status")]
    Enum,
    // 启用
    #[sea_orm(iden = "ENABLED")]
    ENABLED,
    // 禁用
    #[sea_orm(iden = "DISABLED")]
    DISABLED,
    // 禁止
    #[sea_orm(iden = "BANNED")]
    BANNED,
}
// 菜单类型
#[derive(DeriveIden, EnumIter)]
pub enum MenuType {
    #[sea_orm(iden = "MenuType")]
    Enum,
    // 路径
    #[sea_orm(iden = "directory")]
    DIRECTORY,
    // 菜单
    #[sea_orm(iden = "menu")]
    MENU,
}
