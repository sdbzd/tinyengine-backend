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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    pub id: i32,
    pub name: String,
    pub app_id: i32,
    pub route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_body: Option<i8>,
    pub parent_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    pub is_page: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupier_by: Option<String>,
    pub is_default: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_blocks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_history_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

impl From<Page> for PageResponse {
    fn from(page: Page) -> Self {
        let page_content = page.page_content.and_then(|s| {
            if s.is_empty() || s == "{}" {
                None
            } else {
                serde_json::from_str(&s).ok()
            }
        });

        PageResponse {
            id: page.id,
            name: page.name,
            app_id: page.app_id,
            route: page.route,
            page_content,
            is_body: page.is_body,
            parent_id: page.parent_id,
            group: page.group,
            depth: page.depth,
            is_page: page.is_page,
            occupier_by: page.occupier_by,
            is_default: page.is_default,
            content_blocks: page.content_blocks,
            latest_version: page.latest_version,
            latest_history_id: page.latest_history_id,
            tenant_id: page.tenant_id,
            renter_id: page.renter_id,
            site_id: page.site_id,
            created_by: page.created_by,
            last_updated_by: page.last_updated_by,
            created_time: page.created_time,
            last_updated_time: page.last_updated_time,
        }
    }
}
