use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListRequest {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub username: Option<String>,

    #[serde(default)]
    pub email: Option<String>,
}

// 默认值处理函数
fn default_page() -> i32 { 1 }
fn default_page_size() -> i32 { 10 }

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub list: Vec<UserResponse>, //和go的切片功能类似
    pub total: i32,
    pub page: i32,
    pub total_page: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(default)]
    pub username: Option<String>,

    #[serde(default)]
    pub email: Option<String>,

    #[serde(default)]
    pub state: Option<i32>,
}