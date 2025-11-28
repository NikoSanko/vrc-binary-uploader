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
