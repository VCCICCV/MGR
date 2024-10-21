use common::error::InfraError;
use elasticsearch::http::transport::{ SingleNodeConnectionPool, TransportBuilder };
use elasticsearch::Elasticsearch;
use std::future::Future;
use std::sync::Arc;
use crate::config::AppConfig;

// 类型别名
pub type EsClient = Arc<Elasticsearch>;
// 加载配置文件
pub trait EsClientExt: Sized {
    fn build_from_config(config: &AppConfig) -> impl Future<Output = Result<Self, InfraError>>;
}

impl EsClientExt for EsClient {
    async fn build_from_config(config: &AppConfig) -> Result<Self, InfraError> {
        // 1、使用single_node方式创建client
        // let transport = Transport::single_node(&config.es.get_url()).unwrap();
        // let client = Elasticsearch::new(transport);
        // Ok(Arc::new(client))

        // 2、使用builder方式创建client
        let url = config.es.get_url();
        let url_parsed = url
            .parse::<elasticsearch::http::Url>()
            .map_err(|_| InfraError::OtherError("url err".to_string()))?;
        let conn_pool = SingleNodeConnectionPool::new(url_parsed);
        let transport = TransportBuilder::new(conn_pool)
            .disable_proxy()
            .build()
            .map_err(|_| InfraError::OtherError("transport err".to_string()))?;
        let client = Elasticsearch::new(transport);
        Ok(Arc::new(client))
    }
}
#[cfg(test)]
mod tests {
    use elasticsearch::{ cat::CatIndicesParts, DeleteParts, IndexParts, UpdateParts };
    use serde_json::json;
    use super::*;
    use crate::constant::CONFIG;
    #[tokio::test]
    async fn test_add_document() {
        let client_result = EsClient::build_from_config(&CONFIG).await;
        assert!(client_result.is_ok());
        let client = client_result.unwrap();

        let response = client
            .index(IndexParts::IndexId("mgr", "1"))
            .body(
                json!({
                "id": 1,
                "user": "cci",
                "post_date": "2024-01-15T00:00:00Z",
                "message": "Trying out Elasticsearch, so far so good?"
            })
            )
            .send().await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.status_code().is_success());
    }

    #[tokio::test]
    async fn test_get_indices() {
        let client_result = EsClient::build_from_config(&CONFIG).await;
        assert!(client_result.is_ok());
        let client = client_result.unwrap();

        let get_index_response = client
            .cat()
            .indices(CatIndicesParts::Index(&["*"]))
            .send().await;

        assert!(get_index_response.is_ok());
    }

    #[tokio::test]
    async fn test_update_document() {
        let client_result = EsClient::build_from_config(&CONFIG).await;
        assert!(client_result.is_ok());
        let client = client_result.unwrap();

        let update_response = client
            .update(UpdateParts::IndexId("mgr", "1"))
            .body(
                json!({
                "doc": {
                    "message": "Updated message"
                }
            })
            )
            .send().await;

        assert!(update_response.is_ok());
        let update_response = update_response.unwrap();
        assert!(update_response.status_code().is_success());
    }

    #[tokio::test]
    async fn test_delete_document() {
        let client_result = EsClient::build_from_config(&CONFIG).await;
        assert!(client_result.is_ok());
        let client = client_result.unwrap();

        let delete_response = client.delete(DeleteParts::IndexId("mgr", "1")).send().await;

        assert!(delete_response.is_ok());
        let delete_response = delete_response.unwrap();
        assert!(delete_response.status_code().is_success());
    }
}
