use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use crate::api::models::{App, ApiResponse, AppPageResponse};
use crate::error::{AppError, Result};

pub async fn get_all_apps(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<App>>>> {
    let apps = sqlx::query_as::<_, App>(
        "SELECT * FROM t_app ORDER BY created_time DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(apps)))
}

pub async fn get_apps_by_page(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<AppPageResponse>>> {
    let current_page: i32 = params.get("currentPage")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let page_size: i32 = params.get("pageSize")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let offset = (current_page - 1) * page_size;
    
    // Build WHERE clause with optional filters
    let mut where_clauses: Vec<String> = vec![];
    let mut param_values: Vec<String> = vec![];
    
    if let Some(name) = params.get("name") {
        if !name.is_empty() {
            where_clauses.push("name LIKE ?".to_string());
            param_values.push(format!("%{}%", name));
        }
    }
    
    if let Some(framework) = params.get("framework") {
        if !framework.is_empty() {
            where_clauses.push("framework = ?".to_string());
            param_values.push(framework.clone());
        }
    }
    
    if let Some(created_by) = params.get("createdBy") {
        if !created_by.is_empty() {
            where_clauses.push("created_by = ?".to_string());
            param_values.push(created_by.clone());
        }
    }
    
    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // Query with filters
    let query_str = format!(
        "SELECT * FROM t_app {} ORDER BY created_time DESC LIMIT ? OFFSET ?",
        where_clause
    );
    
    let mut query = sqlx::query_as::<_, App>(&query_str);
    for val in &param_values {
        query = query.bind(val);
    }
    query = query.bind(page_size).bind(offset);
    
    let apps = query.fetch_all(&pool).await?;

    // Count query with same filters
    let count_str = format!("SELECT COUNT(*) as count FROM t_app {}", where_clause);
    let mut count_query = sqlx::query(&count_str);
    for val in &param_values {
        count_query = count_query.bind(val);
    }
    
    let count_row = count_query.fetch_one(&pool).await?;
    let total: i64 = count_row.get("count");

    let response = AppPageResponse { apps, total };
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_app_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<App>>> {
    tracing::info!("get_app_by_id called with id: {}", id);
    let app = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("App {} not found", id)))?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn get_app_detail(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<App>>> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("App {} not found", id)))?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn create_app(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<App>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("name is required".into()))?
        .to_string();
    
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let framework = payload.get("framework")
        .and_then(|v| v.as_str())
        .unwrap_or("Vue")
        .to_string();
    
    let platform_id = payload.get("platformId")
        .or(payload.get("platform_id"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(1);
    
    let tenant_id = payload.get("tenantId")
        .or(payload.get("tenant_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let config = payload.get("config")
        .map(|v| v.to_string());

    tracing::info!("Creating app: name={}, framework={}, platform_id={}", name, framework, platform_id);

    sqlx::query(
        r#"
        INSERT INTO t_app (name, platform_id, framework, description, config, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, 'system', 'system')
        "#
    )
    .bind(&name)
    .bind(platform_id)
    .bind(&framework)
    .bind(&description)
    .bind(&config)
    .bind(&tenant_id)
    .execute(&pool)
    .await?;

    let row = sqlx::query("SELECT CAST(MAX(id) AS SIGNED) as id FROM t_app WHERE name = ?")
        .bind(&name)
        .fetch_one(&pool)
    .await?;
    let app_id_int: i64 = row.get("id");

    tracing::info!("New app ID: {}", app_id_int);

    let app = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(app_id_int as i32)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn update_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<App>>> {
    let existing = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("App {} not found", id)))?;

    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(&existing.name);
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .or(existing.description.as_deref());

    sqlx::query(
        r#"
        UPDATE t_app SET 
            name = ?,
            description = ?,
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .execute(&pool)
    .await?;

    let app = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn delete_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<App>>> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("App {} not found", id)))?;

    sqlx::query("DELETE FROM t_app WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn update_app_i18n(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn get_preview_metadata(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let page_id = params.get("pageId")
        .or(params.get("id"))
        .map(|v| v.clone())
        .unwrap_or_default();
    
    Ok(Json(ApiResponse::success(serde_json::json!({
        "pageId": page_id,
        "schema": {}
    }))))
}

pub async fn deploy_page(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let page_id = payload.get("page_id")
        .or(payload.get("id"))
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    if let Some(id) = page_id {
        sqlx::query("UPDATE t_page SET is_published = 1 WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await?;
    }

    Ok(Json(ApiResponse::success(serde_json::json!({
        "success": true,
        "page_id": page_id
    }))))
}

pub async fn get_source_tpl(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>> {
    let platform_id = params.get("platform")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);

    let templates = sqlx::query_as::<_, DatasourceTplRow>(
        "SELECT * FROM t_datasource_tpl WHERE platform_id = ? OR platform_id = 0 ORDER BY id ASC"
    )
    .bind(platform_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let result: Vec<serde_json::Value> = templates
        .into_iter()
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "name": t.name,
                "type": t.datasource_type,
                "host": t.host,
                "port": t.port,
                "database": t.database
            })
        })
        .collect();

    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_app_schema_components(
    State(pool): State<MySqlPool>,
    Path(app_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>> {
    let blocks = sqlx::query_as::<_, Block>(
        "SELECT * FROM t_block WHERE app_id = ? ORDER BY created_time DESC"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let components: Vec<serde_json::Value> = blocks
        .into_iter()
        .map(|b| {
            serde_json::json!({
                "id": b.id,
                "label": b.label,
                "name": b.name,
                "description": b.description
            })
        })
        .collect();

    Ok(Json(ApiResponse::success(components)))
}

#[derive(sqlx::FromRow)]
struct DatasourceTplRow {
    id: i32,
    name: Option<String>,
    datasource_type: Option<String>,
    host: Option<String>,
    port: Option<i32>,
    database: Option<String>,
}

use crate::api::models::Block;
