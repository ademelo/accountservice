pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repositories;
pub mod services;
pub mod state;

pub use api::routes::{create_router, ApiDoc};
pub use config::DatabaseConfig;
pub use error::AppError;
pub use state::AppState;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    fn test_state() -> AppState {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://username:password@localhost:5432/postgres")
            .expect("Failed to create lazy test database pool");

        AppState::new(pool)
    }

    #[tokio::test]
    async fn test_hello_handler() {
        let response = api::hello::hello().await;
        assert_eq!(response, "Hello from accountservice");
    }

    #[tokio::test]
    async fn test_health_handler() {
        let axum::Json(body) = api::health::health().await;
        assert_eq!(body, json!({ "status": "OK" }));
    }

    #[tokio::test]
    async fn test_hello_route() {
        let app = create_router(test_state());

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
        let app = create_router(test_state());

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
        let app = create_router(test_state());

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
        assert!(body["paths"].get("/users").is_some());
    }
}
