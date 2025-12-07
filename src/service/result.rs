use serde::Serialize;
use thiserror::Error;

pub type ServiceResult<T> = Result<ServiceValue<T>, ServiceError>;

pub type InterResult<T> = Result<T, ServiceError>;

pub(super) fn accept<T>() -> ServiceValue<T>
where
    T: Serialize,
{
    ServiceValue::<T>::default()
}

pub(super) fn reject<T, M>(message: M) -> ServiceValue<T>
where
    T: Serialize,
    M: Into<String>,
{
    ServiceValue::<T>::default()
        .with_code(400)
        .with_message(message)
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Generic service error - code: {code}, message: {message}")]
    Generic { code: u16, message: String },

    #[error("JWT encoding/decoding error: {0}")]
    JwtCodec(#[from] jsonwebtoken::errors::Error),
    #[error("Password hash error: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Serialize)]
pub struct ServiceValue<T = ()>
where
    T: Serialize,
{
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ServiceValue<T>
where
    T: Serialize,
{
    #[inline]
    pub fn default() -> Self {
        Self {
            code: 200,
            message: None,
            data: None,
        }
    }

    #[inline]
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = code;

        self
    }

    #[inline]
    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());

        self
    }

    #[inline]
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);

        self
    }
}
