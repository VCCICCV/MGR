
use domain::{model::reponse::{ error::AppResult, response::{ SignInResponse, TokenResponse } }, utils::claim::UserClaims};
use uuid::Uuid;
use crate::dto::{
    command::{ ActiveCommand, RefreshTokenCommand, SignIn2FaCommand, SignInCommand, SignUpCommand },
    query::TokenInfoQuery,
};
// 这里也可以抽象为一个execute，但是需要每个命令都构建一个execute方法，这样冗余代码太多
pub trait CustomerUseCase: Sync + Send {
    async fn info_query_handler(
        &self,
        claims: UserClaims,
        token_info_query: TokenInfoQuery
    ) -> AppResult<UserClaims>;
    async fn refresh_command_handler(
        &self,
        refresh_token_command: RefreshTokenCommand
    ) -> AppResult<TokenResponse>;
    async fn logout_command_handler(&self, user_id: Uuid) -> AppResult;
    async fn sign_in_2fa_command_handler(
        &self,
        sign_in_2fa_command: SignIn2FaCommand
    ) -> AppResult<SignInResponse>;
    async fn sign_in_command_handler(
        &self,
        sign_in_command: SignInCommand
    ) -> AppResult<SignInResponse>;
    async fn active_command_handler(&self, active_command: ActiveCommand) -> AppResult;
    async fn sign_up_command_handler(&self, signup_command: SignUpCommand) -> AppResult<Uuid>;
}
