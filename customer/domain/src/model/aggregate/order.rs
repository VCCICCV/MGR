use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use super::receiver::Receiver;
// NaiveDateTime 是一个没有时区信息的日期时间类型，它只包含日期和时间，不包含时区信息。它的精度可以达到微秒级别。
// DataTime 是一个带有时区信息的日期时间类型，它包含日期、时间和时区信息。它的精度可以达到纳秒级别。
// 数据库中timestamp时区、datatime不带时区
pub struct Order {
    // 订单id
    pub id: i64,
    // 订单号uuid
    pub order_sn: String,
    // 用户id uuid
    pub user_id: String,
    // 订单总金额
    pub total_amount: Decimal,
    // 订单支付金额
    pub pay_amount: Decimal,
    // 订单运费
    pub freight_amount: Decimal,
    // 订单自动天数
    pub auto_confirm_day: i32,
    // 物流公司
    pub delivery_company: String,
    // 物流单号
    pub delivery_sn: String,
    // 发货时间
    pub delivery_time: NaiveDateTime,
    // 收货人
    pub receiver: Receiver,
    // 订单备注
    pub remark: String,
    // 确认状态
    pub confirm_status: i32,
    // 订单状态
    pub order_status: i32,
    // 支付类型
    pub pay_type: i32,
    // 支付信息
    pub pay_url: String,
    // 支付时间
    pub pay_time: NaiveDateTime,
}
