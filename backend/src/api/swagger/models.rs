use crate::api::auth;
use crate::api::users;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(auth::endpoints::register, auth::endpoints::login, users::endpoints::get_me, users::endpoints::update_jwt),
    tags(
        (name = "Auth", description = "Authentication related endpoints"),
        (name = "Users", description = "User management endpoints")
    )
)]
pub struct Swagger;
