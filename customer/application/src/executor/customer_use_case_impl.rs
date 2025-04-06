
use domain::{
    model::{
        aggregate::customer::CustomerBuilder,
        reponse::{ error::{ AppError, AppResult }, response::{ SignInResponse, TokenResponse } },
    },
    repositories::customer_repository::CustomerRepository,
    service::customer_service::CustomerService,
    utils::{ claim::UserClaims, password, session::Session },
};
use infrastructure::{ client::database::DatabaseClient, constant::REFRESH_TOKEN_DECODE_KEY };
use crate::dto::{
    command::{ ActiveCommand, RefreshTokenCommand, SignIn2FaCommand, SignInCommand, SignUpCommand },
    query::TokenInfoQuery,
};
use tracing::info;
use uuid::Uuid;
use sea_orm::TransactionTrait;
use std::sync::Arc;
use super::customer_use_case::CustomerUseCase;
pub struct CustomerUseCaseImpl {
    db: Arc<DatabaseClient>,
    customer_service: Arc<dyn CustomerService>,
    customer_repository: Arc<dyn CustomerRepository>,
    session: Arc<dyn Session>,
}
impl CustomerUseCaseImpl {
    pub fn new(
        db: Arc<DatabaseClient>,
        customer_service: Arc<dyn CustomerService>,
        customer_repository: Arc<dyn CustomerRepository>,
        session: Arc<dyn Session>
    ) -> Self {
        Self {
            db,
            customer_service,
            customer_repository,
            session,
        }
    }
}

impl CustomerUseCase for CustomerUseCaseImpl {
    async fn info_query_handler(
        &self,
        claims: UserClaims,
        token_info_query: TokenInfoQuery
    ) -> AppResult<UserClaims> {
        info!("Info request:{claims:?}: {token_info_query:?}");
        // 解码
        let token_data = UserClaims::decode(&token_info_query.token, &REFRESH_TOKEN_DECODE_KEY)?;
        // 校验是否存在
        self.session.check(&token_data.claims).await?;
        Ok(token_data.claims)
    }
    async fn refresh_command_handler(
        &self,
        refresh_token_command: RefreshTokenCommand
    ) -> AppResult<TokenResponse> {
        // 解码token
        let user_claims = UserClaims::decode(
            &refresh_token_command.token,
            &REFRESH_TOKEN_DECODE_KEY
        )?.claims;
        // 调用领域服务
        match self.customer_service.refresh(&user_claims).await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e),
        }
    }
    async fn logout_command_handler(&self, user_id: Uuid) -> AppResult {
        // 调用领域服务注销
        match self.customer_service.logout(&user_id).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
    async fn sign_in_2fa_command_handler(
        &self,
        sign_in_2fa_command: SignIn2FaCommand
    ) -> AppResult<SignInResponse> {
        // 转bo
        let customer = CustomerBuilder::new()
            .user_id(sign_in_2fa_command.user_id)
            .verify_code(Some(sign_in_2fa_command.code))
            .build();
        // 调用领域服务验证
        match self.customer_service.sign_in_2fa(customer).await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e),
        }
    }
    async fn sign_in_command_handler(
        &self,
        sign_in_command: SignInCommand
    ) -> AppResult<SignInResponse> {
        info!("Sign in request: {sign_in_command:?}.");
        match self.customer_repository.find_by_email_and_status(&sign_in_command.email, 0).await {
            Ok(Some(result)) => {
                // 转bo
                let customer = CustomerBuilder::new()
                    .id(*result.id())
                    .user_id(*result.user_id())
                    .username((*result.username()).to_string())
                    .email(sign_in_command.email)
                    .password(sign_in_command.password)
                    .is2fa(*result.is2fa())
                    .build();
                // 调用领域服务登录验证
                match self.customer_service.sign_in(customer).await {
                    Ok(resp) => Ok(resp),
                    Err(e) => Err(e),
                }
            }
            Ok(None) => Err(AppError::UserNotFound("用户不存在或未激活".to_string())),
            Err(e) => Err(e),
        }
    }
    async fn active_command_handler(&self, active_command: ActiveCommand) -> AppResult {
        info!("激活用户请求: {active_command:?}.");
        // 开启事务
        let tx = self.db.begin().await?;
        // 转bo
        let customer = CustomerBuilder::new()
            .user_id(active_command.user_id)
            .verify_code(Some(active_command.verify_code))
            .is_deleted(0)
            .build();
        // 调用领域服务激活
        self.customer_service.active(&tx, customer).await?;
        // 发送消息
        // 提交事务
        tx.commit().await?;
        Ok(())
    }
    async fn sign_up_command_handler(&self, signup_command: SignUpCommand) -> AppResult<Uuid> {
        info!("注册用户请求: {signup_command:?}.");
        // 开启事务
        let tx = self.db.begin().await?;
        // hash密码
        let hash_password = password::hash(signup_command.password).await?;
        // 生成user_id
        let user_id = Uuid::new_v4();
        //转bo
        let customer = CustomerBuilder::new()
            .user_id(user_id)
            .username(signup_command.username)
            .email(signup_command.email)
            .is_deleted(1)
            .password(hash_password)
            .build();
        self.customer_service.sign_up(&tx, customer).await?;
        // 设置角色
        tx.commit().await?;
        // 使用kafka通知激活发送
        Ok(user_id)
    }
}
//     // pub async fn login(&self, login_command: LoginCommand) -> AppResult<()> {
//         // info!("登录用户请求: {login_command:?}.");
//         // // 事务控制
//         // // 调用领域服务
//         // let token = self.state.customer_service.login(login_command).await?;
//         // 发送消息

//         // // 判断用户是否被删除
//         // if
//         //     let Some(customer) = self.state.customer_repository.find_by_username_and_status(
//         //         &login_command.email,
//         //         0
//         //     ).await?
//         // {
//         //     // 判断密码是否正确
//         //     password::verify(login_command.password, customer.password().to_string()).await?;
//         //     if *customer.is2fa() == 0 {
//         //         // 检查ttl是否过期
//         //         let key = Loginkey { user_id };
//         //         let ttl = self.state.redis.ttl(&key).await?;
//         //         if ttl > 0 {
//         //             return Ok(LoginResponse::Code {
//         //                 expire_in: ttl as u64,
//         //                 message: CHECK_EMAIL_MESSAGE.to_string(),
//         //             });
//         //         }
//         //         let login_code = utils::random_code(CODE_LEN)?;
//         //         // 保存消息到kafka
//         //         // todo
//         //         // 保存登陆验证码到redis
//         //         &self.state.redis.set(&key, &login_code).await?;
//         //     }
//         // }
//         // // 生成sessionid并保存到redis
//         // let session_id = service::session::set(user_id).await?;
//         // // 返回token
//         // let token = generate_tokens(user.user_id(), user.role(), session_id)?;
//         // Ok(LoginResponse::Token(resp))
//     // }
