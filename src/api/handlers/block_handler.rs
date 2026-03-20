use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use crate::api::models::{Block, ApiResponse, CreateBlockRequest, UpdateBlockRequest, BlockCategory, CreateBlockCategoryRequest, BlockGroup, BlockGroupRow, BlockCategoryRow};
use crate::error::{AppError, Result};

pub async fn create_block(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateBlockRequest>,
) -> Result<Json<ApiResponse<Block>>> {
    sqlx::query(
        r#"
        INSERT INTO t_block (label, name, description, content, framework, platform_id, app_id, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&payload.label)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.content)
    .bind(&payload.framework)
    .bind(payload.platform_id.unwrap_or(1))
    .bind(payload.app_id.unwrap_or(1))
    .bind("1")
    .bind("system")
    .bind("system")
    .execute(&pool)
    .await?;

    let block_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let block = get_block_by_id(&pool, block_id as i32).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn get_block(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Block>>> {
    let block = get_block_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn list_blocks(
    State(pool): State<MySqlPool>,
    Path(app_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>(
        "SELECT * FROM t_block WHERE app_id = ? ORDER BY created_time DESC"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn update_block(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBlockRequest>,
) -> Result<Json<ApiResponse<Block>>> {
    sqlx::query(
        r#"
        UPDATE t_block SET 
            name = COALESCE(?, name),
            label = COALESCE(?, label),
            description = COALESCE(?, description),
            content = COALESCE(?, content),
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(&payload.name)
    .bind(&payload.label)
    .bind(&payload.description)
    .bind(&payload.content)
    .bind(id)
    .execute(&pool)
    .await?;

    let block = get_block_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(block)))
}

pub async fn delete_block(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Block>>> {
    let block = get_block_by_id(&pool, id as i32).await?;

    sqlx::query("DELETE FROM t_block WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(block)))
}

pub async fn create_block_category(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateBlockCategoryRequest>,
) -> Result<Json<ApiResponse<BlockCategory>>> {
    sqlx::query(
        r#"
        INSERT INTO t_business_category (code, name, business_group, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&payload.code)
    .bind(&payload.name)
    .bind(&payload.business_group)
    .bind("1")
    .bind("system")
    .bind("system")
    .execute(&pool)
    .await?;

    let cat_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let category = get_category_by_id(&pool, cat_id as i32).await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn list_block_categories(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<BlockCategory>>>> {
    let rows = sqlx::query_as::<_, BlockCategoryRow>(
        "SELECT * FROM t_business_category ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await?;

    let mut categories: Vec<BlockCategory> = Vec::new();
    for row in rows {
        let mut category = BlockCategory::from(row);
        
        let material_ids: Vec<(i32,)> = sqlx::query_as(
            "SELECT material_id FROM r_material_category WHERE category_id = ?"
        )
        .bind(category.id)
        .fetch_all(&pool)
        .await?;

        let mut blocks: Vec<serde_json::Value> = Vec::new();
        for (material_id,) in material_ids {
            if let Ok(block) = get_block_by_id(&pool, material_id).await {
                blocks.push(serde_json::to_value(block).unwrap_or_default());
            }
        }
        category.blocks = blocks;
        categories.push(category);
    }

    Ok(Json(ApiResponse::success(categories)))
}

pub async fn list_all_blocks(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<Block>>>> {
    let blocks = sqlx::query_as::<_, Block>(
        "SELECT * FROM t_block ORDER BY created_time DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(blocks)))
}

pub async fn deploy_block(
    State(pool): State<MySqlPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let block_id = payload.get("block_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| AppError::Validation("block_id is required".to_string()))?;

    sqlx::query("UPDATE t_block SET `public` = 1, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
        .bind(block_id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": block_id,
        "status": "deployed"
    }))))
}

async fn get_block_by_id(pool: &MySqlPool, id: i32) -> Result<Block> {
    sqlx::query_as::<_, Block>("SELECT * FROM t_block WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Block {} not found", id)))
}

async fn get_category_by_id(pool: &MySqlPool, id: i32) -> Result<BlockCategory> {
    let row = sqlx::query_as::<_, BlockCategoryRow>("SELECT * FROM t_business_category WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Block category {} not found", id)))?;
    Ok(BlockCategory::from(row))
}

pub async fn list_block_groups(
    State(pool): State<MySqlPool>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<BlockGroup>>>> {
    let app_id = params.get("app")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(1);
    
    let rows = sqlx::query_as::<_, BlockGroupRow>(
        "SELECT * FROM t_block_group WHERE app_id = ? ORDER BY id ASC"
    )
    .bind(app_id)
    .fetch_all(&pool)
    .await?;

    let mut result_groups: Vec<BlockGroup> = Vec::new();
    for row in rows {
        let mut group = BlockGroup::from(row);
        
        let block_ids: Vec<(i32,)> = sqlx::query_as(
            "SELECT block_id FROM r_block_group_block WHERE block_group_id = ?"
        )
        .bind(group.id)
        .fetch_all(&pool)
        .await?;

        let mut blocks: Vec<serde_json::Value> = Vec::new();
        for (block_id,) in block_ids {
            if let Ok(block) = get_block_by_id(&pool, block_id).await {
                blocks.push(serde_json::to_value(block).unwrap_or_default());
            }
        }
        group.blocks = blocks;
        result_groups.push(group);
    }

    Ok(Json(ApiResponse::success(result_groups)))
}
