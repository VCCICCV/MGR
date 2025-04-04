// use application::{ state::AppState, use_case::customer_use_case::CustomerUseCase };
// use axum::{ extract::State, Json };
// use domain::model::{ dto::query::PageParams, entity::user::User, vo::{error::{AppError, AppResult}, response::ListData} };

// use tracing::info;
// pub async fn list(State(state): State<AppState>, param: PageParams) -> AppResult<Json<ListData<User>>> {
//     info!("Get user list with parameter: {param:?}");
//     let use_case = CustomerUseCase::new(state.into());
//     match use_case.list(param).await {
//         Ok(data) => Ok(Json(ListData { list: data })),
//         Err(e) => Err(AppError::from(e)),
//     }
// }
