// use sea_orm::{ActiveValue, Set};
// use domain::model::aggregate::customer::Customer;
// use crate::po::user::Model;
// // // 转Bo
// impl From<Model> for Customer {
//     fn from(user: Model) -> Self {
//         Customer {
//             user_id: user.user_id,
//             username: user.username,
//             email: user.email,
//             password: user.password,
//             avatar: user.avatar,
//             verify_code: None,
//             receive_address: vec![],
//         }
//     }
// }
// // 转Po，由于seaorm有Model和ActiveModel，我们直接手动实现Bo到Po的转换
// impl From<Customer> for Model {
//     fn from(user: Customer) -> Self {
//         Model {
//             // 设置默认值
//             id: 0,
//             user_id: user.user_id,
//             username: user.username,
//             email: user.email,
//             password: user.password,
//             avatar: user.avatar,
//             is_deleted: user.is_deleted,
//             create_time: user.create_time,
//             update_time: user.update_time,
            
//         }
//     }
// }
