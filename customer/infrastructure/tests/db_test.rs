// use infrastructure::database::db_connection::establish_db_connection;
// use tracing::info;
// #[tokio::test]
// async fn test_establish_db_connection() {
//     // 加载.env 文件，成功返回包含的值，失败返回None
//     dotenvy::dotenv().ok();
//     let conn = establish_db_connection().await;
//     match conn {
//         Ok(_) => info!("Connection test passed"),
//         Err(e) => panic!("Connection test failed: {:?}", e),
//     }
// }