use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "CM001", "数据库错误")
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "CM009", msg.as_str()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "CM002", msg.as_str()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "CM001", "内部服务器错误")
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "CM003", "未授权"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "CM004", msg.as_str()),
        };

        let body = Json(json!({
            "data": null,
            "code": code,
            "message": message,
            "isSuccess": false,
            "error": {
                "code": code,
                "message": message
            },
            "errMsg": message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
