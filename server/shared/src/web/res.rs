use std::{fmt::Debug, string::ToString};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::page::PaginatedData;


#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: u16,
    pub data: Option<T>,
    pub msg: String,
    pub success: bool,
}

#[allow(dead_code)]
impl<T: Serialize> Res<T> {
    pub fn new_paginated(data: PaginatedData<T>) -> Res<PaginatedData<T>> {
        Res {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: "success".to_string(),
            success: true,
        }
    }

    pub fn new_success(data: T, msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: msg.to_string(),
            success: true,
        }
    }

    pub fn new_error(code: u16, msg: &str) -> Self {
        Self {
            code,
            data: None,
            msg: msg.to_string(),
            success: false,
        }
    }

    pub fn new_message(msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            msg: msg.to_string(),
            success: true,
        }
    }

    pub fn new_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            msg: "success".to_string(),
            success: true,
        }
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
