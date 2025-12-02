use async_trait::async_trait;
use log::info;

use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

#[async_trait]
pub trait Converter: Send + Sync {
    async fn png_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>>;
}

pub struct DefaultConverter;

impl DefaultConverter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Converter for DefaultConverter {
    async fn png_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>> {
        info!("Converting image to DDS format (size: {} bytes)", image.len());

        if image.is_empty() {
            return Err(InfrastructureError::Converter(
                "input image is empty".to_string(),
            ));
        }

        // TODO: 実装を追加 - 実際の変換処理
        Ok(image.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::{Converter, DefaultConverter};

    #[tokio::test]
    async fn 画像が空ならエラーを返す() {
        let converter = DefaultConverter::new();
        let result = converter.png_to_dds(&[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn 入力画像が存在する場合に成功を返す() {
        let converter = DefaultConverter::new();
        let input = vec![1, 2, 3];
        let result = converter.png_to_dds(&input).await.unwrap();
        assert_eq!(result, input);
    }
}
