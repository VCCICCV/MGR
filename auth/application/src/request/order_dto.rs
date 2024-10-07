use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct PlaceOrderDto {
    pub user_id: i32,
    pub product_ids: Vec<i32>,
}