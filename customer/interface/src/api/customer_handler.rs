use application::use_case::customer_use_case::CustomerUseCase;
use axum::{ extract::{ Json, State }, response::IntoResponse };
use application::state::AppState;
use domain::model::{
    dto::command::{ ActiveCommand, SignUpCommand },
    vo::response::{ MessageDto, Res, SignUpDto },
};
use garde::Validate;
use shared::error::AppResult;
use tracing::info;

pub async fn sign_up(
    State(state): State<AppState>,
    Json(signup_command): Json<SignUpCommand>
) -> AppResult<Json<SignUpDto>> {
    info!("Register new user with request: {:?}", signup_command);
    let use_case = CustomerUseCase::new(state.into());
    match use_case.sign_up(signup_command).await {
        Ok(user_id) =>
            Ok(
                Json(SignUpDto {
                    user_id: user_id,
                })
            ),
        Err(e) => Err(e),
    }
}
pub async fn active(
    State(state): State<AppState>,
    Json(active_command): Json<ActiveCommand>
) -> AppResult<Json<MessageDto>> {
    let use_case = CustomerUseCase::new(state.clone().into());
    match use_case.active(active_command).await {
        Ok(_) => {
            info!("User successfully activated.");
            Ok(Json(MessageDto::new("User successfully activated.")))
        }
        Err(e) => {
            info!("The user activation operation was not successful: {e:?}");
            Err(e)
        }
    }
}
