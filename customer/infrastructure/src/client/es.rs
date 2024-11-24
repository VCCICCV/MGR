use domain::model::reponse::error::AppResult;
use elasticsearch::http::transport::{ SingleNodeConnectionPool, TransportBuilder };
use elasticsearch::Elasticsearch;

use std::sync::Arc;
use crate::config::AppConfig;

// 类型别名
pub type EsClient = Arc<Elasticsearch>;
// 加载配置文件
pub trait EsClientExt: Sized {
    fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = AppResult<Self>>;
}

impl EsClientExt for EsClient {
    async fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        // 1、使用single_node方式创建client
        // let transport = Transport::single_node(&config.es.get_url()).unwrap();
        // let client = Elasticsearch::new(transport);
        // Ok(Arc::new(client))

        // 2、使用builder方式创建client
        let url = config.es.get_url();
        let url_parsed = url
            .parse::<elasticsearch::http::Url>().expect("Failed to parse url");
        let conn_pool = SingleNodeConnectionPool::new(url_parsed);
        let transport = TransportBuilder::new(conn_pool)
            .disable_proxy()
            .build().expect("Failed to build transport");
        let client = Elasticsearch::new(transport);
        Ok(Arc::new(client))
    }
}
// #[cfg(test)]
// mod tests {
//     use axum::http::HeaderMap;
//     use elasticsearch::{
//         cat::CatIndicesParts,
//         http::{ request::JsonBody, Method },
//         BulkParts,
//         DeleteParts,
//         IndexParts,
//         SearchParts,
//         UpdateParts,
//     };
//     use serde_json::{ json, Value };
//     use super::*;
//     use crate::constant::CONFIG;
//     // #[tokio::test]
//     // async fn test_create_index() {
//     //     // 创建client
//     //     let client_result = EsClient::build_from_config(&CONFIG).await;
//     //     assert!(client_result.is_ok());
//     //     let client = client_result.unwrap();
//     //     // 创建索引定义
//     //     let index_definition = json!({});
//     //     // 发送创建索引请求
//     //     let create_response = client
//     //         .index(elasticsearch::CreateParts::IndexId("new_index", "1"))
//     //         .body(index_definition)
//     //         .send().await;

//     //     assert!(create_response.is_ok());
//     //     let create_response = create_response.unwrap();
//     //     assert!(create_response.status_code().is_success());
//     // }

//     // #[tokio::test]
//     // async fn test_delete_index() {
//     //     // 创建client
//     //     let client_result = EsClient::build_from_config(&CONFIG).await;
//     //     assert!(client_result.is_ok());
//     //     let client = client_result.unwrap();
//     //     // 发送删除索引请求
//     //     let delete_response = client
//     //         .index(elasticsearch::IndexParts::Index("new_index"))
//     //         .send().await;
//     //     assert!(delete_response.is_ok());
//     // }
//     #[tokio::test]
//     async fn test_add_document() {
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();

//         let response = client
//             .index(IndexParts::IndexId("mgr", "1"))
//             .body(
//                 json!({
//                 "id": 1,
//                 "user": "cci",
//                 "post_date": "2024-01-15T00:00:00Z",
//                 "message": "Trying out Elasticsearch, so far so good?"
//             })
//             )
//             .send().await;

//         assert!(response.is_ok());
//         let response = response.unwrap();
//         assert!(response.status_code().is_success());
//     }

//     #[tokio::test]
//     async fn test_get_indices() {
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();

//         let get_index_response = client
//             .cat()
//             .indices(CatIndicesParts::Index(&["*"]))
//             .send().await;
//         assert!(get_index_response.is_ok());
//     }

//     #[tokio::test]
//     async fn test_update_document() {
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();

//         let update_response = client
//             .update(UpdateParts::IndexId("mgr", "1"))
//             .body(
//                 json!({
//                 "doc": {
//                     "message": "Updated message"
//                 }
//             })
//             )
//             .send().await;

//         assert!(update_response.is_ok());
//         let update_response = update_response.unwrap();
//         assert!(update_response.status_code().is_success());
//     }

//     #[tokio::test]
//     async fn test_delete_document() {
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();

//         let delete_response = client.delete(DeleteParts::IndexId("mgr", "1")).send().await;

