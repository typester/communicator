#[derive(Debug, uniffi::Error, thiserror::Error)]
pub enum CommunicatorError {
    #[error("internal error: {0}")]
    InternalError(String),

    #[error("database migrate error: {0}")]
    MigrateError(String),
}

impl From<uniffi::UnexpectedUniFFICallbackError> for CommunicatorError {
    fn from(value: uniffi::UnexpectedUniFFICallbackError) -> Self {
        tracing::error!(?value, "uniffi ffi error");
        Self::InternalError(value.to_string())
    }
}

impl From<anyhow::Error> for CommunicatorError {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<tokio::task::JoinError> for CommunicatorError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<sqlx::Error> for CommunicatorError {
    fn from(value: sqlx::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for CommunicatorError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::MigrateError(value.to_string())
    }
}
