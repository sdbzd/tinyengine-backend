use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use crate::api::models::{Page, ApiResponse, CreatePageRequest, UpdatePageRequest, PageHistory, CreatePageHistoryRequest};
use crate::error::{AppError, Result};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub name: Option<String>,
    pub order_by: Option<String>,
    pub created_by: Option<String>,
}

pub async fn create_page(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<ApiResponse<Page>>> {
    sqlx::query(
        r#"
        INSERT INTO t_page (name, app_id, route, page_content, is_body, parent_id, `group`, is_page, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&payload.name)
    .bind(payload.app_id)
    .bind(&payload.route)
    .bind(&payload.page_content)
    .bind(payload.is_body.unwrap_or(1))
    .bind(payload.parent_id.unwrap_or(0))
    .bind(&payload.group)
    .bind(payload.is_page.unwrap_or(1))
    .bind("1")
    .bind("system")
    .bind("system")
    .execute(&pool)
    .await?;

    let page_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let page = get_page_by_id(&pool, page_id as i32).await?;
    Ok(Json(ApiResponse::success(page)))
}

pub async fn get_page(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Page>>> {
    let page = get_page_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(page)))
}

pub async fn list_pages(
    State(pool): State<MySqlPool>,
    Path(app_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<Page>>>> {
    let pages = sqlx::query_as::<_, Page>(
        "SELECT * FROM t_page WHERE app_id = ? ORDER BY created_time DESC"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(pages)))
}

pub async fn update_page(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<ApiResponse<Page>>> {
    sqlx::query(
        r#"
        UPDATE t_page SET
            name = COALESCE(?, name),
            route = COALESCE(?, route),
            page_content = COALESCE(?, page_content),
            is_body = COALESCE(?, is_body),
            parent_id = COALESCE(?, parent_id),
            `group` = COALESCE(?, `group`),
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(&payload.name)
    .bind(&payload.route)
    .bind(&payload.page_content)
    .bind(payload.is_body)
    .bind(&payload.parent_id)
    .bind(&payload.group)
    .bind(id)
    .execute(&pool)
    .await?;

    let page = get_page_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(page)))
}

pub async fn delete_page(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Page>>> {
    let page = get_page_by_id(&pool, id as i32).await?;

    sqlx::query("DELETE FROM t_page WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(page)))
}

pub async fn create_page_history(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreatePageHistoryRequest>,
) -> Result<Json<ApiResponse<PageHistory>>> {
    let history = create_history_internal(
        &pool,
        payload.page,
        &Some(serde_json::to_string(&payload.page_info.page_content).unwrap_or_default()),
        &payload.message
    ).await?;

    Ok(Json(ApiResponse::success(history)))
}

pub async fn get_page_histories(
    State(pool): State<MySqlPool>,
    Path(page_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<PageHistory>>>> {
    let histories = sqlx::query_as::<_, PageHistory>(
        "SELECT * FROM t_page_history WHERE ref_id = ? ORDER BY created_time DESC"
    )
    .bind(page_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(histories)))
}

pub async fn get_page_history(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PageHistory>>> {
    let history = sqlx::query_as::<_, PageHistory>(
        "SELECT * FROM t_page_history WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Page history {} not found", id)))?;

    Ok(Json(ApiResponse::success(history)))
}

async fn get_page_by_id(pool: &MySqlPool, id: i32) -> Result<Page> {
    sqlx::query_as::<_, Page>("SELECT * FROM t_page WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Page {} not found", id)))
}

async fn create_history_internal(
    pool: &MySqlPool,
    page_id: i32,
    page_content: &Option<String>,
    message: &Option<String>,
) -> Result<PageHistory> {
    let page = get_page_by_id(pool, page_id).await?;
    
    let version = format!("1.0.{}", chrono::Utc::now().timestamp());
    
    sqlx::query(
        r#"
        INSERT INTO t_page_history (ref_id, version, name, app_id, route, page_content, is_body, parent_id, `group`, depth, is_page, is_default, message, is_home, tenant_id, is_published, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(page_id)
    .bind(&version)
    .bind(&page.name)
    .bind(page.app_id)
    .bind(&page.route)
    .bind(page_content)
    .bind(page.is_body)
    .bind(page.parent_id)
    .bind(&page.group)
    .bind(page.depth)
    .bind(page.is_page)
    .bind(page.is_default)
    .bind(message)
    .bind(0)
    .bind(page.tenant_id)
    .bind(0)
    .bind("system")
    .bind("system")
    .execute(pool)
    .await?;

    let history_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(pool)
        .await?
        .get::<i64, _>(0);

    sqlx::query_as::<_, PageHistory>("SELECT * FROM t_page_history WHERE id = ?")
        .bind(history_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}
