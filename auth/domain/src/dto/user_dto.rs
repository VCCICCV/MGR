#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
    pub avatar: Option<String>,
}
