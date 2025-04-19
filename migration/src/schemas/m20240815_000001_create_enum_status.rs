use sea_orm::EnumIter;
use sea_orm_migration::{
    prelude::{sea_query::extension::postgres::Type, *},
    sea_orm::{ConnectionTrait, DbBackend},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {},
            DbBackend::Postgres => {
                // Create Status enum
                manager
                    .create_type(
                        Type::create()
                            .as_enum(Alias::new("Status"))
                            .values([Status::ENABLED, Status::DISABLED, Status::BANNED])
                            .to_owned(),
                    )
                    .await?;

                // Create MenuType enum
                manager
                    .create_type(
                        Type::create()
                            .as_enum(Alias::new("MenuType"))
                            .values([MenuType::DIRECTORY, MenuType::MENU])
                            .to_owned(),
                    )
                    .await?;
            },
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {},
            DbBackend::Postgres => {
                // Drop Status enum
                manager
                    .drop_type(Type::drop().name(Alias::new("Status")).to_owned())
                    .await?;
                // Drop MenuType enum
                manager
                    .drop_type(Type::drop().name(Alias::new("MenuType")).to_owned())
                    .await?;
            },
        }

        Ok(())
    }
}

#[derive(DeriveIden, EnumIter)]
pub enum Status {
    #[sea_orm(iden = "Status")]
    Enum,
    #[sea_orm(iden = "ENABLED")]
    ENABLED,
    #[sea_orm(iden = "DISABLED")]
    DISABLED,
    #[sea_orm(iden = "BANNED")]
    BANNED,
}

#[derive(DeriveIden, EnumIter)]
pub enum MenuType {
    #[sea_orm(iden = "MenuType")]
    Enum,
    #[sea_orm(iden = "directory")]
    DIRECTORY,
    #[sea_orm(iden = "menu")]
    MENU,
}
