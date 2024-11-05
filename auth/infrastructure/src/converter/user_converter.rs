// use anyhow::Ok;

// use crate::po::user::Model;

// //转PO
// impl From<UserUserAggregate> for Model {
//     fn from(user: UserBOUserAggregate) -> Self {
//         Model {
//             id: user.id,
//             user_id: user.user_id,
//             username: user.username,
//             email: user.email,
//             password: user.password,
//             role: user.role,
//             avatar: user.avatar,
//             is_deleted: user.is_deleted,
//             create_time: user.create_time,
//             update_time: user.update_time,
//         }
//     }

// }
// // 转BO
// impl TryFrom<Model> for UserBOUserAggregate {
//     type Error;

use domain::model::aggregate::customer::Customer;

use crate::po::user::Model;

//     fn try_from(user: Model) -> Result<Self, Self::Error> {
//         Ok(User::Model {
//             id: user.id,
//             user_id: user.user_id,
//             username: user.username,
//             email: user.email,
//             password: user.password,
//             role: user.role,
//             avatar: Some(user.avatar),
//             is_deleted: user.is_deleted,
//             create_time: user.create_time,
//             update_time: user.update_time,
//         })
//     }
// }

impl From<Model> for Customer {
    fn from(user: Model) -> Self {
        Customer {
            id: user.id,
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            password: user.password,
            avatar: user.avatar,
            is_deleted: user.is_deleted,
            create_time: user.create_time,
            update_time: user.update_time,
            
        }
    }
}
