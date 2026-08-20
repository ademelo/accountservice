use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use sqlx::postgres::PgPoolOptions;
use accountservice::{create_router, AppState, DatabaseConfig};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations/postgres");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_config = DatabaseConfig::load().expect("Failed to build settings");
    println!("Settings: {:#?}", database_config);

    let (mut client, connection) = tokio_postgres::connect(
        database_config.connection_string().as_str(),
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Failed to run migrations");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_config.database_url())
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

    let state = AppState::new(pool);
    let router = create_router(state);

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("failed to start");

    println!("Server running at http://127.0.0.1:8080");

    axum::serve(listener, router).await?;
    Ok(())
}
