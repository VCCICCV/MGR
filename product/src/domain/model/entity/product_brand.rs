use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProductBrand {
    // id
    id: i32,
    // 品牌名称
    name: String,
    // 品牌介绍
    desc: String,
    // 品牌图
    pic: String,
    // 排序
    sort: i32,
}
