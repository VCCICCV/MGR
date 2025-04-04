#[derive(PartialEq, Eq)]
pub enum OrderStatus {
    PENDING, // 等待
    PROCESSING, // 处理中
    SHIPPED, // 已发货
    DELIVERED, // 已交付
    CANCELLED, // 取消
}
