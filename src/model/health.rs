use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheck {
    pub healthy: bool,
    pub status: String,
    pub comment: String,
}
