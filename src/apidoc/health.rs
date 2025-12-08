use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::health::health_check,
    ),
    tags(
        (name = "Health", description = "Health check endpoint")
    )
)]
pub struct HealthApiDoc;
