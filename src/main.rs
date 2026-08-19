use std::io;
use std::net::Ipv4Addr;
use axum;
use axum::{Json, extract::State};
use config::{Config, File};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tokio;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
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

#[derive(Debug, Clone)]
struct AppState {
    pool: PgPool,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let settings = Config::builder()
        //.add_source(Environment::default())
        .add_source(File::with_name("config/dev.toml"))
        .build()
        .expect("Failed to build settings");

    let  database_config : DatabaseConfig = settings.try_deserialize().unwrap();

    println!("Settings: {:#?}", database_config);

    let (mut client, connection) = tokio_postgres::connect(
        format!("host={} user={} password={} dbname=postgres",
                database_config.host,
                database_config.user,
                database_config.password).as_str(),
        NoTls
    ).await.expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    embedded::migrations::runner().run_async(&mut client).await.expect("Failed to run migrations");

    let database_url = "postgres://username:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    let mut tx = pool.begin().await.expect("Failed to start transaction");
    sqlx::query("INSERT INTO users (first_name, last_name, country) VALUES ($1, $2, $3)")
        .bind("John")
        .bind("Doe")
        .bind("England")
        .execute(&mut *tx)
        .await
        .expect("Failed to insert user 1");

    sqlx::query("INSERT INTO users (first_name, last_name, country) VALUES ($1, $2, $3)")
        .bind("Toto ")
        .bind("Titi")
        .bind("Spain")
        .execute(&mut *tx)
        .await
        .expect("Failed to insert user 2");

    tx.commit().await.expect("Failed to commit transaction");

    let state =AppState {
        pool,
    };

    let router = app(state);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("failed to start");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, router).await
}

fn app(state: AppState) -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .routes(routes!(users))
        .routes(routes!(health))
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
        .with_state(state)
}

#[utoipa::path(get, path = "/", responses((status = OK, body = str)))]
async fn hello() -> &'static str {
    "Hello from accountservice"
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    country: String,
}
#[utoipa::path(
    get,
    path = "/users",
    responses((status = OK, description = "Success", body = str, content_type = "application/json"))
)]
async fn users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to fetch users");

    //let user_list: Vec<i32> = rows.into_iter().map(|row| row).collect();
    Json(users)
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

    fn test_state() -> AppState {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://username:password@localhost:5432/postgres")
            .expect("Failed to create lazy test database pool");

        AppState { pool }
    }

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
        let app = app(test_state());

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
        let app = app(test_state());

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
        let app = app(test_state());

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
