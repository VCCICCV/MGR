use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPassword {
    password: String,
}

impl UserPassword {
    pub fn new(password: String) -> Self {
        UserPassword { password }
    }
}