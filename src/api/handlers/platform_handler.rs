use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use crate::api::models::{ApiResponse, Platform};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let user = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, username, role FROM t_user WHERE username = ? LIMIT 1"
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await?;

    match user {
        Some((id, username, role)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "token": format!("dev_token_{}_{}", id, username),
                "user": {
                    "id": id,
                    "username": username,
                    "role": role
                }
            }))))
        }
        None => {
            sqlx::query(
                "INSERT INTO t_user (username, email, role, tenant_id, created_by, last_updated_by) VALUES (?, '', 'developer', '1', 'system', 'system')"
            )
            .bind(&payload.username)
            .execute(&pool)
            .await?;

            let user_id = sqlx::query("SELECT LAST_INSERT_ID()")
                .fetch_one(&pool)
                .await?
                .get::<i64, _>(0);

            Ok(Json(ApiResponse::success(serde_json::json!({
                "token": format!("dev_token_{}_{}", user_id, payload.username),
                "user": {
                    "id": user_id,
                    "username": payload.username,
                    "role": "developer"
                }
            }))))
        }
    }
}

pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    sqlx::query(
        "INSERT INTO t_user (username, email, role, tenant_id, created_by, last_updated_by) VALUES (?, COALESCE(?, ''), 'developer', '1', 'system', 'system')"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .execute(&pool)
    .await?;

    let user_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    Ok(Json(ApiResponse::success(serde_json::json!({
        "token": format!("dev_token_{}_{}", user_id, payload.username),
        "user": {
            "id": user_id,
            "username": payload.username,
            "role": "developer"
        }
    }))))
}

pub async fn get_current_user(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let user = sqlx::query_as::<_, (i32, String, String, String)>(
        "SELECT id, username, email, role FROM t_user LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;
    
    match user {
        Some((id, username, email, role)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": id,
                "username": username,
                "email": email,
                "role": role
            }))))
        }
        None => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": 1,
                "username": "developer",
                "email": "",
                "role": "admin"
            }))))
        }
    }
}

pub async fn get_user_info(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let user = sqlx::query_as::<_, (i32, String, String, String)>(
        "SELECT id, username, email, role FROM t_user LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;
    
    match user {
        Some((id, username, email, role)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": id,
                "username": username,
                "email": email,
                "role": role
            }))))
        }
        None => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": 1,
                "username": "developer",
                "email": "",
                "role": "admin"
            }))))
        }
    }
}

pub async fn get_tenant_info(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": "1",
        "name": "Default Tenant",
        "platformId": 1
    }))))
}

pub async fn get_resources(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_categories(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_i18n(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn get_organizations(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let orgs = vec![
        serde_json::json!({
            "id": "1",
            "name": "Default Organization"
        })
    ];
    Ok(Json(ApiResponse::success(orgs)))
}

pub async fn get_config(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "platformName": "TinyEngine",
        "version": "1.0.0"
    }))))
}

pub async fn get_user_perms(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "permissions": ["read", "write", "admin"]
    }))))
}

pub async fn get_settings(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn get_system_info(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "version": "1.0.0"
    }))))
}

pub async fn get_menu(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_nav(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_dashboard(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn get_home(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn get_stats(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn get_info(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "name": "TinyEngine",
        "version": "1.0.0"
    }))))
}

pub async fn get_notifications(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_messages(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_events(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_app_preview(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let app = sqlx::query_as::<_, crate::api::models::App>(
        "SELECT * FROM t_app WHERE id = ?"
    )
    .bind(id as i32)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("App {} not found", id)))?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": app.id,
        "name": app.name,
        "framework": app.framework,
        "published": app.published,
        "state": app.state
    }))))
}

pub async fn get_all_platforms(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<Platform>>>, crate::error::AppError> {
    let platforms = sqlx::query_as::<_, Platform>("SELECT * FROM t_platform ORDER BY created_time DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Ok(Json(ApiResponse::success(platforms)))
}

pub async fn get_platform_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Platform>>, crate::error::AppError> {
    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM t_platform WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Platform {} not found", id)))?;
    Ok(Json(ApiResponse::success(platform)))
}

pub async fn create_platform(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Platform>>, crate::error::AppError> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("New Platform")
        .to_string();

    sqlx::query(
        "INSERT INTO t_platform (name, tenant_id, created_by, last_updated_by) VALUES (?, '1', 'system', 'system')"
    )
    .bind(&name)
    .execute(&pool)
    .await?;

    let id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM t_platform WHERE id = ?")
        .bind(id as i32)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse::success(platform)))
}

pub async fn update_platform(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Platform>>, crate::error::AppError> {
    let name = payload.get("name")
        .and_then(|v| v.as_str());

    if let Some(name) = name {
        sqlx::query("UPDATE t_platform SET name = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await?;
    }

    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM t_platform WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Platform {} not found", id)))?;

    Ok(Json(ApiResponse::success(platform)))
}

pub async fn delete_platform(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Platform>>, crate::error::AppError> {
    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM t_platform WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Platform {} not found", id)))?;

    sqlx::query("DELETE FROM t_platform WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(platform)))
}
