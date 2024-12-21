use serde::{ Deserialize, Serialize };
use crate::model::entity::product_brand::ProductBrand;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]

pub struct Product {
    // 品牌
    product_brand: ProductBrand,
    // 商品SPU
    product_spu: ProductSpu,
    // SKU聚合
    product_sku: Vec<ProductSku>,
}