use async_trait::async_trait;
use axum::body::Bytes;
use log::info;
use reqwest::Client;

use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn upload_file(&self, signed_url: &str, file_data: &[u8]) -> InfrastructureResult<()>;
}

pub struct DefaultStorage;

impl DefaultStorage {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Storage for DefaultStorage {
    async fn upload_file(&self, signed_url: &str, file_data: &[u8]) -> InfrastructureResult<()> {
        info!(
            "Uploading file to storage (signed_url: {}, size: {} bytes)",
            signed_url,
            file_data.len()
        );

        if signed_url.trim().is_empty() {
            return Err(InfrastructureError::Storage(
                "signed url is missing".to_string(),
            ));
        }

        if file_data.is_empty() {
            return Err(InfrastructureError::Storage(
                "file data is empty".to_string(),
            ));
        }

        let client = Client::new();
        let response = client
            .put(signed_url)
            .header("Content-Type", "application/octet-stream")
            .body(Bytes::copy_from_slice(file_data))
            .send()
            .await
            .map_err(|e| InfrastructureError::Storage(format!("failed to send request: {e}")))?;

        response
            .error_for_status()
            .map_err(|e| InfrastructureError::Storage(format!("upload failed: {e}")))?;

        info!("Upload succeeded");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DefaultStorage, Storage};

    #[tokio::test]
    async fn 署名付きurlが空ならエラーを返す() {
        let storage = DefaultStorage::new();
        let result = storage.upload_file("", &[1, 2, 3]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn 渡されたファイルが空ならエラーを返す() {
        let storage = DefaultStorage::new();
        let result = storage.upload_file("https://example.com", &[]).await;
        assert!(result.is_err());
    }

    // TODO: 署名付きアップロードURLを使用
    //#[tokio::test]
    //async fn 入力値が正しい場合に成功を返す() {
    //    let storage = DefaultStorage::new();
    //    let result = storage.upload_file("https://example.com", &[1, 2, 3]).await;
    //    assert!(result.is_ok());
    //}
}
