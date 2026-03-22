use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::MySqlPool;
use crate::api::models::ApiResponse;

pub async fn get_app_extensions(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let app_id = params.get("app");
    let category = params.get("category");
    
    let extensions = if let Some(app_id) = app_id {
        let rows: Vec<(i32, String, Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT id, name, category, content FROM t_app_extension WHERE app_id = ? AND category = ?"
        )
        .bind(app_id.parse::<i32>().unwrap_or(1))
        .bind(category.as_ref().map(|s| s.as_str()).unwrap_or(""))
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
        
        rows.into_iter().map(|(id, name, category, content)| {
            serde_json::json!({
                "id": id,
                "name": name,
                "category": category,
                "content": content,
                "app": app_id
            })
        }).collect()
    } else {
        vec![]
    };
    
    Ok(Json(ApiResponse::success(extensions)))
}

pub async fn get_app_extension_by_id(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let app = params.get("app");
    let name = params.get("name");
    let category = params.get("category");
    
    let extension = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>)>(
        "SELECT id, name, category, content FROM t_app_extension WHERE app_id = ? AND name = ? AND category = ? LIMIT 1"
    )
    .bind(app.as_ref().and_then(|v| v.parse().ok()).unwrap_or(1))
    .bind(name.as_ref().map(|s| s.as_str()).unwrap_or(""))
    .bind(category.as_ref().map(|s| s.as_str()).unwrap_or(""))
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    match extension {
        Some((id, name, category, content)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": id,
                "name": name,
                "category": category,
                "content": content,
                "app": app
            }))))
        }
        None => Ok(Json(ApiResponse::success(serde_json::json!({})))),
    }
}

pub async fn create_app_extension(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Extension");
    let category = payload.get("category").and_then(|v| v.as_str()).unwrap_or("bridge");
    let content = payload.get("content").map(|v| v.to_string());
    let app_id = payload.get("app").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1);

    sqlx::query(
        "INSERT INTO t_app_extension (name, category, content, app_id, created_by, last_updated_by) VALUES (?, ?, ?, ?, 'system', 'system')"
    )
    .bind(name)
    .bind(category)
    .bind(&content)
    .bind(app_id)
    .execute(&pool)
    .await
    .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn update_app_extension(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let id = payload.get("id").and_then(|v| v.as_i64()).map(|v| v as i32);
    
    if let Some(id) = id {
        let name = payload.get("name").and_then(|v| v.as_str());
        let content = payload.get("content").map(|v| v.to_string());
        
        if let Some(name) = name {
            sqlx::query("UPDATE t_app_extension SET name = ? WHERE id = ?")
                .bind(name)
                .bind(id)
                .execute(&pool)
                .await
                .ok();
        }
    }

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn delete_app_extension(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let id = params.get("id").and_then(|v| v.parse::<i32>().ok());
    
    if let Some(id) = id {
        sqlx::query("DELETE FROM t_app_extension WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .ok();
    }

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn ext_get_page_histories(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let page_id = params.get("pageId").and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
    
    let histories: Vec<serde_json::Value> = sqlx::query_as::<_, (i32, i32, String, Option<String>, String)>(
        "SELECT id, ref_id, name, message, version FROM t_page_history WHERE ref_id = ? ORDER BY created_time DESC"
    )
    .bind(page_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(id, ref_id, name, message, version)| {
        serde_json::json!({
            "id": id,
            "pageId": ref_id,
            "name": name,
            "message": message,
            "version": version
        })
    })
    .collect();

    Ok(Json(ApiResponse::success(histories)))
}

pub async fn ext_get_page_history_detail(
    State(pool): State<MySqlPool>,
    Path(history_id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let history: Option<(i32, i32, String, Option<String>, String, Option<String>)> = sqlx::query_as(
        "SELECT id, ref_id, name, message, version, page_content FROM t_page_history WHERE id = ?"
    )
    .bind(history_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    match history {
        Some((id, ref_id, name, message, version, content)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": id,
                "pageId": ref_id,
                "name": name,
                "message": message,
                "version": version,
                "pageContent": content
            }))))
        }
        None => Ok(Json(ApiResponse::success(serde_json::json!({}))))
    }
}

