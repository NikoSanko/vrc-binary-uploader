use log::{info, error};
use crate::infrastructure::{Storage, Converter};
use crate::service::error::{ServiceError, ServiceResult};

pub struct UploadSingleImageService;

impl UploadSingleImageService {
    pub async fn execute(signed_url: &str, image: &[u8]) -> ServiceResult<()> {
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

        // NOTE: dxt1形式のddsに変換
        let dds_data = Converter::png_to_dds(image).await.map_err(|e| {
            error!("Failed to convert image to dds: {}", e);
            ServiceError::from(e)
        })?;

        // NOTE: ストレージにアップロード
        Storage::upload_file(signed_url, &dds_data).await.map_err(|e| {
            error!("Failed to upload file to storage: {}", e);
            ServiceError::from(e)
        })?;

        Ok(())
    }
}