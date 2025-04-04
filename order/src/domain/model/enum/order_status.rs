pub enum OrderStatus {
    // 已创建
    Created,                                    
    // 待支付
    PendingPayment,
    // 支付失败
    PaymentFailed,
    // 已发货
    Shipped,
    // 派送中
    OutForDelivery,
    // 已送达
    Delivered,
    // 退货中
    Returning,
    // 已取消
    Canceled,
    // 已退货
    Returned,
}