pub async fn ext_create_page_history(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let page_id = payload.get("page_id").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1);
    let message = payload.get("message").and_then(|v| v.as_str()).map(|s| s.to_string());

    let version = format!("1.0.{}", chrono::Utc::now().timestamp());

    sqlx::query(
        "INSERT INTO t_page_history (ref_id, version, name, page_content, created_by, last_updated_by) VALUES (?, ?, 'Auto Save', '{}', 'system', 'system')"
    )
    .bind(page_id)
    .bind(&version)
    .execute(&pool)
    .await
    .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true, "version": version}))))
}

pub async fn ext_delete_page_history(
    State(pool): State<MySqlPool>,
    Path(history_id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    sqlx::query("DELETE FROM t_page_history WHERE id = ?")
        .bind(history_id)
        .execute(&pool)
        .await
        .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn ext_get_sources(
    State(pool): State<MySqlPool>,
    Path(app_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let sources: Vec<serde_json::Value> = sqlx::query_as::<_, (i32, String, Option<String>, String)>(
        "SELECT id, name, description, created_by FROM t_datasource WHERE app_id = ?"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(id, name, description, created_by)| {
        serde_json::json!({
            "id": id,
            "name": name,
            "description": description,
            "createdBy": created_by
        })
    })
    .collect();

    Ok(Json(ApiResponse::success(sources)))
}

pub async fn ext_get_source_detail(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let source: Option<(i32, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, config FROM t_datasource WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    match source {
        Some((id, name, config)) => {
            Ok(Json(ApiResponse::success(serde_json::json!({
                "id": id,
                "name": name,
                "config": config
            }))))
        }
        None => Ok(Json(ApiResponse::success(serde_json::json!({}))))
    }
}

pub async fn ext_create_source(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("DataSource");
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let app_id = payload.get("appId").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1);

    sqlx::query(
        "INSERT INTO t_datasource (name, description, app_id, created_by, last_updated_by) VALUES (?, ?, ?, 'system', 'system')"
    )
    .bind(name)
    .bind(&description)
    .bind(app_id)
    .execute(&pool)
    .await
    .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn ext_update_source(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = payload.get("name").and_then(|v| v.as_str());
    
    if let Some(name) = name {
        sqlx::query("UPDATE t_datasource SET name = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await
            .ok();
    }

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn ext_delete_source(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    sqlx::query("DELETE FROM t_datasource WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn get_app_templates(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_app_template(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn create_app_from_template(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let template_id = payload.get("templateId").and_then(|v| v.as_i64()).map(|v| v as i32);
    Ok(Json(ApiResponse::success(serde_json::json!({"success": true, "templateId": template_id}))))
}

pub async fn get_i18n_entries(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_i18n_langs(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let langs = vec![
        serde_json::json!({
            "label": "中文",
            "value": "zh_CN",
            "is_default": true
        }),
        serde_json::json!({
            "label": "English",
            "value": "en_US",
            "is_default": false
        })
    ];
    Ok(Json(ApiResponse::success(langs)))
}

pub async fn get_datasource_list(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let app_id = params.get("app")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);
    
    let datasources: Vec<serde_json::Value> = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>)>(
        "SELECT id, name, description, config FROM t_datasource WHERE app_id = ?"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(id, name, description, config)| {
        let config_json: serde_json::Value = config
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(serde_json::Value::Null);
        serde_json::json!({
            "id": id,
            "name": name,
            "description": description,
            "config": config_json
        })
    })
    .collect();

    Ok(Json(ApiResponse::success(datasources)))
}

pub async fn get_i18n_entry(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({}))))
}

pub async fn create_i18n_entry(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn create_i18n_entries_batch(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn update_i18n_entry(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn delete_i18n_entries_bulk(
    State(_pool): State<MySqlPool>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn update_app_i18n_entries(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn multi_update_app_i18n(
    State(_pool): State<MySqlPool>,
    Path(_id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn get_task_status(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id, "status": "pending"}))))
}

pub async fn ai_chat(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": format!("AI response to: {}", message),
        "success": true
    }))))
}

pub async fn ai_completions(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"choices": []}))))
}

pub async fn get_encrypt_key(
    State(_pool): State<MySqlPool>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"key": ""}))))
}

pub async fn ai_search(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"results": []}))))
}

