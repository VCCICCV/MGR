use bb8_redis::redis;
use thiserror::Error;
// 自定义错误类型
#[derive(Error, Debug)]
pub enum DomainError {
    // 1. 实体相关错误
    #[error("User entity validation error: {0}")] UserEntityValidationError(String),
    #[error("User not found with ID: {0}")] UserNotFoundError(String),
    #[error("User already exists with email: {0}")] UserAlreadyExistsError(String),
    #[error("Invalid user role: {0}")] InvalidUserRoleError(String),
    // 2. 业务规则相关错误
    #[error("Insufficient balance for user: {0}")] InsufficientBalanceError(u32),
    #[error("User is not eligible for this operation")]
    UserNotEligibleError,
    #[error("Operation not allowed at this time for user: {0}")] OperationNotAllowedError(u32),
    // 3. 仓储层相关错误（如果仓储层的错误需要在领域层进行特殊处理）
    #[error("Database error while saving user: {0}")] DatabaseSaveUserError(String),
    #[error("Database error while retrieving user: {0}")] DatabaseRetrieveUserError(String),
    #[error("Database connection error: {0}")] DatabaseConnectionError(String),
    // 4. 领域服务相关错误
    #[error("Password verification failed for user: {0}")] PasswordVerificationError(u32),
    #[error("Token generation failed for user: {0}")] TokenGenerationError(u32),
    #[error("Token verification failed")]
    TokenVerificationError,
    // 5. 与领域内数据一致性相关的错误
    #[error("Data integrity violation in user profile")]
    UserProfileDataIntegrityError,
    #[error("Inconsistent user state: {0}")] InconsistentUserStateException(String),
}
#[derive(Error, Debug)]
pub enum AppError {
    // {0}是应该格式化占位符，使用时将其替换为实际的错误消息。
    #[error("Request parameter error: {0}")] ReqParamError(String),
    #[error("Delete error: {0}")] ReqDeleteFail(String),
    #[error("IO error: {0}")] IOError(String),
    #[error("Register error: {0}")] RegisterError(String),
    #[error("Login error: {0}")] LoginError(String),
    #[error("Authenticate error: {0}")] AuthenticateError(String),
    #[error("Refresh token error: {0}")] RefreshTokenError(String),
    #[error("Network error: {0}")] NetworkError(String),
    #[error("Other error: {0}")] OtherError(String),
}
// 基础设施层错误类型
#[derive(Error, Debug)]
pub enum InfraError {
    #[error("Database error: {0}")] DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)] RedisError(#[from] redis::RedisError),
    #[error("IO error: {0}")] IoError(#[from] std::io::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("User create error: {0}")] UserError(String),
    #[error("Insert error: {0}")] InsertError(String),
    #[error("Email mismatch")]
    EmailMismatch,
    #[error("Password hash error: {0}")] PasswordHashError(String),
    #[error("Password verify error: {0}")] PasswordVerifyError(String),
    #[error("Jwt encode error: {0}")] JwtEncodeError(String),
    #[error("Jwt decode error: {0}")] JwtDecodeError(String),
    #[error("Network timeout error: {0}")] NetworkTimeoutError(String),
    #[error("Network connection error: {0}")] NetworkConnectionError(String),
    #[error("Config load error: {0}")] ConfigLoadError(String),
    #[error("Invalid input format error: {0}")] InvalidInputFormatError(String),
    #[error("Missing required field error: {0}")] MissingRequiredFieldError(String),
    #[error("ES error: {0}")] TransportError(elasticsearch::Error),
    #[error("ES Client error: {0}")] ClientError(elasticsearch::Error),
    #[error("Other error: {0}")] OtherError(String),
}
