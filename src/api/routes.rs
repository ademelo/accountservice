use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use crate::{
    api::{health, hello, users},
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Account Service API",
        version = "1.0.0",
        description = "API for the Account Service"
    ),
    paths(
        hello::hello,
        health::health,
        users::list_users,
        users::get_user,
    )
)]
pub struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello::hello))
        .routes(routes!(health::health))
        .routes(routes!(users::list_users))
        .routes(routes!(users::get_user))
        .routes(routes!(users::create_user))
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
        .with_state(state)
}
