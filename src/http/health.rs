
use crate::http::result::HttpResult;
use crate::model::health::HealthCheck;
use crate::service;

pub async fn health_check() -> HttpResult<HealthCheck> {
    service::health::health_check().await.into()
}
