use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use crate::api::models::ApiResponse;

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
                "INSERT INTO t_user (username, role, tenant_id, created_by, last_updated_by) VALUES (?, 'developer', '1', 'system', 'system')"
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
        "INSERT INTO t_user (username, email, role, tenant_id, created_by, last_updated_by) VALUES (?, ?, 'developer', '1', 'system', 'system')"
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
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": 1,
        "username": "developer",
        "role": "admin"
    }))))
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
