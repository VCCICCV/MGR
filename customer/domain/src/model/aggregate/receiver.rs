use chrono::NaiveDateTime;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Receiver {
    // 收货人
    pub receive_name: String,
    // 收货电话
    pub receive_phone: String,
    // 收货邮编
    pub receive_post_code: String,
    // 收货省份
    pub receive_province: String,
    // 收货城市
    pub receive_city: String,
    // 收货区域
    pub receive_region: String,
    // 收货详细地址
    pub receive_detail_address: String,
    // 收货时间
    pub receive_time: NaiveDateTime,
}
