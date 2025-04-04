use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerId {
    user_id: String,
}

impl CustomerId {
    pub fn new(user_id: String) -> Self {
        Self{user_id}
    }
}
