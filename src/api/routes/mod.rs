use super::handlers::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::MySqlPool;

pub fn create_app_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/apps/list", get(get_all_apps))
        .route("/apps/page", get(get_apps_by_page))
        .route("/apps/detail/:id", get(get_app_detail))
        .route("/apps/create", post(create_app))
        .route("/apps/update/:id", post(update_app))
        .route("/apps/delete/:id", get(delete_app))
        .route("/apps/:id", get(get_app_by_id))
        .route("/apps/:id", delete(delete_app))
        .route("/apps/i18n/:id", post(update_app_i18n))
        .route("/apps/extension/list", get(get_app_extensions))
        .route("/apps/extension", get(get_app_extension_by_id))
        .route("/apps/extension/create", post(create_app_extension))
        .route("/apps/extension/update", post(update_app_extension))
        .route("/apps/extension/delete", get(delete_app_extension))
        .route("/pages/list/:aid", get(get_all_pages))
        .route("/pages/detail/:id", get(get_page_detail))
        .route("/pages/create", post(create_page))
        .route("/pages/update/:id", post(update_page))
        .route("/pages/delete/:id", get(delete_page))
        .route("/pages/deploy", post(deploy_page))
        .route("/pages/copy", post(copy_page))
        .route("/pageHistory/restore", post(restore_page_history))
        .route("/pages/histories", get(ext_get_page_histories))
        .route(
            "/pages/histories/detail/:history_id",
            get(ext_get_page_history_detail),
        )
        .route("/pages/history/create", post(ext_create_page_history))
        .route(
            "/pages/histories/delete/:history_id",
            get(ext_delete_page_history),
        )
        .route("/pages/history/published", post(get_published_pages))
        .route("/pages/histories/find", get(find_page_histories_by_name))
        .route("/preview/metadata", get(get_preview_metadata))
        .route("/sources/list/:aid", get(ext_get_sources))
        .route("/sources/detail/:id", get(ext_get_source_detail))
        .route("/sources/create", post(ext_create_source))
        .route("/sources/update/:id", post(ext_update_source))
        .route("/sources/delete/:id", get(ext_delete_source))
        .route("/source_tpl", get(get_source_tpl))
        .route("/app-template/list", get(get_app_templates))
        .route("/app-template/:id", get(get_app_template))
        .route("/app-template/create", post(create_app_from_template))
        .route("/i18n/entries", get(get_i18n_entries))
        .route("/i18n/langs", get(get_i18n_langs))
        .route("/datasource/list", get(get_datasource_list))
        .route("/i18n/entries/:id", get(get_i18n_entry))
        .route("/i18n/entries/create", post(create_i18n_entry))
        .route(
            "/i18n/entries/batch/create",
            post(create_i18n_entries_batch),
        )
        .route("/i18n/entries/update/:id", post(update_i18n_entry))
        .route("/i18n/entries/bulk/delete", post(delete_i18n_entries_bulk))
        .route(
            "/apps/:id/i18n/entries/update",
            post(update_app_i18n_entries),
        )
        .route(
            "/apps/:id/i18n/entries/multiUpdate",
            post(multi_update_app_i18n),
        )
        .route("/tasks/status/:id", get(get_task_status))
        .route("/ai/chat", post(ai_chat))
        .route("/chat/completions", post(ai_completions))
        .route("/encrypt-key", post(get_encrypt_key))
        .route("/ai/search", post(ai_search))
        .route("/apps/canvas/lock", get(get_canvas_lock))
        .with_state(pool)
}

