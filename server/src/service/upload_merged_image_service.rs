use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;

use crate::infrastructure::{Converter, Storage};
use crate::model::{Image, ImageError};
use crate::service::error::{ServiceError, ServiceResult};

#[async_trait]
pub trait UploadMergedImageService: Send + Sync {
    async fn execute(&self, signed_url: &str, images: &[Vec<u8>]) -> ServiceResult<()>;
}

pub struct UploadMergedImageServiceImpl {
    converter: Arc<dyn Converter>,
    storage: Arc<dyn Storage>,
}

impl UploadMergedImageServiceImpl {
    pub fn new(converter: Arc<dyn Converter>, storage: Arc<dyn Storage>) -> Self {
        Self { converter, storage }
    }
}

#[async_trait]
impl UploadMergedImageService for UploadMergedImageServiceImpl {
    async fn execute(&self, signed_url: &str, images: &[Vec<u8>]) -> ServiceResult<()> {
        if signed_url.trim().is_empty() {
            return Err(ServiceError::Validation(
                "signed url must not be empty".to_string(),
            ));
        }

        if images.is_empty() {
            return Err(ServiceError::Validation(
                "images must not be empty".to_string(),
            ));
        }

        info!(
            "Starting upload_merged_image_service (image count: {})",
            images.len()
        );

        // 各画像をモデルに変換してからDDSに変換
        let mut dds_data_list = Vec::new();
        for (index, image_bytes) in images.iter().enumerate() {
            // 画像データをモデルに変換（バリデーション付き）
            let image_model = Image::try_from(image_bytes.as_slice()).map_err(|e| {
                match e {
                    ImageError::EmptyData => ServiceError::Validation(format!(
                        "image at index {} is empty",
                        index
                    )),
                    ImageError::DecodeError(msg) => ServiceError::Validation(format!(
                        "failed to decode image at index {}: {}",
                        index, msg
                    )),
                    ImageError::InvalidDimensions { width, height } => {
                        ServiceError::Validation(format!(
                            "image at index {}: dimensions must be multiples of 4 (width: {}, height: {})",
                            index, width, height
                        ))
                    }
                }
            })?;

            let dds_data = self
                .converter
                .jpeg_to_dds(image_model.as_bytes())
                .await
                .map_err(|e| {
                    error!("Failed to convert image {} to dds: {}", index, e);
                    ServiceError::from(e)
                })?;

            dds_data_list.push(dds_data);
        }

        // 独自形式にまとめる
        let merged_data = create_merged_format(&dds_data_list)?;

        // ストレージにアップロード
        self.storage
            .upload_file(signed_url, &merged_data)
            .await
            .map_err(|e| {
                error!("Failed to upload merged file to storage: {}", e);
                ServiceError::from(e)
            })?;

        info!("Upload merged image succeeded");
        Ok(())
    }
}

/// 複数のDDSデータを独自形式にまとめる
///
/// フォーマット:
/// - Header: Texture Count (4byte, Int32, Little Endian)
/// - Index: Data Size List (4byte * N, 各DDSデータのサイズ)
/// - Data: Concatenated DDS Binaries
fn create_merged_format(dds_data_list: &[Vec<u8>]) -> ServiceResult<Vec<u8>> {
    let count = dds_data_list.len();

    if count == 0 {
        return Err(ServiceError::Validation(
            "dds data list must not be empty".to_string(),
        ));
    }

    // Header Section: Texture Count (4byte, Int32, Little Endian)
    let mut result = Vec::new();
    result.extend_from_slice(&(count as i32).to_le_bytes());

    // Index Section: Data Size List (4byte * N)
    for dds_data in dds_data_list {
        let size = dds_data.len() as i32;
        result.extend_from_slice(&size.to_le_bytes());
    }

    // Data Section: Concatenated DDS Binaries
    for dds_data in dds_data_list {
        result.extend_from_slice(dds_data);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::infrastructure::{MockConverter, MockStorage};

    #[tokio::test]
    async fn 空のurlならエラーを返す() {
        let service = UploadMergedImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let result = service.execute("", &[vec![1]]).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
    }

    #[tokio::test]
    async fn 画像リストが空ならエラーを返す() {
        let service = UploadMergedImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let result = service.execute("https://example.com", &[]).await;
        assert!(matches!(result, Err(ServiceError::Validation(_))));
    }

    #[tokio::test]
    async fn 変換に失敗したならエラーを返す() {
        let service = UploadMergedImageServiceImpl::new(
            Arc::new(MockConverter::fail("fail")),
            Arc::new(MockStorage::succeed()),
        );
        let result = service.execute("https://example.com", &[vec![1]]).await;
        assert!(matches!(result, Err(ServiceError::Infrastructure(_))));
    }

    #[tokio::test]
    async fn 正しい入力値なら成功を返す() {
        let service = UploadMergedImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::succeed()),
        );
        let result = service
            .execute("https://example.com", &[vec![1], vec![2]])
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn ストレージのアップロードに失敗したならエラーを返す() {
        let service = UploadMergedImageServiceImpl::new(
            Arc::new(MockConverter::succeed()),
            Arc::new(MockStorage::fail("upload failed")),
        );
        let result = service.execute("https://example.com", &[vec![1]]).await;
        assert!(matches!(result, Err(ServiceError::Infrastructure(_))));
    }

    #[test]
    fn 独自形式のバイナリが正しく生成される() {
        let dds_data_list = vec![
            vec![1, 2, 3, 4, 5],      // 5 bytes
            vec![6, 7, 8, 9, 10, 11], // 6 bytes
        ];

        let result = create_merged_format(&dds_data_list).unwrap();

        // Header: count = 2 (4 bytes, little endian)
        assert_eq!(result[0..4], [2, 0, 0, 0]);

        // Index: size1 = 5 (4 bytes, little endian)
        assert_eq!(result[4..8], [5, 0, 0, 0]);

        // Index: size2 = 6 (4 bytes, little endian)
        assert_eq!(result[8..12], [6, 0, 0, 0]);

        // Data: first DDS
        assert_eq!(result[12..17], [1, 2, 3, 4, 5]);

        // Data: second DDS
        assert_eq!(result[17..23], [6, 7, 8, 9, 10, 11]);

        // 全体のサイズ: 4 (header) + 8 (index) + 11 (data) = 23
        assert_eq!(result.len(), 23);
    }

    #[test]
    fn 空のリストならエラーを返す() {
        let result = create_merged_format(&[]);
        assert!(matches!(result, Err(ServiceError::Validation(_))));
    }
}
