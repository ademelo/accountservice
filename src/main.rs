use axum;
use axum::{Json, Router};
use axum::routing::get;
use axum_swagger_ui::swagger_ui;
use tokio;
use serde_json::{json, Value};
use axum_openapi3::{
    build_openapi, // function for building the openapi spec
    endpoint,      // macro for defining endpoints
    reset_openapi, // function for cleaning the openapi cache (mostly used for testing)
    AddRoute,      // `add` method for Router to add routes also to the openapi spec
};


#[tokio::main]
async fn main() {
    let doc_url = "swagger/openapi.json";
    let app = Router::new()
        .route("/swagger", get(|| async { swagger_ui(doc_url) }))
        //.route(doc_url, get(|| async { include_str!("openapi.json") }))
        .route("/", get(hello))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind server");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

//#[endpoint(method = "POST", path = "/todos", description = "Insert a new todo")]
async fn hello() -> &'static str {
    "Hello from accountservice"
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "OK" }))
}
