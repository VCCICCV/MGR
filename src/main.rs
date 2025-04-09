mod api;
mod client;
mod cmd;
mod error;
mod configure;
mod constant;
mod middleware;
mod model;
mod repository;
mod router;
mod server;
mod service;
mod utils;
//*! # 这里是启动函数
//*!
//*! `main` 函数是应用程序的入口点
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let start_err = server::app::start().await;
    println!("{:?}", start_err);
}
