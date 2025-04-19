use serde::{Deserialize, Serialize};
use shared::res::PageRequest;
use validator::Validate;

use crate::entities::sea_orm_active_enums::{MenuType, Status};


#[derive(Debug, Serialize, Deserialize)]
pub struct MenuPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct MenuInput {
    pub menu_type: MenuType,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Menu name must be between 1 and 100 characters"
    ))]
    pub menu_name: String,
    pub icon_type: Option<i32>,
    #[validate(length(max = 100, message = "Icon must not exceed 100 characters"))]
    pub icon: Option<String>,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Route name must be between 1 and 100 characters"
    ))]
    pub route_name: String,
    #[validate(length(
        min = 1,
        max = 200,
        message = "Route path must be between 1 and 200 characters"
    ))]
    pub route_path: String,
    #[validate(length(
        min = 1,
        max = 200,
        message = "Component must be between 1 and 200 characters"
    ))]
    pub component: String,
    #[validate(length(max = 100, message = "Path param must not exceed 100 characters"))]
    pub path_param: Option<String>,
    pub status: Status,
    #[validate(length(max = 100, message = "Active menu must not exceed 100 characters"))]
    pub active_menu: Option<String>,
    pub hide_in_menu: Option<bool>,
    #[validate(length(min = 1, max = 50, message = "PID must be between 1 and 50 characters"))]
    pub pid: String,
    pub sequence: i32,
    #[validate(length(max = 100, message = "i18n key must not exceed 100 characters"))]
    pub i18n_key: Option<String>,
    pub keep_alive: Option<bool>,
    pub constant: bool,
    #[validate(length(max = 200, message = "Href must not exceed 200 characters"))]
    pub href: Option<String>,
    pub multi_tab: Option<bool>,
}

pub type CreateMenuInput = MenuInput;

#[derive(Deserialize, Validate)]
pub struct UpdateMenuInput {
    pub id: i32,
    #[serde(flatten)]
    pub menu: MenuInput,
}
