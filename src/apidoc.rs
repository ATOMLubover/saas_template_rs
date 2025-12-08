mod auth;
mod health;
mod user;

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/check", api = health::HealthApiDoc),
        (path = "/api/users", api = user::UserApiDoc),
        (path = "/auth", api = auth::AuthApiDoc),
    ),
)]
pub struct ApiDoc;
