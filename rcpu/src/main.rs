mod info;

use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;

// Define a response struct
#[derive(Serialize)]
struct Response {
    message: String,
}

// GET "/cpu"
async fn cpu() -> Json<Response> {
    Json(Response {
        message: "WIP".to_string(),
    })
}

// GET "/ram"
async fn ram() -> Json<Response> {
    Json(Response {
        message: "WIP".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // let app = Router::new()
    //     .route("/cpu", get(cpu))
    //     .route("/ram", get(ram));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    
    // println!("Server running on http://{}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();

    for i in (0..5) {
        println!("---------------");
        let load = info::cpu::get_cpu_load().unwrap();
        println!("LOAD: {:?}%", load)
    }
}
