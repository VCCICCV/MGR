// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::error::Error;
//     use tokio::test;

//     #[test]
//     async fn test_find_by_email() -> Result<(), Box<dyn Error>> {
//         // 创建模拟的应用状态
//         let config = AppConfig { /* 初始化 */ };
//         let state = AppState::new(config).await?;

//         // 创建用户存储库实例
//         let repo = UserRepositoryImpl::new(&state);

//         // 测试存在的邮箱
//         let result = repo.find_by_email("test@example.com".to_string()).await?;
//         assert!(result.is_some());

//         // 测试不存在的邮箱
//         let result = repo.find_by_email("nonexistent@example.com".to_string()).await?;
//         assert!(result.is_none());

//         Ok(())
//     }
// }