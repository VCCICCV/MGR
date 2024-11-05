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

// use domain::model::aggregate::customer::Customer;
// use crate::dto::response_dto::ListData;

// pub struct UserAssembler;

// impl UserAssembler {
//     // 将领域实体转换为DTO
//     pub fn to_list_data(customers: Vec<Customer>, total: u64, page_num: u64, page_size: u64) -> ListData<Customer> {
//         // 计算总页数，能整除说明最后一页是满的，否则最后一页不满
//         let total_pages = if total % page_size == 0 {
//             total / page_size
//         } else {
//             // 最后一页不满总页数+1
//             total / page_size + 1
//         };
//         ListData {
//             list: customers,
//             total,
//             total_pages,
//             page_num,
//         }
//     }
// }