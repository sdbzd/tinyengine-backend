mod config;
mod db;
mod error;
mod api;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tinyengine_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_config = config::AppConfig::new()
        .unwrap_or_else(|_| config::AppConfig {
            database: config::DatabaseConfig {
                host: "localhost".to_string(),
                port: 3306,
                username: "root".to_string(),
                password: "".to_string(),
                name: "tinyengine".to_string(),
                max_connections: 10,
            },
            redis: config::RedisConfig {
                host: "localhost".to_string(),
                port: 6379,
                password: "".to_string(),
                db: 0,
            },
            app: config::ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                env: "development".to_string(),
            },
            log: config::LogConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        });

    let pool = db::create_pool(&app_config).await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_router = api::routes::create_app_router(pool.clone());
    let v1_app_router = api::routes::create_v1_app_router(pool.clone());
    let page_router = api::routes::create_page_router(pool.clone());
    let block_router = api::routes::create_block_router(pool.clone());
    let schema2code_router = api::routes::create_schema2code_router();
    let platform_router = api::routes::create_platform_router(pool.clone());

    let app = Router::new()
        .nest("/app-center/api", app_router)
        .nest("/app-center/v1/api", v1_app_router)
        .nest("/app-center/api", page_router)
        .nest("/material-center/api", block_router)
        .nest("/material-center/api", schema2code_router)
        .nest("/platform-center", platform_router)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = format!("{}:{}", app_config.app.host, app_config.app.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
