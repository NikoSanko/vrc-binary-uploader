use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("converter error: {0}")]
    Converter(String),
    #[error("storage error: {0}")]
    Storage(String),
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;

