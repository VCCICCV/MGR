#[async_trait]
pub trait TAccessKeyService {
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError>;
    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError>;
    async fn delete_access_key(&self, id: &str) -> Result<(), AppError>;

    async fn initialize_access_key(&self) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysAccessKeyService;

impl SysAccessKeyService {
    async fn create_access_key_in_transaction(
        &self,
        txn: &DatabaseTransaction,
        access_key: SysAccessKeyActiveModel,
    ) -> Result<SysAccessKeyModel, AppError> {
        let result = access_key.insert(txn).await.map_err(AppError::from)?;

        // 添加到验证器
        server_core::sign::add_key(ValidatorType::Simple, &result.access_key_id, None).await;
        server_core::sign::add_key(
            ValidatorType::Complex,
            &result.access_key_id,
            Some(&result.access_key_secret),
        )
        .await;

        Ok(result)
    }

    async fn delete_access_key_in_transaction(
        &self,
        txn: &DatabaseTransaction,
        id: &str,
    ) -> Result<(), AppError> {
        // 先获取 access key 信息
        let access_key = SysAccessKey::find_by_id(id)
            .one(txn)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(AccessKeyError::AccessKeyNotFound))?;

        // 从数据库中删除
        SysAccessKey::delete_by_id(id)
            .exec(txn)
            .await
            .map_err(AppError::from)?;

        // 从验证器中移除
        server_core::sign::remove_key(ValidatorType::Simple, &access_key.access_key_id).await;
        server_core::sign::remove_key(ValidatorType::Complex, &access_key.access_key_id).await;

        Ok(())
    }
}

#[async_trait]
impl TAccessKeyService for SysAccessKeyService {
    async fn find_paginated_access_keys(
        &self,
        params: AccessKeyPageRequest,
    ) -> Result<PaginatedData<SysAccessKeyModel>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysAccessKey::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(SysAccessKeyColumn::Domain.contains(keywords));
            query = query.filter(condition);
        }

        let total = query
            .clone()
            .count(db.as_ref())
            .await
            .map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_access_key(
        &self,
        input: CreateAccessKeyInput,
    ) -> Result<SysAccessKeyModel, AppError> {
        let db = db_helper::get_db_connection().await?;
        let txn = db.begin().await.map_err(AppError::from)?;

        let access_key_id = format!("AK{}", Ulid::new().to_string());
        let access_key_secret = format!("SK{}", Ulid::new().to_string());

        let access_key = SysAccessKeyActiveModel {
            id: Set(Ulid::new().to_string()),
            domain: Set(input.domain),
            status: Set(input.status),
            description: Set(input.description),
            access_key_id: Set(access_key_id),
            access_key_secret: Set(access_key_secret),
            created_at: Set(Local::now().naive_local()),
            created_by: Set("TODO".to_string()),
            ..Default::default()
        };

        let result = match self
            .create_access_key_in_transaction(&txn, access_key)
            .await
        {
            Ok(result) => {
                txn.commit().await.map_err(AppError::from)?;
                result
            },
            Err(e) => {
                txn.rollback().await.map_err(AppError::from)?;
                return Err(e);
            },
        };

        Ok(result)
    }

    async fn delete_access_key(&self, id: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let txn = db.begin().await.map_err(AppError::from)?;

        match self.delete_access_key_in_transaction(&txn, id).await {
            Ok(_) => {
                txn.commit().await.map_err(AppError::from)?;
                Ok(())
            },
            Err(e) => {
                txn.rollback().await.map_err(AppError::from)?;
                Err(e)
            },
        }
    }

    async fn initialize_access_key(&self) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let access_keys = SysAccessKey::find()
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        for access_key in access_keys {
            server_core::sign::add_key(ValidatorType::Simple, &access_key.access_key_id, None)
                .await;
            server_core::sign::add_key(
                ValidatorType::Complex,
                &access_key.access_key_id,
                Some(&access_key.access_key_secret),
            )
            .await;
        }

        Ok(())
    }
}

#[instrument(skip(rx))]
pub async fn api_key_validate_listener(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Any + Send>>,
) {
    while let Some(event) = rx.recv().await {
        if let Some(api_key_event) = event.downcast_ref::<ApiKeyEvent>() {
            project_info!("API key validated: {:?}", api_key_event);
        }
    }
}
