use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use crate::api::models::{Page, ApiResponse, PageHistory, PageResponse};
use crate::error::{AppError, Result};

pub async fn get_all_pages(
    State(pool): State<MySqlPool>,
    Path(app_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<PageResponse>>>> {
    let pages = sqlx::query_as::<_, Page>(
        "SELECT * FROM t_page WHERE app_id = ? ORDER BY created_time DESC"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await?;

    let responses: Vec<PageResponse> = pages.into_iter().map(PageResponse::from).collect();
    Ok(Json(ApiResponse::success(responses)))
}

pub async fn get_page_detail(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PageResponse>>> {
    let page = get_page_by_id(&pool, id).await?;
    let response = PageResponse::from(page);
    Ok(Json(ApiResponse::success(response)))
}

pub async fn create_page(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<PageResponse>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("name is required".into()))?
        .to_string();
    
    let app_id = payload.get("app_id")
        .or(payload.get("appId"))
        .and_then(|v| v.as_i64())
        .ok_or_else(|| AppError::Validation("app_id is required".into()))? as i32;
    
    let route = payload.get("route")
        .and_then(|v| v.as_str())
        .unwrap_or("/")
        .to_string();
    
    let page_content = payload.get("page_content")
        .or(payload.get("pageContent"))
        .map(|v| v.to_string())
        .unwrap_or_default();
    
    let is_page = payload.get("is_page")
        .or(payload.get("isPage"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i8)
        .unwrap_or(1);
    
    let parent_id = payload.get("parent_id")
        .or(payload.get("parentId"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(0);
    
    let group = payload.get("group")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    let is_body = payload.get("is_body")
        .or(payload.get("isBody"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i8)
        .unwrap_or(1);
    
    let tenant_id = payload.get("tenant_id")
        .or(payload.get("tenantId"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "1".to_string());

    if is_page == 1 {
        sqlx::query(
            r#"
            INSERT INTO t_page (name, app_id, route, page_content, is_body, parent_id, `group`, is_page, is_default, tenant_id, created_by, last_updated_by)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, 'system', 'system')
            "#
        )
        .bind(&name)
        .bind(app_id)
        .bind(&route)
        .bind(&page_content)
        .bind(is_body)
        .bind(parent_id)
        .bind(&group)
        .bind(is_page)
        .bind(&tenant_id)
        .execute(&pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            INSERT INTO t_page (name, app_id, route, page_content, is_body, parent_id, `group`, is_page, is_default, tenant_id, created_by, last_updated_by)
            VALUES (?, ?, '', '', 0, ?, ?, ?, 0, ?, 'system', 'system')
            "#
        )
        .bind(&name)
        .bind(app_id)
        .bind(parent_id)
        .bind(&group)
        .bind(is_page)
        .bind(&tenant_id)
        .execute(&pool)
        .await?;
    }

    let row = sqlx::query("SELECT CAST(MAX(id) AS SIGNED) as id FROM t_page WHERE name = ? AND app_id = ?")
        .bind(&name)
        .bind(app_id)
        .fetch_one(&pool)
        .await?;
    let page_id: i64 = row.get("id");

    let page = get_page_by_id(&pool, page_id as i32).await?;
    let response = PageResponse::from(page);
    Ok(Json(ApiResponse::success(response)))
}

pub async fn update_page(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<PageResponse>>> {
    let existing = get_page_by_id(&pool, id).await?;

    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(&existing.name);
    let route = payload.get("route")
        .and_then(|v| v.as_str())
        .unwrap_or(existing.route.as_str());
    let page_content = payload.get("page_content")
        .or(payload.get("pageContent"))
        .map(|v| v.to_string())
        .or(existing.page_content);
    
    let is_body = payload.get("is_body")
        .or(payload.get("isBody"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i8)
        .or(existing.is_body);
    
    let parent_id = payload.get("parent_id")
        .or(payload.get("parentId"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(existing.parent_id);
    
    let group = payload.get("group")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or(existing.group);
    
    let is_page = payload.get("is_page")
        .or(payload.get("isPage"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i8)
        .unwrap_or(existing.is_page);

    sqlx::query(
        r#"
        UPDATE t_page SET
            name = ?,
            route = ?,
            page_content = ?,
            is_body = ?,
            parent_id = ?,
            `group` = ?,
            is_page = ?,
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(name)
    .bind(route)
    .bind(&page_content)
    .bind(is_body)
    .bind(parent_id)
    .bind(&group)
    .bind(is_page)
    .bind(id)
    .execute(&pool)
    .await?;

    let page = get_page_by_id(&pool, id).await?;
    let response = PageResponse::from(page);
    Ok(Json(ApiResponse::success(response)))
}

pub async fn delete_page(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Page>>> {
    let page = get_page_by_id(&pool, id).await?;

    sqlx::query("DELETE FROM t_page WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(page)))
}

pub async fn get_page_histories(
    State(pool): State<MySqlPool>,
    Path(page_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<PageHistory>>>> {
    let histories = sqlx::query_as::<_, PageHistory>(
        "SELECT * FROM t_page_history WHERE ref_id = ? ORDER BY created_time DESC"
    )
    .bind(page_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(histories)))
}

pub async fn create_page_history(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<PageHistory>>> {
    let page_id = payload.get("page_id")
        .or(payload.get("ref_id"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(1);
    let message = payload.get("message")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let page = get_page_by_id(&pool, page_id).await?;
    let version = format!("1.0.{}", chrono::Utc::now().timestamp());
    let page_content = page.page_content.unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO t_page_history (ref_id, version, name, app_id, route, page_content, is_body, parent_id, `group`, depth, is_page, is_default, message, is_home, tenant_id, is_published, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, 0, 'system', 'system')
        "#
    )
    .bind(page_id)
    .bind(&version)
    .bind(&page.name)
    .bind(page.app_id)
    .bind(&page.route)
    .bind(&page_content)
    .bind(page.is_body)
    .bind(page.parent_id)
    .bind(&page.group)
    .bind(page.depth)
    .bind(page.is_page)
    .bind(page.is_default)
    .bind(&message)
    .bind(page.tenant_id)
    .execute(&pool)
    .await?;

    let history_row = sqlx::query("SELECT CAST(MAX(id) AS SIGNED) as id FROM t_page_history WHERE ref_id = ? AND version = ?")
        .bind(page_id)
        .bind(&version)
        .fetch_one(&pool)
        .await?;
    let history_id: i64 = history_row.get("id");

    let history = sqlx::query_as::<_, PageHistory>("SELECT * FROM t_page_history WHERE id = ?")
        .bind(history_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse::success(history)))
}

pub async fn get_page_history(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
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

pub async fn get_page_info(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": 1,
        "name": "Default Page",
        "route": "/"
    }))))
}

pub async fn get_published_pages(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<PageHistory>>>> {
    let app_id = payload.get("appId")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    
    let page_id = payload.get("pageId")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    let mut query = "SELECT * FROM t_page_history WHERE is_published = 1".to_string();
    
    if let Some(aid) = app_id {
        query.push_str(&format!(" AND app_id = {}", aid));
    }
    if let Some(pid) = page_id {
        query.push_str(&format!(" AND ref_id = {}", pid));
    }
    query.push_str(" ORDER BY created_time DESC LIMIT 50");

    let histories = sqlx::query_as::<_, PageHistory>(&query)
        .fetch_all(&pool)
        .await?;

    Ok(Json(ApiResponse::success(histories)))
}

pub async fn find_page_histories_by_name(
    State(pool): State<MySqlPool>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<PageHistory>>>> {
    let name = params.get("name")
        .map(|s| s.as_str())
        .unwrap_or("");
    let app_id = params.get("app")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1);

    let histories = sqlx::query_as::<_, PageHistory>(
        "SELECT * FROM t_page_history WHERE app_id = ? AND name LIKE ? ORDER BY created_time DESC LIMIT 50"
    )
    .bind(app_id)
    .bind(format!("%{}%", name))
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(histories)))
}

async fn get_page_by_id(pool: &MySqlPool, id: i32) -> Result<Page> {
    sqlx::query_as::<_, Page>("SELECT * FROM t_page WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Page {} not found", id)))
}

#[axum::debug_handler]
pub async fn copy_page(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<PageResponse>>> {
    let source_id = payload.get("page_id")
        .or(payload.get("id"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .ok_or_else(|| AppError::Validation("page_id is required".to_string()))?;

    let source = get_page_by_id(&pool, source_id).await?;

    let timestamp = chrono::Utc::now().timestamp_millis();

    let base_name = payload.get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| source.name.clone());

    let base_route = payload.get("route")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| source.route.clone());

    let new_name = format!("{}_{}", base_name, timestamp);
    let new_route = format!("{}_{}", base_route, timestamp);

    let new_app_id = payload.get("app_id")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(source.app_id);

    tracing::info!("Copy page: source_id={}, new_name={}, new_app_id={}", source_id, new_name, new_app_id);

    let insert_result = sqlx::query(
        r#"
        INSERT INTO t_page (name, app_id, route, page_content, is_body, parent_id, `group`, is_page, depth, is_default, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, 'system', 'system')
        "#
    )
    .bind(&new_name)
    .bind(new_app_id)
    .bind(&new_route)
    .bind(&source.page_content)
    .bind(source.is_body)
    .bind(source.parent_id)
    .bind(&source.group)
    .bind(source.is_page)
    .bind(source.depth)
    .bind(source.tenant_id)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => tracing::info!("Insert successful"),
        Err(e) => {
            tracing::error!("Insert failed: {}", e);
            return Err(AppError::Internal(e.to_string()).into());
        }
    }

    let new_id_row = sqlx::query("SELECT CAST(MAX(id) AS SIGNED) as id FROM t_page WHERE name = ? AND app_id = ?")
        .bind(&new_name)
        .bind(new_app_id)
        .fetch_one(&pool)
        .await?;
    let new_id: i64 = new_id_row.get("id");

    tracing::info!("New page id: {}", new_id);

    let new_page = get_page_by_id(&pool, new_id as i32).await?;
    let response = PageResponse::from(new_page);
    Ok(Json(ApiResponse::success(response)))
}

pub async fn restore_page_history(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<PageResponse>>> {
    let history_id = payload.get("history_id")
        .or(payload.get("historyId"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .ok_or_else(|| AppError::Validation("history_id is required".to_string()))?;

    let history = sqlx::query_as::<_, PageHistory>(
        "SELECT * FROM t_page_history WHERE id = ?"
    )
    .bind(history_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("History {} not found", history_id)))?;

    let page_id = history.ref_id;

    sqlx::query(
        r#"
        UPDATE t_page SET 
            page_content = ?,
            name = ?,
            route = ?,
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(&history.page_content)
    .bind(&history.name)
    .bind(&history.route)
    .bind(page_id)
    .execute(&pool)
    .await?;

    let page = get_page_by_id(&pool, page_id).await?;
    let response = PageResponse::from(page);
    Ok(Json(ApiResponse::success(response)))
}
