use serde::Serialize;

use super::sys_menu::MenuRoute;

#[derive(Clone, Debug, Serialize)]
pub struct AuthOutput {
    pub token: String,
    // 为了复用soybean-admin-nestjs前端,暂时弃用
    // pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoOutput {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserRoute {
    pub routes: Vec<MenuRoute>,
    pub home: String,
}
