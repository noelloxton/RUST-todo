mod handler;
mod model;
mod response;
mod route;

use axum::{response::IntoResponse, routing::get, Json, Router};

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Endpoint is healthy!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthchecker", get(health_check_handler));

    println!("Server started successfully at port 8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
