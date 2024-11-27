use axum::async_trait;
use domain::model::reponse::error::AppResult;
use uuid::Uuid;
use crate::dto::command::{ ActiveCommand, SignUpCommand };
// 这里也可以抽象为一个execute，但是需要每个命令都构建一个execute方法，这样冗余代码太多

#[async_trait]
pub trait CustomerUseCase: Sync + Send {
    async fn login_command_handler(&self, sign_up_command: SignUpCommand) -> AppResult;
    async fn active_command_handler(&self, active_command: ActiveCommand) -> AppResult;
    async fn sign_up_command_handler(&self, signup_command: SignUpCommand) -> AppResult<Uuid>;
}