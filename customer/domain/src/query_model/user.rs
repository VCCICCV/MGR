use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub is_deleted: i16,
    pub is2fa: i16,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
