pub mod error;
mod upload_single_image_service;

pub use error::ServiceError;
pub use upload_single_image_service::{UploadSingleImageService, UploadSingleImageServiceImpl};
