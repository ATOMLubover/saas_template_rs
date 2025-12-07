use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

use crate::service::result::{ServiceError, ServiceResult};

#[derive(Debug, Serialize, ToSchema)]
pub struct HttpResult<T>
where
    T: Serialize,
{
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> HttpResult<T>
where
    T: Serialize,
{
    pub fn new(code: StatusCode, message: Option<String>, data: Option<T>) -> Self {
        Self {
            code: code.into(),
            message,
            data,
        }
    }
}

impl<T> From<ServiceError> for HttpResult<T>
where
    T: Serialize,
{
    fn from(err: ServiceError) -> Self {
        // For trae level only, we log the error internally but do not expose it to clients.
        tracing::trace!("ServiceError encountered: {}", err);

        // We never expose internal error details to clients.
        match err {
            ServiceError::Generic { code, message } => HttpResult::new(
                StatusCode::from_u16(code).unwrap_or(StatusCode::BAD_REQUEST),
                Some(message),
                None,
            ),
            ServiceError::JwtCodec(_) => {
                HttpResult::new(StatusCode::INTERNAL_SERVER_ERROR, None, None)
            }
            ServiceError::PasswordHash(_) => {
                HttpResult::new(StatusCode::INTERNAL_SERVER_ERROR, None, None)
            }
            ServiceError::Cache(_) => {
                HttpResult::new(StatusCode::INTERNAL_SERVER_ERROR, None, None)
            }
            ServiceError::Database(_) => {
                HttpResult::new(StatusCode::INTERNAL_SERVER_ERROR, None, None)
            }
        }
    }
}

impl<T> From<ServiceResult<T>> for HttpResult<T>
where
    T: Serialize,
{
    fn from(service_result: ServiceResult<T>) -> Self {
        // Only log in debug level to avoid cluttering logs.
        match service_result {
            Ok(value) => HttpResult {
                code: value.code,
                message: value.message,
                data: value.data,
            },
            Err(err) => Self::from(err),
        }
    }
}

impl<T> IntoResponse for HttpResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = Json(self);

        (status, body).into_response()
    }
}
