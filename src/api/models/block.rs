use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Block {
    pub id: i32,
    pub label: String,
    pub name: Option<String>,
    pub framework: Option<String>,
    pub content: Option<String>,
    pub assets: Option<String>,
    pub last_build_info: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub latest_version: Option<String>,
    pub latest_history_id: Option<i32>,
    pub screenshot: Option<String>,
    pub path: Option<String>,
    pub occupier_by: Option<String>,
    pub is_official: Option<i8>,
    pub public: Option<i32>,
    pub is_default: Option<i8>,
    pub tiny_reserved: Option<i8>,
    pub npm_name: Option<String>,
    pub i18n: Option<String>,
    pub platform_id: i32,
    pub app_id: i32,
    pub content_blocks: Option<String>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_by: String,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRequest {
    pub label: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub framework: Option<String>,
    pub platform_id: Option<i32>,
    pub app_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBlockRequest {
    pub name: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockCategory {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub business_group: Option<String>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_by: String,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockCategoryRequest {
    pub code: String,
    pub name: String,
    pub business_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub app_id: i32,
    pub platform_id: i32,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_by: String,
    pub last_updated_time: DateTime<Utc>,
}
