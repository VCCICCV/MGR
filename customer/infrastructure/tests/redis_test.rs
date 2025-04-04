// use infrastructure::{database::redis_connection::establish_redis_pool_connection, utils::redis_util::RedisUtil};
// use redis::AsyncCommands;
// #[tokio::test]
// async fn test_establish_db_connection() {
//     // 加载.env 文件，成功返回包含的值，失败返回None
//     dotenvy::dotenv().ok();
//     let pool = establish_redis_pool_connection().await;

//     match pool {
//         Ok(pool) => {
//             let mut conn = pool.get().await.unwrap();
//             conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
//             let result: String = conn.get("foo").await.unwrap();
//             assert_eq!(result, "bar");
//         }
//         Err(e) => panic!("Failed to establish Redis connection: {}", e),
//     }
// }
// #[tokio::test]
// async fn test_establish_db_connection2() {
//     // 加载.env 文件，成功返回包含的值，失败返回None
//     dotenvy::dotenv().ok();
//     // 建立redis连接池
//     let pool = establish_redis_pool_connection().await.unwrap();

//     // 获取连接
//     let mut conn = pool.get().await.unwrap();

//     // 查询所有key
//     let keys: Vec<String> = conn.keys("*").await.unwrap();

//     // 打印key
//     for key in &keys {
//         println!("Redis key: {}", key);
//     }
// }
// // redis_util_test
// #[tokio::test]
// async fn test_redis_util() {
//     // 加载.env 文件，成功返回包含的值，失败返回None
//     dotenvy::dotenv().ok();
//     let redis_util = RedisUtil::new().await.unwrap();
//     redis_util.set("foo", "bar").await.unwrap();
//     let result1: Option<String> = Some(redis_util.get("foo").await.unwrap().unwrap());
//     println!("{:?}", result1);
//     // let result2 = redis_util.get("foo").await.unwrap();
//     // println!("{:?}", result2)

// }
