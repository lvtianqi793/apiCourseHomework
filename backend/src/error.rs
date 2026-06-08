use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("请求参数错误：{0}")]
    BadRequest(String),
    #[error("未登录或登录已过期")]
    Unauthorized,
    #[error("没有权限执行该操作")]
    Forbidden,
    #[error("资源不存在")]
    NotFound,
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
    #[error("文件错误：{0}")]
    Io(#[from] std::io::Error),
    #[error("上传错误：{0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error("内部服务错误：{0}")]
    Internal(String),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::Io(_) | Self::Multipart(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "code": status.as_u16() as i32,
            "message": self.to_string(),
            "data": null
        }));

        (status, body).into_response()
    }
}
