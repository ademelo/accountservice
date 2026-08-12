use std::io;
use std::net::Ipv4Addr;
use axum;
use axum::{Json};
use axum::routing::MethodRouter;
use tokio;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::AllowMethods;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .route("/health", MethodRouter::new().get(health))
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST,8080))
        .await
        .expect("failed to start");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, router).await
}

#[utoipa::path(get, path = "/", responses((status = OK, body = str)))]
async fn hello() -> &'static str {
    "Hello from accountservice"
}

#[utoipa::path(
    method(get, head),
    path = "/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
async fn health() -> Json<Value> {
    Json(json!({ "status": "OK" }))
}
