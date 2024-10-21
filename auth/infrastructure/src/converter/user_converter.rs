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
