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
    msg: String,
}

// GET "/cpu"
async fn cpu() -> Result<Json<Response>, RcpuError> {
    let load = info::cpu::get_load()?;
    Ok(Json(Response {
        msg: format!("{}", load),
    }))
}

// GET "/ram"
async fn ram() -> Json<Response> {
    Json(Response {
        msg: "WIP".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/cpu", get(cpu))
        .route("/ram", get(ram));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
