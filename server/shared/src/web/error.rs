use axum::response::{ IntoResponse, Response };
use redis::RedisError;
use sea_orm::DbErr;

use super::res::Res;

pub trait ApiError {
    fn code(&self) -> u16;
    fn message(&self) -> String;
}

#[derive(Debug)]
pub struct AppError {
    pub code: u16,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Res::<()>::new_error(self.code, self.message.as_str()).into_response()
    }
}

impl ApiError for AppError {
    fn code(&self) -> u16 {
        self.code
    }

    fn message(&self) -> String {
        self.message.to_string()
    }
}

impl ApiError for DbErr {
    fn code(&self) -> u16 {
        match self {
            DbErr::ConnectionAcquire(_) => 503, // 服务不可用
            DbErr::TryIntoErr { .. } => 400, // 请求参数错误
            DbErr::Conn(_) => 500, // 服务器内部错误
            DbErr::Exec(_) => 500, // 执行错误
            DbErr::Query(_) => 500, // 查询错误
            DbErr::ConvertFromU64(_) => 400, // 请求参数错误
            DbErr::UnpackInsertId => 500, // 服务器内部错误
            DbErr::UpdateGetPrimaryKey => 500, // 更新错误
            DbErr::RecordNotFound(_) => 404, // 未找到记录
            DbErr::AttrNotSet(_) => 400, // 请求参数错误
            DbErr::Custom(_) => 500, // 自定义错误
            DbErr::Type(_) => 400, // 类型错误
            DbErr::Json(_) => 400, // JSON解析错误
            DbErr::Migration(_) => 500, // 迁移错误
            DbErr::RecordNotInserted => 400, // 记录未插入
            DbErr::RecordNotUpdated => 404, // 记录未更新
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}



impl From<RedisError> for AppError {
    fn from(err: RedisError) -> Self {
        use redis::ErrorKind;
        let code = match err.kind() {
            ErrorKind::ResponseError => 500, // Redis响应错误
            ErrorKind::AuthenticationFailed => 401, // 认证失败
            ErrorKind::TypeError => 400, // 类型错误
            ErrorKind::ExecAbortError => 500, // 执行中止
            ErrorKind::BusyLoadingError => 503, // Redis正在加载
            ErrorKind::InvalidClientConfig => 400, // 客户端配置错误
            ErrorKind::IoError => 503, // IO错误，可能是网络问题
            ErrorKind::ExtensionError => 500, // 扩展错误
            _ => 500, // 其他错误
        };

        let message = if let Some(redis_code) = err.code() {
            format!("[{}] {}", redis_code, err)
        } else {
            format!("{}", err)
        };

        AppError { code, message }
    }
}
