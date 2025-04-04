use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    // 加载.env 环境配置文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().expect(".env file not found");
    cli::run_cli(migration::Migrator).await;
}