pub async fn get_canvas_lock(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let id = params.get("id")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);
    let state = params.get("state").cloned().unwrap_or_else(|| "occupy".to_string());
    let lock_type = params.get("type").cloned().unwrap_or_else(|| "page".to_string());
    
    tracing::info!("Canvas lock: id={}, state={}, type={}", id, state, lock_type);
    
    match state.as_str() {
        "occupy" => {
            let target_table = if lock_type == "page" { "t_page" } else { "t_block" };
            let query = format!(
                "UPDATE {} SET occupier_by = 'developer' WHERE id = ?",
                target_table
            );
            sqlx::query(&query)
                .bind(id)
                .execute(&pool)
                .await
                .ok();
            
            Ok(Json(ApiResponse::success(serde_json::json!({
                "operate": "success"
            }))))
        }
        "release" => {
            let target_table = if lock_type == "page" { "t_page" } else { "t_block" };
            let query = format!(
                "UPDATE {} SET occupier_by = NULL WHERE id = ?",
                target_table
            );
            sqlx::query(&query)
                .bind(id)
                .execute(&pool)
                .await
                .ok();
            
            Ok(Json(ApiResponse::success(serde_json::json!({
                "operate": "success"
            }))))
        }
        _ => {
            let target_table = if lock_type == "page" { "t_page" } else { "t_block" };
            let query = format!(
                "SELECT occupier_by FROM {} WHERE id = ?",
                target_table
            );
            let is_locked = sqlx::query(&query)
                .bind(id)
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten()
                .is_some();
            
            Ok(Json(ApiResponse::success(serde_json::json!({
                "locked": is_locked,
                "operate": "success"
            }))))
        }
    }
}

pub async fn get_block_groups(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let ids = params.get("id");
    let app_id = params.get("app").and_then(|v| v.parse::<i32>().ok());
    
    let groups: Vec<serde_json::Value> = sqlx::query_as::<_, (i32, String, Option<String>)>(
        "SELECT id, name, description FROM t_block_group WHERE app_id = ? OR 1=1 LIMIT 50"
    )
    .bind(app_id.unwrap_or(1))
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(id, name, description)| {
        serde_json::json!({
            "id": id,
            "name": name,
            "description": description
        })
    })
    .collect();

    Ok(Json(ApiResponse::success(groups)))
}

