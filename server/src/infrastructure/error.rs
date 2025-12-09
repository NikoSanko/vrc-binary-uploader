use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("converter error: {0}")]
    Converter(String),
    #[error("storage error: {0}")]
    Storage(String),
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
