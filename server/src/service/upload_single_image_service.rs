use async_trait::async_trait;
use log::{info, error};
use std::sync::Arc;

use crate::infrastructure::{Converter, Storage};
use crate::service::error::{ServiceError, ServiceResult};

#[async_trait]
pub trait UploadSingleImageService: Send + Sync {
    async fn execute(&self, signed_url: &str, image: &[u8]) -> ServiceResult<()>;
}

pub struct UploadSingleImageServiceImpl {
    converter: Arc<dyn Converter>,
    storage: Arc<dyn Storage>,
}

impl UploadSingleImageServiceImpl {
    pub fn new(
        converter: Arc<dyn Converter>,
        storage: Arc<dyn Storage>,
    ) -> Self {
        Self { converter, storage }
    }
}

#[async_trait]
impl UploadSingleImageService for UploadSingleImageServiceImpl {
    async fn execute(&self, signed_url: &str, image: &[u8]) -> ServiceResult<()> {
        if signed_url.trim().is_empty() {
            return Err(ServiceError::Validation(
                "signed url must not be empty".to_string(),
            ));
        }

        if image.is_empty() {
            return Err(ServiceError::Validation(
                "image bytes must not be empty".to_string(),
            ));
        }

        info!("Starting upload_single_image_service");

        let dds_data = self.converter.png_to_dds(image).await.map_err(|e| {
            error!("Failed to convert image to dds: {}", e);
            ServiceError::from(e)
        })?;

        self.storage.upload_file(signed_url, &dds_data).await.map_err(|e| {
            error!("Failed to upload file to storage: {}", e);
            ServiceError::from(e)
        })?;

        Ok(())
    }
}