pub async fn create_block_group(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Group");
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let app_id = payload.get("appId").and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(1);

    sqlx::query(
        "INSERT INTO t_block_group (name, description, app_id, platform_id, created_by, last_updated_by) VALUES (?, ?, ?, 1, 'system', 'system')"
    )
    .bind(name)
    .bind(&description)
    .bind(app_id)
    .execute(&pool)
    .await
    .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn update_block_group(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = payload.get("name").and_then(|v| v.as_str());
    
    if let Some(name) = name {
        sqlx::query("UPDATE t_block_group SET name = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await
            .ok();
    }

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn delete_block_group(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    sqlx::query("DELETE FROM t_block_group WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .ok();

    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn get_business_categories(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_business_categories_by_group(
    State(_pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let _group = params.get("group").map(|s| s.clone()).unwrap_or_default();
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_component_libraries(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn create_component_library(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn update_component_library(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn delete_component_library(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_component_library_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn create_component_bundle(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn split_component_bundle(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn create_components_batch(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn ext_get_resources(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_resource_by_id(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_resources_by_group(
    State(_pool): State<MySqlPool>,
    Path(_group_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn search_resources(
    State(_pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    let _keyword = params.get("keyword").map(|s| s.clone()).unwrap_or_default();
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn create_resource(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn upload_resource(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn create_resources_batch(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"success": true}))))
}

pub async fn update_resource(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn delete_resource(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_resource_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn download_resource(
    State(_pool): State<MySqlPool>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"name": name}))))
}

pub async fn get_resource_groups(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_resource_groups_by_app(
    State(_pool): State<MySqlPool>,
    Path(_app_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn create_resource_group(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn update_resource_group(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn delete_resource_group(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_resource_group_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_models(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn find_model(
    State(_pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let name = params.get("nameCn").map(|s| s.clone()).unwrap_or_default();
    Ok(Json(ApiResponse::success(serde_json::json!({"name": name}))))
}

pub async fn create_model(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn update_model(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn delete_model(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_model_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_model_sql(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"sql": "", "id": id}))))
}

pub async fn get_all_model_sqls(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn create_page_template(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn delete_page_templates_bulk(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn get_page_template_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_page_templates(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_task_detail(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id, "status": "pending"}))))
}

pub async fn get_tasks_status(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_platform_histories(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_platform_history(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn create_platform_history(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn update_platform_history(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn delete_platform_history(
    State(_pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_tenants(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn create_tenant(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn update_tenant(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(payload)))
}

pub async fn delete_tenant(
    State(_pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let id = params.get("id").map(|s| s.clone()).unwrap_or_default();
    Ok(Json(ApiResponse::success(serde_json::json!({"id": id}))))
}

pub async fn get_resources_api(
    State(_pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, crate::error::AppError> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_app_schema_v1(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::error::AppError> {
    let app = sqlx::query_as::<_, crate::api::models::App>(
        "SELECT * FROM t_app WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    match app {
        Some(app) => {
            let pages = sqlx::query_as::<_, crate::api::models::Page>(
                "SELECT * FROM t_page WHERE app_id = ?"
            )
            .bind(id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let home_page_id = app.home_page_id.unwrap_or(0);

            let components_tree: Vec<serde_json::Value> = pages.into_iter().map(|page| {
                let is_home = page.id == home_page_id;
                let page_content: Option<serde_json::Value> = page.page_content.and_then(|s| {
                    if s.is_empty() || s == "{}" {
                        None
                    } else {
                        serde_json::from_str(&s).ok()
                    }
                });
                let group = page.group.unwrap_or_default();

                serde_json::json!({
                    "id": page.id,
                    "label": page.name,
                    "name": page.name,
                    "route": page.route,
                    "path": page.route,
                    "componentName": if page.is_page == 1 { "Page" } else { "Folder" },
                    "isHome": is_home,
                    "is_body": page.is_body.unwrap_or(0),
                    "parentId": page.parent_id,
                    "parent_id": page.parent_id,
                    "group": group,
                    "depth": page.depth.unwrap_or(0),
                    "is_page": page.is_page,
                    "is_default": page.is_default,
                    "page_content": page_content,
                    "meta": {
                        "id": page.id,
                        "label": page.name,
                        "name": page.name,
                        "route": page.route,
                        "isHome": is_home,
                        "isBody": page.is_body.unwrap_or(0) == 1,
                        "group": group
                    }
                })
            }).collect();

            let packages = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, Option<String>, Option<String>)>(
                "SELECT id, name, npm, version, content, css FROM t_component_library LIMIT 100"
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let packages_json: Vec<serde_json::Value> = packages.into_iter().map(|(id, name, npm, version, content, css)| {
                let npm_obj: serde_json::Value = npm
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or(serde_json::Value::Null);
                serde_json::json!({
                    "id": id,
                    "name": name,
                    "npm": npm_obj,
                    "version": version.unwrap_or_default(),
                    "content": content,
                    "css": css
                })
            }).collect();

            let extensions = sqlx::query_as::<_, (i32, String, String, Option<String>)>(
                "SELECT id, name, category, content FROM t_app_extension WHERE app_id = ?"
            )
            .bind(id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let mut bridge = vec![];
            let mut utils = vec![];
            for (id, name, category, content) in extensions {
                let item = serde_json::json!({
                    "id": id,
                    "name": name,
                    "type": category,
                    "content": content
                });
                if category == "bridge" {
                    bridge.push(item);
                } else {
                    utils.push(item);
                }
            }

            let config = app.config.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    serde_json::from_str(&s).ok()
                }
            }).unwrap_or(serde_json::json!({
                "sdkVersion": "1.0.3",
                "historyMode": "hash",
                "targetRootID": "app"
            }));

            let schema = serde_json::json!({
                "id": app.id,
                "meta": {
                    "id": app.id,
                    "name": app.name,
                    "label": app.name,
                    "framework": app.framework.unwrap_or_else(|| "Vue".to_string()),
                    "isDemo": app.is_demo.is_some() && app.is_demo.unwrap() == 1,
                    "globalState": app.global_state,
                    "config": config
                },
                "componentsTree": components_tree,
                "componentsMap": [],
                "dataSource": {
                    "list": []
                },
                "i18n": {
                    "zh_CN": {},
                    "en_US": {}
                },
                "utils": utils,
                "bridge": bridge,
                "constants": app.constants.unwrap_or_default(),
                "css": app.css.unwrap_or_default(),
                "config": config,
                "packages": packages_json,
                "version": ""
            });

            Ok(Json(ApiResponse::success(schema)))
        }
        None => Ok(Json(ApiResponse::success(serde_json::json!({}))))
    }
}
