use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{MySqlPool, Row};
use crate::api::models::{App, ApiResponse, CreateAppRequest, UpdateAppRequest, AppSchema};
use crate::error::{AppError, Result};

pub async fn create_app(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateAppRequest>,
) -> Result<Json<ApiResponse<App>>> {
    sqlx::query(
        r#"
        INSERT INTO t_app (name, platform_id, framework, description, tenant_id, created_by, last_updated_by)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&payload.name)
    .bind(payload.platform_id.unwrap_or(1))
    .bind(&payload.framework)
    .bind(&payload.description)
    .bind(&payload.tenant_id)
    .bind("system")
    .bind("system")
    .execute(&pool)
    .await?;

    let app_id = sqlx::query("SELECT LAST_INSERT_ID()")
        .fetch_one(&pool)
        .await?
        .get::<i64, _>(0);

    let app = get_app_by_id(&pool, app_id as i32).await?;
    Ok(Json(ApiResponse::success(app)))
}

pub async fn get_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<App>>> {
    let app = get_app_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(app)))
}

pub async fn list_apps(
    State(pool): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<App>>>> {
    let apps = sqlx::query_as::<_, App>(
        "SELECT * FROM t_app ORDER BY created_time DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiResponse::success(apps)))
}

pub async fn update_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateAppRequest>,
) -> Result<Json<ApiResponse<App>>> {
    let existing = get_app_by_id(&pool, id as i32).await?;
    let _ = existing;

    sqlx::query(
        r#"
        UPDATE t_app SET 
            name = COALESCE(?, name),
            description = COALESCE(?, description),
            home_page_id = COALESCE(?, home_page_id),
            global_state = COALESCE(?, global_state),
            data_source_global = COALESCE(?, data_source_global),
            extend_config = COALESCE(?, extend_config),
            last_updated_by = 'system',
            last_updated_time = NOW()
        WHERE id = ?
        "#
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.home_page_id)
    .bind(&payload.global_state.as_ref().map(|v| v.to_string()))
    .bind(&payload.data_source_global.as_ref().map(|v| v.to_string()))
    .bind(&payload.extend_config)
    .bind(id)
    .execute(&pool)
    .await?;

    let app = get_app_by_id(&pool, id as i32).await?;
    Ok(Json(ApiResponse::success(app)))
}

pub async fn delete_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<App>>> {
    let app = get_app_by_id(&pool, id as i32).await?;

    sqlx::query("DELETE FROM t_app WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(app)))
}

pub async fn get_app_schema(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<AppSchema>>> {
    let app = get_app_by_id(&pool, id as i32).await?;

    let schema = AppSchema {
        id: app.id,
        meta: crate::api::models::AppMeta {
            name: app.name.clone(),
            description: app.description,
            app_id: Some(app.id.to_string()),
            tenant: app.tenant_id.clone(),
            git_group: app.git_group.clone(),
            project_name: app.project_name.clone(),
            branch: app.branch.clone(),
            framework: app.framework.unwrap_or_default(),
            version: None,
        },
        i18n: serde_json::json!({}),
        utils: vec![],
        data_source: app.data_source_global
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or(serde_json::json!({})),
        global_state: vec![],
        page_schema: serde_json::json!([]),
        block_schema: vec![],
        components_map: vec![],
    };

    Ok(Json(ApiResponse::success(schema)))
}

pub async fn publish_app(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    sqlx::query("UPDATE t_app SET published = 1, last_updated_by = 'system', last_updated_time = NOW() WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "code": 200,
        "url": null,
        "result": "App published successfully"
    }))))
}

async fn get_app_by_id(pool: &MySqlPool, id: i32) -> Result<App> {
    sqlx::query_as::<_, App>("SELECT * FROM t_app WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("App {} not found", id)))
}
