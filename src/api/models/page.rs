use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub app_id: i32,
    pub route: String,
    pub page_content: Option<String>,
    pub is_body: Option<i8>,
    pub parent_id: i32,
    pub group: Option<String>,
    pub depth: Option<i32>,
    pub is_page: i8,
    pub occupier_by: Option<String>,
    pub is_default: i8,
    pub content_blocks: Option<String>,
    pub latest_version: Option<String>,
    pub latest_history_id: Option<i32>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePageRequest {
    pub name: String,
    pub app_id: i32,
    pub route: String,
    pub page_content: Option<String>,
    pub is_body: Option<i8>,
    pub parent_id: Option<i32>,
    pub group: Option<String>,
    pub is_page: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePageRequest {
    pub name: Option<String>,
    pub route: Option<String>,
    pub page_content: Option<String>,
    pub is_body: Option<i8>,
    pub parent_id: Option<i32>,
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageHistory {
    pub id: i32,
    pub ref_id: i32,
    pub version: Option<String>,
    pub name: String,
    pub app_id: i32,
    pub route: String,
    pub page_content: Option<String>,
    pub is_body: Option<i8>,
    pub parent_id: i32,
    pub group: Option<String>,
    pub depth: Option<i32>,
    pub is_page: i8,
    pub is_default: i8,
    pub message: Option<String>,
    pub is_home: i8,
    pub content_blocks: Option<String>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub is_published: i8,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePageHistoryRequest {
    pub page: i32,
    pub message: Option<String>,
    pub page_info: PageHistoryInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageHistoryInfo {
    pub page_content: serde_json::Value,
    pub parent_id: Option<i32>,
    pub is_home: bool,
    pub is_body: bool,
    pub group: Option<String>,
}
