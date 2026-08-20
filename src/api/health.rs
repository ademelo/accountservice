use axum::Json;
use serde_json::{json, Value};

#[utoipa::path(
    method(get, head),
    path = "/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn health() -> Json<Value> {
    Json(json!({ "status": "OK" }))
}
