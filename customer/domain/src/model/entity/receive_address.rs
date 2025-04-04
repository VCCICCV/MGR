use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReceiveAddress {
    // id
    pub id: i64,
    // 用户uuid
    pub user_id: String,
    // 是否默认
    pub is_default: i16,
    // 收货人
    pub receive_name: String,
    // 收货手机号
    pub receive_phone: String,
    // 收货省份
    pub receive_province: String,
    // 收货城市
    pub receive_city: String,
    // 收货区
    pub receive_region: String,
    // 详细地址
    pub receive_detail_address: String,
}
