use crate::infrastructure::InfrastructureError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
