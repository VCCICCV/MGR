use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Command {
    // id
    id: i64,
    // 父级id
    parent_id: i64,
    // 商品id
    product_id: Uuid,
    // 商品skuid
    product_sku_id: Uuid,
    // 用户id
    user_id: Uuid,
    // 点赞数量
    like_count: i32,
    // 回复数量
    reply_count: i32,
    // 评分
    star: i32,
    // 评论内容
    content: String,
    // 回复标识 0：用户1商家
    comment_flag: i16,
    // 匿名标识 0：否，1：是
    hide_flag: i16,
    // 追加标识 0：否，1：是
    append_flag: i16,
    // 评论图片地址
    resource: String,
}
