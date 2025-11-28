use thiserror::Error;
use crate::infrastructure::InfrastructureError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;