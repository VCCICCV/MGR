//*! # 这里是启动函数
//*!
//*! `main` 函数是应用程序的入口点，它返回一个 `Result<(), Box<dyn Error>>`。
mod server;
mod configure;
mod error;
mod constant;
mod state;
mod cmd;
mod dto;
mod infrastructure;
mod domain;
mod routers;
mod interface;
mod utils;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let start_err = server::start().await;
    println!("{:?}", start_err);
}
