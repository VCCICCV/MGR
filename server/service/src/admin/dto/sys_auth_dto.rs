use shared::constant::Audience;

#[derive(Clone, Debug)]
pub struct LoginContext {
    pub client_ip: String,
    pub client_port: Option<i32>,
    pub address: String,
    pub user_agent: String,
    pub request_id: String,
    pub audience: Audience,
    pub login_type: String,
    pub domain: String,
}
