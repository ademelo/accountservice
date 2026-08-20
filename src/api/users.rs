use axum::{
    extract::{Path, State},
    Json,
};
use crate::{
    domain::user::User,
    error::AppError,
    services::user_service::UserService,
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = OK, description = "Success", body = [User], content_type = "application/json"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = UserService::get_all_users(&state.pool).await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = OK, description = "User found", body = User),
        (status = NOT_FOUND, description = "User not found")
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let user = UserService::get_user_by_id(&state.pool, id).await?;
    Ok(Json(user))
}
