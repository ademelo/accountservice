#[utoipa::path(
    get,
    path = "/",
    responses((status = OK, body = str))
)]
pub async fn hello() -> &'static str {
    "Hello from accountservice"
}
