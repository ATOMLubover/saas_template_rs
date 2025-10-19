use crate::service::result::{ServiceResult, succeed};

pub async fn health_check() -> ServiceResult<String> {
    // Add some dependency checks here if needed in the future.

    Ok(succeed()
        .with_message("Pong from server.")
        .with_code(200)
        .with_data("A test payload".to_string()))
}
