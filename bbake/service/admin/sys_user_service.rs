use async_trait::async_trait;
use sea_orm::{ EntityTrait, PaginatorTrait, QueryOrder, QuerySelect };

use crate::{
    client::{ database::DatabaseClient, redis::RedisClient },
    model::{ admin::response::UserWithoutPassword, entity::{ prelude::SysUser, sys_user } },
    shared::{ error::AppError, res::{ Direction, PageRequest, PageResponse } },
};
#[async_trait]
pub trait SysUserService: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<UserWithoutPassword>, AppError>;
    async fn get_page(
        &self,
        req: PageRequest
    ) -> Result<PageResponse<UserWithoutPassword>, AppError>;
    async fn get_by_id(&self, id: i64) -> Result<UserWithoutPassword, AppError>;
    // async fn post(&self, user: UserWithoutPassword) -> Result<UserWithoutPassword, AppError>;
    async fn update(&self, user: UserWithoutPassword) -> Result<UserWithoutPassword, AppError>;
    async fn delete(&self, id: i64) -> Result<UserWithoutPassword, AppError>;
}
#[derive(Clone)]
pub struct SysUserServiceImpl {
    db: DatabaseClient,
    redis: RedisClient,
}
impl SysUserServiceImpl {
    pub fn new(db: DatabaseClient, redis: RedisClient) -> Self {
        Self {
            db,
            redis,
        }
    }
}
// 私有方法
impl SysUserServiceImpl {
    fn parse_sort_field(field: &str) -> sys_user::Column {
        match field {
            "id" => sys_user::Column::Id,
            "created_at" => sys_user::Column::CreatedAt,
            "username" => sys_user::Column::Username,
            _ => sys_user::Column::Id, // 默认使用ID字段
        }
    }
}
// 公共方法
#[async_trait]
impl SysUserService for SysUserServiceImpl {
    async fn get_all(&self) -> Result<Vec<UserWithoutPassword>, AppError> {
        SysUser::find()
            .all(&self.db).await
            .map(|users| users.into_iter().map(UserWithoutPassword::from).collect())
            .map_err(AppError::from)
    }
    async fn get_page(
        &self,
        req: PageRequest
    ) -> Result<PageResponse<UserWithoutPassword>, AppError> {
        let PageRequest { current, size, sort_by, direction } = req;

        // 计算分页偏移量
        let offset = current.saturating_sub(1) * size;
        let limit = size;

        // 构建基础查询
        let mut query = SysUser::find();

        // 添加排序（需要实现parse_sort_field）
        if let Some(sort_field) = sort_by {
            match direction {
                Some(Direction::ASC) => {
                    query = query.order_by_asc(SysUserServiceImpl::parse_sort_field(&sort_field));
                }
                Some(Direction::DESC) => {
                    query = query.order_by_desc(SysUserServiceImpl::parse_sort_field(&sort_field));
                }
                None => {} // 无排序方向时不处理
            }
        }

        // 创建分页器并获取数据
        let paginator = query.paginate(&self.db, limit);
        let data = paginator
            .fetch_page(offset / limit).await?
            .into_iter()
            .map(UserWithoutPassword::from)
            .collect();

        // 获取总数
        let total = paginator.num_items().await?;

        Ok(PageResponse {
            current,
            size,
            total,
            data,
        })
    }

    async fn get_by_id(&self, id: i64) -> Result<UserWithoutPassword, AppError> {
        todo!()
    }

    // async fn post(&self, user: UserWithoutPassword) -> Result<UserWithoutPassword, AppError> {
    //     todo!()
    // }

    async fn update(&self, user: UserWithoutPassword) -> Result<UserWithoutPassword, AppError> {
        todo!()
    }

    async fn delete(&self, id: i64) -> Result<UserWithoutPassword, AppError> {
        todo!()
    }
}
