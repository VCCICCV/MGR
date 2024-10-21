use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReceiveAddress {
    pub id: i64,
    pub user_id: i64,
    pub is_default: i16,
    pub receive_name: String,
    pub receive_phone: String,
    pub receive_province: String,
    pub receive_city: String,
    pub receive_region: String,
    pub receive_detail_address: String,
}
// 实现ReceiveAddress的new方法，方便创建对象
impl ReceiveAddress {
    pub fn new(
        id: i64,
        user_id: i64,
        is_default: i16,
        receive_name: String,
        receive_phone: String,
        receive_province: String,
        receive_city: String,
        receive_region: String,
        receive_detail_address: String,
    ) -> Self {
        ReceiveAddress{
            id,
            user_id,
            is_default,
            receive_name,
            receive_phone,
            receive_province,
            receive_city,
            receive_region,
            receive_detail_address,
        }
    }
}