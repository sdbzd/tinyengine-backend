use axum::Json;
use crate::api::models::ApiResponse;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema2CodeRequest {
    pub platform: Option<i64>,
    pub app: i64,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub schema: serde_json::Value,
    pub block_schema: Option<serde_json::Value>,
    pub content_blocks: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub panel_name: String,
    pub panel_value: String,
    pub panel_type: String,
    pub prettier_opts: PrettierOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierOptions {
    pub print_width: i32,
    pub semi: bool,
    pub single_quote: bool,
    pub trailing_comma: String,
}

pub fn generate_code(request: Schema2CodeRequest) -> ApiResponse<Vec<CodeFile>> {
    let schema = request.page_info.schema;
    let component_name = schema
        .get("componentName")
        .and_then(|v| v.as_str())
        .unwrap_or("Page");

    let files = vec![
        CodeFile {
            panel_name: format!("{}.vue", component_name),
            panel_value: generate_vue_file(&schema),
            panel_type: "vue".to_string(),
            prettier_opts: PrettierOptions {
                print_width: 120,
                semi: false,
                single_quote: true,
                trailing_comma: "none".to_string(),
            },
        },
        CodeFile {
            panel_name: "index.js".to_string(),
            panel_value: generate_router_file(&schema),
            panel_type: "js".to_string(),
            prettier_opts: PrettierOptions {
                print_width: 120,
                semi: false,
                single_quote: true,
                trailing_comma: "none".to_string(),
            },
        },
    ];

    ApiResponse::success(files)
}

fn generate_vue_file(schema: &serde_json::Value) -> String {
    let component_name = schema
        .get("componentName")
        .and_then(|v| v.as_str())
        .unwrap_or("Page");

    let css = schema.get("css").and_then(|v| v.as_str()).unwrap_or("");

    let children_str = generate_template_children(schema);

    format!(
        r#"<template>
  <div class="{}">
{}
  </div>
</template>

<script setup>
import {{ ref }} from 'vue'

// Component logic
</script>

<style scoped>
{}
</style>
"#,
        component_name.to_lowercase(),
        children_str,
        css
    )
}

fn generate_template_children(schema: &serde_json::Value) -> String {
    let children = schema
        .get("children")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|child| {
                    let tag = child
                        .get("componentName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("div");
                    format!("    <{} />", tag.to_lowercase())
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    children
}

fn generate_router_file(schema: &serde_json::Value) -> String {
    let component_name = schema
        .get("componentName")
        .and_then(|v| v.as_str())
        .unwrap_or("Page");

    let lc_name = component_name.to_lowercase();
    format!(
        r#"import {{ createRouter, createWebHistory }} from 'vue-router'
import {{ {component_name} }} from './{}.vue'

const routes = [
  {{
    path: '/',
    name: '{lc}',
    component: {component_name}
  }}
]

const router = createRouter({{
  history: createWebHistory(),
  routes
}})

export default router
"#,
        lc = lc_name
    )
}

pub async fn schema2code(
    Json(request): Json<Schema2CodeRequest>,
) -> Result<Json<ApiResponse<Vec<CodeFile>>>> {
    let files = generate_code(request);
    Ok(Json(files))
}
