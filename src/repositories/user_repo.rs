use sqlx::PgPool;
use crate::domain::user::User;
use crate::error::AppError;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, country FROM users"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, country FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        pool: &PgPool,
        first_name: &str,
        last_name: &str,
        country: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (first_name, last_name, country) VALUES ($1, $2, $3) RETURNING id, first_name, last_name, country"
        )
        .bind(first_name)
        .bind(last_name)
        .bind(country)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
