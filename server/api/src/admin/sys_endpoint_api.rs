use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::{Path, Query},
    Extension,
};
use axum_casbin::{casbin::MgmtApi, CasbinAxumLayer};
use server_core::web::{auth::User, error::AppError, page::PaginatedData, res::Res};
use server_service::admin::{
    EndpointPageRequest, EndpointTree, SysEndpointModel, SysEndpointService, TEndpointService,
};

pub struct SysEndpointApi;

impl SysEndpointApi {
    pub async fn get_paginated_endpoints(
        Query(params): Query<EndpointPageRequest>,
        Extension(service): Extension<Arc<SysEndpointService>>,
    ) -> Result<Res<PaginatedData<SysEndpointModel>>, AppError> {
        service
            .find_paginated_endpoints(params)
            .await
            .map(Res::new_data)
    }

    pub async fn get_auth_endpoints(
        Path(role_code): Path<String>,
        Extension(user): Extension<User>,
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
    ) -> Result<Res<Vec<BTreeMap<String, String>>>, AppError> {
        let enforcer = cache_enforcer.get_enforcer();
        let enforcer_read = enforcer.read().await;

        let policies = enforcer_read.get_filtered_policy(0, vec![role_code, user.domain()]);

        let formatted_policies = policies
            .into_iter()
            .map(|policy| {
                let mut values = policy.into_iter();
                let mut map = BTreeMap::new();
                map.insert("v0".to_string(), values.next().unwrap_or_default());
                map.insert("v1".to_string(), values.next().unwrap_or_default());
                map.insert("v2".to_string(), values.next().unwrap_or_default());
                map.insert("v3".to_string(), values.next().unwrap_or_default());
                map
            })
            .collect();

        Ok(Res::new_data(formatted_policies))
    }

    pub async fn tree_endpoint(
        Extension(service): Extension<Arc<SysEndpointService>>,
    ) -> Result<Res<Vec<EndpointTree>>, AppError> {
        service.tree_endpoint().await.map(Res::new_data)
    }
}
