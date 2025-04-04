use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerEmail{
    email: String,
}

impl CustomerEmail {
    pub fn new(email: String) -> Self {
        CustomerEmail{ email }
    }
}