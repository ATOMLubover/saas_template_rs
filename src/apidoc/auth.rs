use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::auth::login_user,
    ),
    tags(
        (name = "Auth", description = "Authentication endpoint")
    )
)]
pub struct AuthApiDoc;
