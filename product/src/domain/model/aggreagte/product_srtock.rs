use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProductStock{
    // 订单号
    order_sn:String,
    // 库存详情
    product_stock_details:Vec<ProductStockDetail>,
}