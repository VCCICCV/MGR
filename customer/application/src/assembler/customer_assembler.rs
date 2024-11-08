use chrono::Utc;
use domain::model::aggregate::customer::Customer;
use uuid::Uuid;
use crate::dto::request_command::CustomerCommand;
impl From<CustomerCommand> for Customer {
    fn from(command: CustomerCommand) -> Self {
        // 生成用户id
        let user_id = Uuid::new_v4();
        Customer {
            // 用户id
            user_id: user_id.as_u128() as i64,
            username: command.username,
            email: command.email,
            password: command.password,
            avatar: command.avatar,
        }
    }
}
