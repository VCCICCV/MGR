use axum::async_trait;
use domain::model::reponse::error::AppResult;
use uuid::Uuid;
use crate::dto::command::{ ActiveCommand, SignUpCommand };

#[async_trait]
pub trait CustomerUseCase: Sync + Send {
    async fn login_command_handler(&self, sign_up_command: SignUpCommand) -> AppResult;
    async fn active_command_handler(&self, active_command: ActiveCommand) -> AppResult;
    async fn sign_up_command_handler(&self, signup_command: SignUpCommand) -> AppResult<Uuid>;
}
