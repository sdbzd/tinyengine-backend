use super::handlers::*;
use super::handlers::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::MySqlPool;

pub fn create_app_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/apps", post(create_app))
        .route("/apps", get(list_apps))
        .route("/apps/:id", get(get_app))
        .route("/apps/:id", put(update_app))
        .route("/apps/:id", delete(delete_app))
        .route("/apps/schema/:id", get(get_app_schema))
        .route("/apps/publish/:id", post(publish_app))
        .route("/apps/page", get(list_apps))
        .route("/apps/create", post(create_app))
        .route("/apps/detail/:id", get(get_app))
        .route("/apps/delete/:id", get(delete_app))
        .route("/apps/preview/:id", get(get_app_preview))
        .with_state(pool.clone())
}

pub fn create_v1_app_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/apps/schema/:id", get(get_app_schema))
        .with_state(pool)
}

pub fn create_page_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/pages/create", post(create_page))
        .route("/pages/list/:appId", get(list_pages))
        .route("/pages/detail/:id", get(get_page))
        .route("/pages/update/:id", post(update_page))
        .route("/pages/delete/:id", get(delete_page))
        .route("/pages/histories", get(get_page_histories))
        .route("/pages/histories/create", post(create_page_history))
        .route("/pages/histories/:id", get(get_page_history))
        .route("/pages/histories/delete/:id", get(delete_page))
        .route("/update/:id", post(update_page))
        .with_state(pool.clone())
}

pub fn create_block_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/block/create", post(create_block))
        .route("/blocks", get(list_all_blocks))
        .route("/blocks/:app_id", get(list_blocks))
        .route("/block/list/:app_id", get(list_blocks))
        .route("/block/detail/:id", get(get_block))
        .route("/block/update/:id", post(update_block))
        .route("/block/delete/:id", get(delete_block))
        .route("/block/deploy", post(deploy_block))
        .route("/block-categories", get(list_block_categories))
        .route("/block-categories", post(create_block_category))
        .route("/block-groups", get(list_block_groups))
        .with_state(pool.clone())
}

pub fn create_schema2code_router() -> Router {
    Router::new().route("/schema2code", post(schema2code))
}

pub fn create_platform_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/api/user/login", post(login))
        .route("/api/user/register", post(register))
        .route("/api/user/me", get(get_current_user))
        .with_state(pool)
}