//         assert!(delete_response.is_ok());
//         let delete_response = delete_response.unwrap();
//         assert!(delete_response.status_code().is_success());
//     }
//     #[tokio::test]
//     async fn test_doc() {
//         // 1、创建client
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();
//         // 2、定义DSL语句
//         // let body = b"{\"query\":{\"match_all\":{}}}";
//         let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

//         // add the first operation and document
//         body.push(json!({"index": {"_id": "1"}}).into());
//         body.push(
//             json!({
//     "id": 1,
//     "user": "kimchy",
//     "post_date": "2009-11-15T00:00:00Z",
//     "message": "Trying out Elasticsearch, so far so good?"
// }).into()
//         );

//         // add the second operation and document
//         body.push(json!({"index": {"_id": "2"}}).into());
//         body.push(
//             json!({
//     "id": 2,
//     "user": "forloop",
//     "post_date": "2020-01-08T00:00:00Z",
//     "message": "Bulk indexing with the rust client, yeah!"
// }).into()
//         );
//         // 3、发送请求
//         let response = client.bulk(BulkParts::Index("mgr")).body(body).send().await.unwrap();
//         assert!(response.status_code().is_success());
//     }
//     // 索引操作
//     #[tokio::test]
//     async fn test_create_index() {
//         // 1、创建client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、定义DSL
//         let index_definition = json!({
//             // "mappings":{
//             //     "properties":{
//             //         "age":{
//             //             "type":"integer"
//             //         }
//             //     }
//             // }
//         });
//         // 3、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Put,
//             format!("/mgr").as_str(),
//             HeaderMap::new(),
//             None,
//             Some(index_definition.to_string().as_bytes().to_vec()),
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_query_index() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、定义查询 DSL 语句
//         let query = json!({
//         "query": {
//             "match_all": {}
//         }
//     });
//         // 3、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Get,
//             format!("/mgr/_search").as_str(),
//             HeaderMap::new(),
//             None,
//             Some(query.to_string().as_bytes().to_vec()),
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_query_index2() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Get,
//             format!("/mgr").as_str(),
//             HeaderMap::new(),
//             None,
//             None,
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_update_index() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、定义查询 DSL 语句
//         let update_content =
//             json!({
//             "properties":{
//                 "age":{
//                 "type":"integer"
//                 }
//             }
//     });
//         // 3、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Put,
//             format!("/mgr/_mapping").as_str(),
//             HeaderMap::new(),
//             None,
//             Some(update_content.to_string().as_bytes().to_vec()),
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_delete_index() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、发送请求
//         let response = client.send::<(), ()>(
//             Method::Delete,
//             format!("/mgr").as_str(),
//             HeaderMap::new(),
//             None,
//             None,
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     // 文档操作
//     #[tokio::test]
//     async fn test_create_doc() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、定义查询 DSL 语句
//         let doc_content =
//             json!({
//             "id": "1",
//             "user": "kimchy",
//             "post_date": "2009-11-15T00:00:00Z",
//             "message": "Trying out Elasticsearch, so far so good?"
//         });
//         // 3、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Post,
//             format!("/mgr/_doc/1").as_str(),
//             HeaderMap::new(),
//             None,
//             Some(doc_content.to_string().as_bytes().to_vec()),
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_get_doc() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Get,
//             format!("/mgr/_doc/1").as_str(),
//             HeaderMap::new(),
//             None,
//             None,
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_get_doc_detail() {
//         // // 1、创建 client
//         // let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // // 2、发送请求
//         // let response = client.send::<Vec<u8>, ()>(
//         //     Method::Get,
//         //     "/mgr/_search",
//         //     HeaderMap::new(),
//         //     None,
//         //     None,
//         //     None
//         // ).await;
//         // assert!(response.is_ok());
//         // let response = response.unwrap();
//         // println!("{:?}", response);
//         // // 检查响应状态码
//         // assert_eq!(response.status_code().is_success(), true);
//         // // 解析响应体
//         // let response_body = response.json::<Value>().await.unwrap();
//         // for record in response_body.as_array().unwrap() {
//         //     // print the name of each index
//         //     println!("{}", record["index"].as_str().unwrap());
//         // }
//     }
//     #[tokio::test]
//     async fn test_update_doc() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、定义查询 DSL 语句
//         let doc_content =
//             json!({
//             "doc": {
//                 "message": "Updated message"
//             }
//         });
//         // 3、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Post,
//             format!("/mgr/_update/1").as_str(),
//             HeaderMap::new(),
//             None,
//             Some(doc_content.to_string().as_bytes().to_vec()),
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_delete_doc() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2、发送请求
//         let response = client.send::<Vec<u8>, ()>(
//             Method::Delete,
//             format!("/mgr/_doc/1").as_str(),
//             HeaderMap::new(),
//             None,
//             None,
//             None
//         ).await;
//         assert!(response.is_ok());
//         let response = response.unwrap();
//         println!("{:?}", response);
//         assert_eq!(response.status_code().is_success(), true);
//     }
//     #[tokio::test]
//     async fn test_bulk_add_to_mgr() {
//         // 1、创建client
//         let client_result = EsClient::build_from_config(&CONFIG).await;
//         assert!(client_result.is_ok());
//         let client = client_result.unwrap();
//         // 2、定义DSL语句
//         let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);
//         // 添加第一个操作和文档
//         body.push(json!({"index": {"_id": "1"}}).into());
//         body.push(
//             json!({
//     "id": 1,
//     "user": "kimchy",
//     "post_date": "2009-11-15T00:00:00Z",
//     "message": "Trying out Elasticsearch, so far so good?"
// }).into()
//         );

//         // 添加第二个操作和文档
//         body.push(json!({"index": {"_id": "2"}}).into());
//         body.push(
//             json!({
//     "id": 2,
//     "user": "forloop",
//     "post_date": "2020-01-08T00:00:00Z",
//     "message": "Bulk indexing with the rust client, yeah!"
// }).into()
//         );
//         // 3、发送请求
//         let response = client.bulk(BulkParts::Index("mgr")).body(body).send().await.unwrap();
//         assert!(response.status_code().is_success());
//     }
//     // 搜索
//     #[tokio::test]
//     async fn test_search_match_all() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "match_all": {
//                 }
//             }
//         })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
//     #[tokio::test]
//     async fn test_search_match() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "match": {
//                     "message": "good"
//                 }
//             }
//         })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
//     #[tokio::test]
//     async fn test_search_multi_match() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "multi_match": {
//                     "query": "good",
//                     "fields": [
//                         "message",
//                         "user"
//                         ]
//                     }
//                 }
//             })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
//     #[tokio::test]
//     async fn test_search_term() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "term": {
//                     "user": "kimchy"
//                     }
//                 }
//             })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
//     #[tokio::test]
//     async fn test_search_range() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "range": {
//                     "id": {
//                         "gte": 1,
//                         "lte": 1
//                         }
//                     }
//                 }
//             })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
//     #[tokio::test]
//     async fn test_function_score_query() {
//         // 1、创建 client
//         let client = EsClient::build_from_config(&CONFIG).await.unwrap();
//         // 2. 执行搜索
//         let response = client
//             .search(SearchParts::Index(&["mgr"]))
//             .from(0)
//             .size(5)
//             .body(
//                 json!({
//             "query": {
//                 "function_score": {
//                     "query": {
//                         "match": {// 查询方法
//                         "message": "good"
//                         }
//                     },
//                     "functions": [ // 算分函数
//                     {
//                         "filter": {// 只有符合过滤条件的才被计算
//                         "term": {// 根据词条精确查询
//                         "id": 1
//                         }
//                         },
//                         "weight": 3 // 指定加权函数
//                     }
//                 ],
//                     // 加权模式：相乘
//                     "boost_mode": "multiply"
//                     }
//                 }
//             })
//             )
//             .send().await
//             .unwrap();
//         // 3. 解析响应
//         let response_body = response.json::<Value>().await.unwrap();
//         // 搜索耗时
//         let took = response_body["took"].as_i64().unwrap();
//         println!("took: {}ms", took);
//         // 搜索结果
//         for hit in response_body["hits"]["hits"].as_array().unwrap() {
//             println!("{:?}", hit["_source"]);
//         }
//     }
// }
