use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
}
