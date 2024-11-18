use serde::{Deserialize, Serialize};

/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParamsCommand {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}