use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;

use crate::infrastructure::{Converter, Storage};
use crate::model::{Image, ImageError};
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
    pub fn new(converter: Arc<dyn Converter>, storage: Arc<dyn Storage>) -> Self {
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

        // 画像データをモデルに変換（バリデーション付き）
        let image_model = Image::try_from(image).map_err(|e| match e {
            ImageError::EmptyData => {
                ServiceError::Validation("image bytes must not be empty".to_string())
            }
            ImageError::DecodeError(msg) => {
                ServiceError::Validation(format!("failed to decode image: {}", msg))
            }
            ImageError::InvalidDimensions { width, height } => ServiceError::Validation(format!(
                "image dimensions must be multiples of 4 (width: {}, height: {})",
                width, height
            )),
        })?;

        info!("Starting upload_single_image_service");

        let dds_data = self
            .converter
            .jpeg_to_dds(image_model.as_bytes())
            .await
            .map_err(|e| {
                error!("Failed to convert image to dds: {}", e);
                ServiceError::from(e)
            })?;

        self.storage
            .upload_file(signed_url, &dds_data)
            .await
            .map_err(|e| {
                error!("Failed to upload file to storage: {}", e);
                ServiceError::from(e)
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::infrastructure::{MockConverter, MockStorage};
    use tokio::fs;

    #[tokio::test]
    async fn 空のurlならエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = service.execute("", &jpeg_data).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
    }

    #[tokio::test]
    async fn 変換に失敗したならエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::fail("fail")),
            Arc::new(MockStorage::succeed()),
        );
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = service.execute("https://example.com", &jpeg_data).await;
        assert!(matches!(result, Err(ServiceError::Infrastructure(_))));
    }

    #[tokio::test]
    async fn 正しい入力値なら成功を返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = service.execute("https://example.com", &jpeg_data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn ストレージのアップロードに失敗したならエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::fail("upload failed")),
        );
        let jpeg_data = fs::read("resources/4_multiple_size.jpg")
            .await
            .unwrap();
        let result = service.execute("https://example.com", &jpeg_data).await;
        assert!(matches!(result, Err(ServiceError::Infrastructure(_))));
    }

    #[tokio::test]
    async fn 空の画像データならバリデーションエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let result = service.execute("https://example.com", &[]).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
        if let Err(ServiceError::Validation(msg)) = result {
            assert!(msg.contains("image bytes must not be empty"));
        }
    }

    #[tokio::test]
    async fn 無効な画像データならバリデーションエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let invalid_data = vec![0, 1, 2, 3, 4, 5];
        let result = service.execute("https://example.com", &invalid_data).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
        if let Err(ServiceError::Validation(msg)) = result {
            assert!(msg.contains("failed to decode image"));
        }
    }

    #[tokio::test]
    async fn 四の倍数でないサイズの画像ならバリデーションエラーを返す() {
        let service = UploadSingleImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );

        let jpeg_data = fs::read("resources/not_4_multiple_height.jpg")
            .await
            .unwrap();
        let result = service.execute("https://example.com", &jpeg_data).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
        if let Err(ServiceError::Validation(msg)) = result {
            assert!(msg.contains("dimensions must be multiples of 4"));
        }
    }
}
