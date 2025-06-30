
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RcpuError {
    #[error("Error reading system info: {0}")]
    IO(#[from] std::io::Error),
    #[error("CPU Info parse error: {0}")]
    Cpu(&'static str),
    #[error("RAM Info parse error: {0}")]
    Ram(&'static str),
    #[error("Memory check exited with non-zero exit code: {0}")]
    Disk(i32),
    #[error("Integer parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Interior nul byte found: {0}")]
    Nul(#[from] std::ffi::NulError),
    #[error("Invalid request")]
    InvalidRequest
}

impl IntoResponse for RcpuError {
    fn into_response(self) -> Response {
        match self {
            // return 404
            RcpuError::InvalidRequest => StatusCode::NOT_FOUND.into_response(),
            
            // otherwise return 500 with a JSON response
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}