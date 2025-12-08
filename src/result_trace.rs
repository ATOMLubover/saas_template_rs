use tracing::{debug, error, warn};

// A streamable extension trait for Result to log errors at different levels.
pub trait ResultTrace<T, E>
where
    E: std::error::Error,
{
    fn trace_error(self) -> Result<T, E>;
    fn trace_warn(self) -> Result<T, E>;
    fn trace_debug(self) -> Result<T, E>;
}

impl<T, E> ResultTrace<T, E> for Result<T, E>
where
    E: std::error::Error,
{
    #[tracing::instrument(skip(self))]
    fn trace_error(self) -> Result<T, E> {
        if let Err(err) = &self {
            error!("Error: {}", err);
        }

        self
    }

    #[tracing::instrument(skip(self))]
    fn trace_warn(self) -> Result<T, E> {
        if let Err(err) = &self {
            warn!("Warning: {}", err);
        }

        self
    }

    #[tracing::instrument(skip(self))]
    fn trace_debug(self) -> Result<T, E> {
        if let Err(err) = &self {
            debug!("Debug: {}", err);
        }

        self
    }
}
