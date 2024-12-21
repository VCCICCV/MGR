#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProductSku {
    //  id
    id: i32,
    // 商品id
    product_id: i32,
    // 价格
    price: BigDecimal,
    // 库存
    stock: i32,
    // 锁定库存
    lock_stock: i32,
    // 图片
    pic: String,
    // 属性 json格式
    attribute: String,
}
