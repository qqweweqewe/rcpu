mod info;
mod error;

use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;

use crate::error::RcpuError;

#[derive(Serialize)]
struct Response {
    cpu: Option<String>,
    ram: Option<String>,
    err: Option<String>
}

// GET "/stats"
async fn stats() -> Result<Json<Response>, RcpuError> {
    let cpu_load = info::cpu::get_load()?;
    let ram_used = info::ram::get_busy()?;

    Ok(Json(Response {
        cpu: Some(format!("{}", cpu_load)),
        ram: Some(format!("{}", ram_used)),
        err: None
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/stats", get(stats));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
