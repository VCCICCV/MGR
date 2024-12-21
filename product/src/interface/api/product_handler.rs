
use axum::extract::State;

use tracing::info;

use crate::{dto::request::{ProductCategoryQuery, Res}, error::AppResult, state::AppState};

pub async fn list_categories(State(
    state,
): State<AppState>) -> AppResult<Res<ProductCategoryQuery>> {
    info!("Query product categories");
    // let use_case = CustomerUseCase::new(state.into());
    // match state.product_use_case.sign_up_command_handler().await {
    //     Ok(user_id) =>
    //         Ok(
    //             Res::with_data(user_id)
    //         ),
    //     Err(e) => Err(e),
    // }
    todo!()
}
