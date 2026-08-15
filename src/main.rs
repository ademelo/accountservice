use std::io;
use std::net::Ipv4Addr;
use axum;
use axum::{Json};
use tokio_postgres::NoTls;
use tokio;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations/postgres");
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Account Service API",
        version = "1.0.0",
        description = "API for the Account Service"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let (mut client, connection) = tokio_postgres::connect(
        "host=localhost user=username password=password dbname=postgres",
        NoTls
    ).await.expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    embedded::migrations::runner().run_async(&mut client).await.expect("Failed to run migrations");

    let router = app();

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("failed to start");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, router).await
}

fn app() -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .routes(routes!(users))
        .routes(routes!(health))
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}

#[utoipa::path(get, path = "/", responses((status = OK, body = str)))]
async fn hello() -> &'static str {
    "Hello from accountservice"
}

#[utoipa::path(
    get,
    path = "/users",
    responses((status = OK, description = "Success", body = str, content_type = "application/json"))
)]
async fn users() -> Json<Value> {
    let user_list = vec![1, 2, 3];
    Json(json!(user_list))
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_hello_handler() {
        let response = hello().await;
        assert_eq!(response, "Hello from accountservice");
    }

    #[tokio::test]
    async fn test_health_handler() {
        let Json(body) = health().await;
        assert_eq!(body, json!({ "status": "OK" }));
    }

    #[tokio::test]
    async fn test_hello_route() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello from accountservice");
    }

    #[tokio::test]
    async fn test_health_route() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "status": "OK" }));
    }

    #[tokio::test]
    async fn test_openapi_json() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/apidoc/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["info"]["title"], "Account Service API");
        assert!(body["paths"].get("/").is_some());
        assert!(body["paths"].get("/health").is_some());
    }
}
