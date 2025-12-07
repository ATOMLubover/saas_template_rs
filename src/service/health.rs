use crate::{
    model::health::HealthCheck,
    service::result::{ServiceResult, accept},
};

pub async fn health_check() -> ServiceResult<HealthCheck> {
    let heath_check = HealthCheck {
        healthy: true,
        status: "OK".to_string(),
        comment: "Service is running smoothly.".to_string(),
    };

    Ok(accept()
        .with_message("Pong from server.")
        .with_data(heath_check))
}
