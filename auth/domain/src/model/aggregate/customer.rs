use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub is_deleted: i16,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
// convert CustomerDto to Customer
// impl From<CustomerDto> for Customer{
//     fn from(dto: CustomerDto) -> Self {
//         Slef{
//             id: dto.id,
//             user_id: dto.user_id,
//             username: dto.username,
//             email: dto.email,
//             password: dto.password,
//             avatar: dto.avatar,
//             is_deleted: dto.is_deleted,
//             create_time: dto.create_time,
//             update_time: dto.update_time,
//         }
//     }
// }
