use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}