use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcpuError {
    #[error("Error reading system info: {0}")]
    IO(#[from] std::io::Error),
    #[error("CPU Info parse error: {0}")]
    Cpu(&'static str),
    #[error("RAM Info parse error: {0}")]
    Ram(&'static str),
    #[error("Integer parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError)
}

impl IntoResponse for RcpuError {
    fn into_response(self) -> Response {
        // Return 500 with a JSON response
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(crate::Response {
                cpu: None,
                ram: None,
                err: Some(format!("Internal Server Error: {}", self)),
            })
        ).into_response()
    }
}