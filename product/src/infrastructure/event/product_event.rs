#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductEvent {
    // 创建事件
    Created {},
    // 更新事件
    Updated {},
    // 删除事件
    Deleted {},
    // 库存变化事件
    StockChanged {},
    // 下架事件
    Unlisted {},
    // 上架事件
    Listed {},
}
impl Message for ProductEvent {
    fn name(&self) -> &'static str {
        match self {
            ProductEvent::Created { .. } => "ProductWasCreated",
            ProductEvent::Updated { .. } => "ProductWasUpdated",
            ProductEvent::Deleted { .. } => "ProductWasDeleted",
            ProductEvent::StockChanged { .. } => "ProductStockWasChanged",
            ProductEvent::Unlisted { .. } => "ProductWasUnlisted",
            ProductEvent::Listed { .. } => "ProductWasListed",
        }
    }
}
