use log::*;
use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

pub struct Converter;

impl Converter {
    pub async fn png_to_dds(image: &[u8]) -> InfrastructureResult<Vec<u8>> {
        info!("Converting image to DDS format (size: {} bytes)", image.len());

        if image.is_empty() {
            return Err(InfrastructureError::Converter(
                "input image is empty".to_string(),
            ));
        }

        // TODO: 実装を追加 - 実際の変換処理
        // 今はサンプルとして入力そのままを返す
        Ok(image.to_vec())
    }
}
