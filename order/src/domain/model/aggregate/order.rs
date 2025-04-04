use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Order {
    // 订单id
    order_id: i32,
    // 用户id
    user_id: String,
    // 订单编号
    order_sn: String,
    // 订单总金额
    total_amount:Decimal,
    // 支付金额
    pay_amount:Decimal,
    // 运费金额
    freight_amount:Decimal,
    // 支付方式
    
    // 支付时间
    // 订单类型
    // 自动确认天数
    // 物流公司
    // 物流单号
    // 收货人信息
    // 订单备注信息
    // 收货状态 0：未接收 1：已接收
    // 订单创建时间
    // 发货时间
    // 订单状态
    // 订单商品集合
}
