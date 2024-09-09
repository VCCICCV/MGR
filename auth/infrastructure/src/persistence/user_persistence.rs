use anyhow::anyhow;
use config::db_config::get_db_connection;
use sea_orm::{entity::*, query::*, DbErr, sea_query::Expr, DatabaseConnection};
use crate::{ config, entities };
pub struct UserPersistence;
use application::common::res::{ListData, PageParams};
use application::service::user_service_trait::UserServiceTrait;

impl UserServiceTrait for UserPersistence {
    /// get_user_list 获取用户列表
    /// page_params 分页参数
    pub async fn get_users_list(&self, page_params: PageParams, req: SysUserSearchReq) -> Result<ListData<UserWithDept>> {


        Ok(res)
    }




    // pub async fn create_user(username: &str, email: &str) -> Result<(), DbErr> {
    //     let db = get_db_connection().await;
    //     let new_user = entities::user::ActiveModel {
    //         id: Default::default(),
    //         username: Set(username.to_string()),
    //         email: Set(email.to_string()),
    //     };
    //     entities::user::Entity::insert(new_user).exec(&db).await?;
    //     Ok(())
    // }
    //
    // pub async fn get_user_by_id(id: i32) -> Result<Option<entities::user::Model>, DbErr> {
    //     let db = get_db_connection().await;
    //     let user = entities::user::Entity::find_by_id(id).one(&db).await?;
    //     Ok(user)
    // }
    //
    // pub async fn update_user(id: i32, username: &str, email: &str) -> Result<(), DbErr> {
    //     let db = get_db_connection().await;
    //     entities::user::Entity
    //         ::update_many()
    //         .col_expr(entities::user::Column::Username, Expr::value(username.to_string()))
    //         .col_expr(entities::user::Column::Email, Expr::value(email.to_string()))
    //         .filter(entities::user::Column::Id.eq(id))
    //         .exec(&db).await?;
    //     Ok(())
    // }
    //
    // pub async fn delete_user(id: i32) -> Result<(), DbErr> {
    //     let db = get_db_connection().await;
    //     entities::user::Entity::delete_by_id(id).exec(&db).await?;
    //     Ok(())
    // }
}
