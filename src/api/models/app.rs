use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct App {
    #[serde(rename = "appId")]
    pub id: i32,
    pub name: String,
    pub platform_id: i32,
    pub platform_history_id: Option<i32>,
    pub publish_url: Option<String>,
    pub editor_url: Option<String>,
    pub visit_url: Option<String>,
    pub image_url: Option<String>,
    pub assets_url: Option<String>,
    pub state: Option<i32>,
    pub published: Option<i8>,
    pub home_page_id: Option<i32>,
    pub app_website: Option<String>,
    pub css: Option<String>,
    pub config: Option<String>,
    pub constants: Option<String>,
    pub data_handler: Option<String>,
    pub latest: Option<String>,
    pub git_group: Option<String>,
    pub project_name: Option<String>,
    pub branch: Option<String>,
    pub is_demo: Option<i8>,
    pub is_default: Option<i8>,
    pub template_type: Option<String>,
    pub set_template_time: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub set_template_by: Option<String>,
    pub set_default_by: Option<String>,
    pub framework: Option<String>,
    pub global_state: Option<String>,
    pub default_lang: Option<String>,
    pub extend_config: Option<String>,
    pub data_hash: Option<String>,
    pub can_associate: Option<i8>,
    pub data_source_global: Option<String>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub description: Option<String>,
    pub platform_id: Option<i32>,
    pub framework: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub home_page_id: Option<i32>,
    pub page_content: Option<serde_json::Value>,
    pub global_state: Option<serde_json::Value>,
    pub data_source_global: Option<serde_json::Value>,
    pub extend_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSchema {
    pub id: i32,
    pub meta: AppMeta,
    pub i18n: serde_json::Value,
    pub utils: Vec<serde_json::Value>,
    pub data_source: serde_json::Value,
    pub global_state: Vec<serde_json::Value>,
    pub page_schema: serde_json::Value,
    pub block_schema: Vec<serde_json::Value>,
    pub components_map: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMeta {
    pub name: String,
    pub description: Option<String>,
    pub app_id: Option<String>,
    pub tenant: Option<String>,
    pub git_group: Option<String>,
    pub project_name: Option<String>,
    pub branch: Option<String>,
    pub framework: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Platform {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub theme_id: Option<i32>,
    pub material_history_id: Option<i32>,
    pub business_category_id: Option<i32>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: i32,
    pub org_code: String,
    pub name: String,
    pub enabled: Option<i8>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: Option<String>,
    pub last_updated_by: Option<String>,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub enable: Option<i8>,
    pub tenant_id: Option<String>,
    pub renter_id: Option<String>,
    pub site_id: Option<String>,
    pub created_by: String,
    pub last_updated_by: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub code: String,
    pub message: String,
    pub is_success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            code: "200".to_string(),
            message: "操作成功".to_string(),
            is_success: true,
            error: None,
            err_msg: None,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            data: Some(data),
            code: "200".to_string(),
            message: message.to_string(),
            is_success: true,
            error: None,
            err_msg: None,
        }
    }

    pub fn failed(error_code: &str, message: &str) -> Self {
        Self {
            data: None,
            code: error_code.to_string(),
            message: message.to_string(),
            is_success: false,
            error: Some(serde_json::json!({
                "code": error_code,
                "message": message
            })),
            err_msg: Some(message.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPageResponse {
    pub apps: Vec<App>,
    pub total: i64,
}
