use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEmail{
    email: String,
}

impl UserEmail {
    pub fn new(email: String) -> Self {
        UserEmail{ email }
    }
}