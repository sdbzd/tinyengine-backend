use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use crate::api::models::{Block, ApiResponse, BlockCategory, BlockCategoryRow, BlockGroup, BlockGroupRow, Tenant, User};
use crate::error::{AppError, Result};

pub async fn get_blocks_list(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM t_block ORDER BY created_time DESC LIMIT 100")
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn get_block_count(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<i64>>> {
    let name_cn = params.get("name_cn");
    let description = params.get("description");
    
    let count: (i64,) = if name_cn.is_some() || description.is_some() {
        let name_pattern = format!("%{}%", name_cn.map(|s| s.as_str()).unwrap_or("%"));
        let desc_pattern = format!("%{}%", description.map(|s| s.as_str()).unwrap_or("%"));
        sqlx::query_as("SELECT COUNT(*) FROM t_block WHERE name LIKE ? AND description LIKE ?")
            .bind(&name_pattern)
            .bind(&desc_pattern)
            .fetch_one(&pool)
            .await?
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM t_block")
            .fetch_one(&pool)
            .await?
    };
    Ok(Json(ApiResponse::success(count.0)))
}

pub async fn get_block_detail(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Block>>> {
    tracing::info!("get_block_detail called with id: {}", id);
    let block = get_block_by_id(&pool, id).await?;
    tracing::info!("Block found: {:?}", block);
    Ok(Json(ApiResponse::success(block)))
}

pub async fn create_block(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Block>>> {
    let label = payload.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("block")
        .to_string();
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Block")
        .to_string();
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let content = payload.get("content")
        .or(payload.get("blockContent"))
        .map(|v| v.to_string())
        .unwrap_or_default();
    let framework = payload.get("framework")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO t_block (label, name, description, content, framework, platform_id, app_id, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, 1, 1, '1', 'system', 'system')
        "#
    )
    .bind(&label)
    .bind(&name)
    .bind(&description)
    .bind(&content)
    .bind(&framework)
    .execute(&pool)
    .await?;

    let block_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let block = get_block_by_id(&pool, block_id as i32).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn delete_block(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Block>>> {
    let block = get_block_by_id(&pool, id).await?;
    sqlx::query("DELETE FROM t_block WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn get_blocks_paginated(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let page: i32 = params.get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let page_size: i32 = params.get("pageSize")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);
    let offset = (page - 1) * page_size;

    let blocks = sqlx::query_as::<_, Block>(
        "SELECT * FROM t_block ORDER BY created_time DESC LIMIT ? OFFSET ?"
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn get_block_tags(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<String>>>> {
    let tags: Vec<(String,)> = sqlx::query_as("SELECT DISTINCT tags FROM t_block WHERE tags IS NOT NULL AND tags != ''")
        .fetch_all(&pool)
        .await?;
    let result: Vec<String> = tags.into_iter().map(|(t,)| t).collect();
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_blocks_not_in_group(
    State(pool): State<MySqlPool>,
    Path(group_id): Path<i32>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>(
        r#"
        SELECT b.* FROM t_block b 
        LEFT JOIN r_block_group_block rg ON b.id = rg.block_id AND rg.block_group_id = ?
        WHERE rg.block_id IS NULL
        ORDER BY b.created_time DESC
        LIMIT 100
        "#
    )
    .bind(group_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn get_blocks_list2(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM t_block ORDER BY created_time DESC LIMIT 100")
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn get_all_tenants(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<Tenant>>>> {
    let tenants = sqlx::query_as::<_, Tenant>("SELECT * FROM t_tenant ORDER BY created_time DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Ok(Json(ApiResponse::success(tenants)))
}

pub async fn get_all_users(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<User>>>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM t_user ORDER BY created_time DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Ok(Json(ApiResponse::success(users)))
}

pub async fn get_blocks(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let app_id = params.get("appId");
    let group_id = params.get("groupId");

    let blocks = if let Some(app_id) = app_id {
        let app_id: i32 = app_id.parse().unwrap_or(1);
        sqlx::query_as::<_, Block>("SELECT * FROM t_block WHERE app_id = ? ORDER BY created_time DESC")
            .bind(app_id)
            .fetch_all(&pool)
            .await?
    } else {
        sqlx::query_as::<_, Block>("SELECT * FROM t_block ORDER BY created_time DESC LIMIT 100")
            .fetch_all(&pool)
            .await?
    };

    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn update_block(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Block>>> {
    let existing = get_block_by_id(&pool, id).await?;

    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(existing.name.as_ref().map(|s| s.as_str()).unwrap_or("Block"));
    let label = payload.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or(existing.label.as_str());
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .or(existing.description.as_deref());
    let content = payload.get("content")
        .or(payload.get("blockContent"))
        .map(|v| v.to_string())
        .unwrap_or(existing.content.unwrap_or_default());

    sqlx::query(
        r#"
        UPDATE t_block SET 
            name = ?,
            label = ?,
            description = ?,
            content = ?,
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(name)
    .bind(label)
    .bind(description)
    .bind(&content)
    .bind(id)
    .execute(&pool)
    .await?;

    let block = get_block_by_id(&pool, id).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn get_block_by_label(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Block>>> {
    let label = params.get("label")
        .ok_or_else(|| AppError::Validation("label is required".to_string()))?;
    let app_id = params.get("appId")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);

    let block = sqlx::query_as::<_, Block>(
        "SELECT * FROM t_block WHERE label = ? AND app_id = ? LIMIT 1"
    )
    .bind(label)
    .bind(app_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Block with label {} not found", label)))?;

    Ok(Json(ApiResponse::success(block)))
}

pub async fn deploy_block(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Block>>> {
    let block_id = payload.get("block_id")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .or_else(|| payload.get("blockId").and_then(|v| v.as_i64()).map(|v| v as i32));

    if let Some(id) = block_id {
        sqlx::query("UPDATE t_block SET `public` = 1, is_official = 1, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await?;
        
        let block = get_block_by_id(&pool, id).await?;
        Ok(Json(ApiResponse::success(block)))
    } else {
        Err(AppError::Validation("block_id is required".into()))
    }
}

pub async fn list_materials(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn list_components(
    _state: State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>> {
    Ok(Json(ApiResponse::success(Vec::<serde_json::Value>::new())))
}

pub async fn get_materials_list(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM t_block ORDER BY created_time DESC LIMIT 100")
        .fetch_all(&pool)
        .await?;
    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn create_material(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Block>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Material")
        .to_string();
    let label = payload.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("material")
        .to_string();

    sqlx::query(
        r#"
        INSERT INTO t_block (label, name, platform_id, app_id, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, 1, 1, '1', 'system', 'system')
        "#
    )
    .bind(&label)
    .bind(&name)
    .execute(&pool)
    .await?;

    let id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let block = get_block_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn update_material(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Block>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str());

    if let Some(name) = name {
        sqlx::query("UPDATE t_block SET name = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await?;
    }

    let block = get_block_by_id(&pool, id).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn delete_material(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Block>>> {
    let block = get_block_by_id(&pool, id).await?;
    sqlx::query("DELETE FROM t_block WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn get_material_detail(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Block>>> {
    let block = get_block_by_id(&pool, id).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn list_block_categories(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<BlockCategory>>>> {
    let rows = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category ORDER BY id ASC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    let categories: Vec<BlockCategory> = rows.into_iter().map(BlockCategory::from).collect();
    Ok(Json(ApiResponse::success(categories)))
}

pub async fn create_block_category(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<BlockCategory>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Category")
        .to_string();
    let code = payload.get("code")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| name.to_lowercase().replace(' ', "_"));

    sqlx::query(
        r#"
        INSERT INTO t_business_category (name, code, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, '1', 'system', 'system')
        "#
    )
    .bind(&name)
    .bind(&code)
    .execute(&pool)
    .await?;

    let id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let row = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    let category = BlockCategory::from(row);

    Ok(Json(ApiResponse::success(category)))
}

pub async fn list_block_groups(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<BlockGroup>>>> {
    let _app_id = params.get("app")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);
    
    let rows = sqlx::query_as::<_, BlockGroupRow>(
        "SELECT * FROM t_block_group ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();
    let groups: Vec<BlockGroup> = rows.into_iter().map(BlockGroup::from).collect();

    Ok(Json(ApiResponse::success(groups)))
}

async fn get_block_by_id(pool: &MySqlPool, id: i32) -> Result<Block> {
    sqlx::query_as::<_, Block>("SELECT * FROM t_block WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Block {} not found", id)))
}

pub async fn get_block_history(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>> {
    let block_id = params.get("block")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);

    let histories = sqlx::query_as::<_, BlockHistoryRow>(
        "SELECT * FROM t_block_history WHERE ref_id = ? ORDER BY created_time DESC"
    )
    .bind(block_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let result: Vec<serde_json::Value> = histories
        .into_iter()
        .map(|h| {
            serde_json::json!({
                "id": h.id,
                "ref_id": h.ref_id,
                "version": h.version,
                "name": h.name,
                "content": h.content,
                "created_time": h.created_time
            })
        })
        .collect();

    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_block_categories(
    State(pool): State<MySqlPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<BlockCategory>>>> {
    let categories = sqlx::query_as::<_, BlockCategoryRow>(
        "SELECT * FROM t_business_category ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let result: Vec<BlockCategory> = categories.into_iter().map(BlockCategory::from).collect();

    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_block_category_api(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<BlockCategory>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Category")
        .to_string();
    let code = payload.get("code")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| name.to_lowercase().replace(' ', "_"));

    sqlx::query(
        r#"
        INSERT INTO t_business_category (name, code, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, '1', 'system', 'system')
        "#
    )
    .bind(&name)
    .bind(&code)
    .execute(&pool)
    .await?;

    let id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let row = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    let category = BlockCategory::from(row);

    Ok(Json(ApiResponse::success(category)))
}

pub async fn update_block_category_api(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<BlockCategory>>> {
    let name = payload.get("name")
        .and_then(|v| v.as_str());
    let code = payload.get("code")
        .and_then(|v| v.as_str());

    if let Some(name) = name {
        sqlx::query("UPDATE t_business_category SET name = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&pool)
            .await?;
    }
    if let Some(code) = code {
        sqlx::query("UPDATE t_business_category SET code = ?, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
            .bind(code)
            .bind(id)
            .execute(&pool)
            .await?;
    }

    let row = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    let category = BlockCategory::from(row);

    Ok(Json(ApiResponse::success(category)))
}

pub async fn delete_block_category_api(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<BlockCategory>>> {
    let row = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    let category = BlockCategory::from(row);

    sqlx::query("DELETE FROM t_business_category WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(category)))
}

#[derive(sqlx::FromRow)]
struct BlockHistoryRow {
    id: i32,
    ref_id: Option<i32>,
    version: Option<String>,
    name: Option<String>,
    content: Option<String>,
    created_time: Option<chrono::DateTime<chrono::Utc>>,
}
