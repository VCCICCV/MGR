// use domain::model::entity::user::User;
// use crate::dto::response_dto::UserRegisterRespDTO;
// // 领域实体转出DTO
// impl From<User> for UserRegisterRespDTO {
//     fn from(user: User) -> Self {
//         UserRegisterRespDTO {
//             username: user.get_username(),
//             email: user.get_email(),
//             password: user.get_password(),
//         }
//     }
// }
// // 领域实体转出DTO
// impl From<User> for UserLoginRespDTO {
//     fn from(user: User) -> Self {
//         UserLoginRespDTO {
//             username: user.get_username(),
//             email: user.get_email(),
//         }
//     }
// }
// DTO转领域实体
