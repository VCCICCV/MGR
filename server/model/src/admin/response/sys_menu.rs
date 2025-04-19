use chrono::NaiveDateTime;
use serde::Serialize;

use crate::entities::sea_orm_active_enums::{MenuType, Status};


#[derive(Debug, Serialize, Clone)]
pub struct MenuRoute {
    pub name: String,
    pub path: String,
    pub component: String,
    pub meta: RouteMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuRoute>>,
    pub id: i32,
    pub pid: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct RouteMeta {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    pub constant: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MenuTree {
    pub id: i32,
    pub pid: String,
    #[serde(rename = "menuType")]
    pub menu_type: MenuType,
    #[serde(rename = "menuName")]
    pub menu_name: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "iconType")]
    pub icon_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "routeName")]
    pub route_name: String,
    #[serde(rename = "routePath")]
    pub route_path: String,
    pub component: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pathParam")]
    pub path_param: Option<String>,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    pub sequence: i32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    pub constant: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedBy")]
    pub updated_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>,
}