pub fn create_material_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/block/list", get(get_blocks_list))
        .route("/block/count", get(get_block_count))
        .route("/block/detail/:id", get(get_block_detail))
        .route("/block/create", post(create_block))
        .route("/block/delete/:id", get(delete_block))
        .route("/block", get(get_blocks_paginated))
        .route("/block/tags", get(get_block_tags))
        .route("/block/notgroup/:groupId", get(get_blocks_not_in_group))
        .route("/block-history", get(get_block_history))
        .route("/block-categories", get(get_block_categories))
        .route("/block-categories", post(create_block_category_api))
        .route("/block-categories/:id", put(update_block_category_api))
        .route("/block-categories/:id", delete(delete_block_category_api))
        .route("/block/list2", get(get_blocks_list2))
        .route("/block/tenants", get(get_all_tenants))
        .route("/block/users", get(get_all_users))
        .route("/blocks", get(get_blocks))
        .route("/block/update/:id", post(update_block))
        .route("/block/label", get(get_block_by_label))
        .route("/block/deploy", post(deploy_block))
        .route("/material/list", get(get_materials_list))
        .route("/material/create", post(create_material))
        .route("/material/update/:id", post(update_material))
        .route("/material/delete/:id", delete(delete_material))
        .route("/material/detail/:id", get(get_material_detail))
        .route("/block-groups", get(get_block_groups))
        .route("/block-groups/create", post(create_block_group))
        .route("/block-groups/update/:id", post(update_block_group))
        .route("/block-groups/delete/:id", get(delete_block_group))
        .route("/business-category/list", get(get_business_categories))
        .route(
            "/business-category/find",
            get(get_business_categories_by_group),
        )
        .route("/component-library/list", get(get_component_libraries))
        .route("/component-library/create", post(create_component_library))
        .route(
            "/component-library/update/:id",
            post(update_component_library),
        )
        .route(
            "/component-library/delete/:id",
            delete(delete_component_library),
        )
        .route(
            "/component-library/detail/:id",
            get(get_component_library_detail),
        )
        .route("/component/bundle/create", post(create_component_bundle))
        .route("/component/bundle/split", post(split_component_bundle))
        .route("/component/batch/create", post(create_components_batch))
        .route("/resource/list", get(ext_get_resources))
        .route("/resource/:id", get(get_resource_by_id))
        .route(
            "/resource/find/:resource_group_id",
            get(get_resources_by_group),
        )
        .route("/resource/like", get(search_resources))
        .route("/resource/create", post(create_resource))
        .route("/resource/upload", post(upload_resource))
        .route("/resource/create/batch", post(create_resources_batch))
        .route("/resource/update/:id", put(update_resource))
        .route("/resource/delete/:id", delete(delete_resource))
        .route("/resource/detail/:id", get(get_resource_detail))
        .route("/resource/download/:name", get(download_resource))
        .route("/resource-group/list", get(get_resource_groups))
        .route("/resource-group/:app_id", get(get_resource_groups_by_app))
        .route("/resource-group/create", post(create_resource_group))
        .route("/resource-group/update/:id", put(update_resource_group))
        .route("/resource-group/delete/:id", delete(delete_resource_group))
        .route("/resource-group/detail/:id", get(get_resource_group_detail))
        .route("/model/list", get(get_models))
        .route("/model/find", get(find_model))
        .route("/model/create", post(create_model))
        .route("/model/update/:id", put(update_model))
        .route("/model/delete/:id", delete(delete_model))
        .route("/model/detail/:id", get(get_model_detail))
        .route("/model/table/:id", get(get_model_sql))
        .route("/model/table/list", get(get_all_model_sqls))
        .route("/page-template/create", post(create_page_template))
        .route(
            "/template-basic/bulk/delete",
            post(delete_page_templates_bulk),
        )
        .route("/template-basic/detail/:id", get(get_page_template_detail))
        .route("/template-basic/list", get(get_page_templates))
        .route("/tasks/:id", get(get_task_detail))
        .route("/tasks/status", get(get_tasks_status))
        .with_state(pool)
}

pub fn create_platform_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/platform/list", get(get_all_platforms))
        .route("/platform/:id", get(get_platform_by_id))
        .route("/platform/create", post(create_platform))
        .route("/platform/update/:id", post(update_platform))
        .route("/platform/delete/:id", get(delete_platform))
        .route("/platform-history/list", get(get_platform_histories))
        .route("/platform-history/:id", get(get_platform_history))
        .route("/platform-history/create", post(create_platform_history))
        .route("/platform-history/update/:id", put(update_platform_history))
        .route(
            "/platform-history/delete/:id",
            delete(delete_platform_history),
        )
        .route("/tenant/list", get(get_tenants))
        .route("/tenant/create", post(create_tenant))
        .route("/tenant/update", post(update_tenant))
        .route("/tenant/delete", delete(delete_tenant))
        .route("/user/login", post(login))
        .route("/user/register", post(register))
        .route("/user/me", get(get_current_user))
        .route("/user/info", get(get_user_info))
        .route("/user/tenant", get(get_tenant_info))
        .route("/user/perms", get(get_user_perms))
        .route("/resource", get(get_resources_api))
        .route("/categories", get(get_categories))
        .route("/i18n", get(get_i18n))
        .route("/organizations", get(get_organizations))
        .route("/config", get(get_config))
        .route("/settings", get(get_settings))
        .route("/system", get(get_system_info))
        .route("/menu", get(get_menu))
        .route("/nav", get(get_nav))
        .route("/dashboard", get(get_dashboard))
        .route("/home", get(get_home))
        .route("/stats", get(get_stats))
        .route("/info", get(get_info))
        .route("/notifications", get(get_notifications))
        .route("/messages", get(get_messages))
        .route("/events", get(get_events))
        .with_state(pool)
}

pub fn create_schema2code_router() -> Router {
    Router::new().route("/schema2code", post(schema2code))
}

pub fn create_v1_app_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/apps/schema/:id", get(get_app_schema_v1))
        .route(
            "/apps/schema/components/:id",
            get(get_app_schema_components),
        )
        .with_state(pool)
}
