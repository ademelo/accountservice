use sqlx::PgPool;
use crate::domain::user::User;
use crate::error::AppError;
use crate::repositories::user_repo::UserRepository;

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
        UserRepository::find_all(pool).await
    }

    pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, AppError> {
        UserRepository::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create_user(
        pool: &PgPool,
        user: User,
    ) -> Result<User, AppError> {
        UserRepository::create(
            pool,
            &user.first_name,
            &user.last_name,
            &user.country
        ).await
    }
}
