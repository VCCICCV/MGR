use async_trait::async_trait;
use pingora::{
    lb::LoadBalancer,
    prelude::{ HttpPeer, Opt, RoundRobin },
    proxy::{ ProxyHttp, Session },
    server::Server,
};
use pingora::services::{ listening::Service as ListeningService, Service };
use clap::Parser;
mod service;

use std::sync::Arc;
// 负载均衡器
// pub struct LB(Arc<LoadBalancer<RoundRobin>>);
// // 实现代理trait
// #[async_trait]
// impl ProxyHttp for LB {
//     type CTX = ();

//     #[doc = " Define how the `ctx` should be created."]
//     fn new_ctx(&self) -> Self::CTX {
//         ()
//     }

//     async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
//         let upstream = self.0
//             .select(b"", 256) // 对于轮询，哈希不重要
//             .unwrap();
//         println!("上游对等体是：{upstream:?}");
//         // 设置 SNI 到 one.one.one.one
//         let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
//         Ok(peer)
//     }
// }
pub struct BackgroundService;
fn main() {
    println!("Hello, world!");
    // 解析命令行参数
    let opt = Some(Opt::parse());
    //创建一个 Server 类型的实例 my_server，传入命令行参数解析结果
    let mut my_server = Server::new(opt).unwrap();
    // 服务器的初始化相关操作
    my_server.bootstrap();

    // 定义一个代理服务
    let proxy_service = service::proxy::proxy_service(
        "0.0.0.0:6141", // listen
        "1.1.1.1:80" // proxy to
    );

    // 定义一个 Prometheus 服务
    //后续可以通过该端口来获取程序相关的指标信息（比如服务器的运行状态、性能指标等，具体依赖于 Prometheus 相关的集成实现）
    let mut prometheus_service_http = ListeningService::prometheus_http_service();
    prometheus_service_http.add_tcp("127.0.0.1:6150");
    //  定义一个后台服务
    let background_service = background_service("example", BackgroundService {});
    // 启动服务器
    my_server.run_forever();
}
