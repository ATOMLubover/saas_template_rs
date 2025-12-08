use crate::http::result::HttpResult;
use crate::model::health::HealthCheck;
use crate::service;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HttpResult<HealthCheck>)
    ),
    tag = "Health"
)]
pub async fn health_check() -> HttpResult<HealthCheck> {
    service::health::health_check().await.into()
}
