#[async_trait]
pub trait CustomerRepository: Send + Sync {
    // 根据spuid查询商品信息
    async fn find_by_spu_id(&self, spu_id: &Uuid) -> AppResult<Option<Customer>>;
    // 锁定商品库存
    // 解锁商品库存
    // 更新商品库存
}
