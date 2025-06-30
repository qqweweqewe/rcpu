mod info;
mod error;

use axum::{
    extract::Path, routing::get, Json, Router
};
use serde_json::{json, Value};

use crate::error::RcpuError;

// GET "/cpu"
async fn cpu() -> Result<Json<Value>, RcpuError> {
    Ok(Json(json!(
        {
            "cpu": info::cpu::get_load()?
        }
    )))
}

// GET "/ram"
async fn ram() -> Result<Json<Value>, RcpuError> {
    Ok(Json(json!(
        {
            "ram": info::ram::get_busy()?
        }
    )))

}

// Updated disk handler
async fn disk(Path(format): Path<String>) -> Result<Json<Value>, RcpuError> {

    match format.as_str() {
        "percentage" => {
            Ok(Json(json!(
                { 
                    "percentage": info::disk::percentage()? 
                }
            )))
        }
        "bytes" => {
            let (total, used) = info::disk::bytes()?;
            Ok(Json(json!(
                { 
                    "used": used, 
                    "total": total 
                }
            )))
        }
        _ => Err(RcpuError::InvalidRequest),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/cpu", get(cpu))
        .route("/ram", get(ram))
        .route("/disk", get(disk));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
