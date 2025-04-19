use server_core::web::{error::AppError, res::Res};

pub struct SysSandboxApi;

impl SysSandboxApi {
    pub async fn test_simple_api_key() -> Result<Res<String>, AppError> {
        Ok(Res::new_data("SimpleApiKey".to_string()))
    }

    pub async fn test_complex_api_key() -> Result<Res<String>, AppError> {
        Ok(Res::new_data("ComplexApiKey".to_string()))
    }
}
