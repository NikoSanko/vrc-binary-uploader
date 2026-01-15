use std::io::Write;

use async_trait::async_trait;
use log::info;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tempfile::Builder;
use tokio::fs;
use tokio::process::Command;

use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

#[async_trait]
pub trait Converter: Send + Sync {
    async fn jpeg_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>>;
    async fn convert(&self, input_path: &Path, output_path: &Path) -> InfrastructureResult<()>;
}

pub struct DefaultConverter;

impl DefaultConverter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Converter for DefaultConverter {
    async fn jpeg_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>> {
        info!(
            "Converting image to DDS format (size: {} bytes)",
            image.len()
        );

        if image.is_empty() {
            return Err(InfrastructureError::Converter(
                "input image is empty".to_string(),
            ));
        }

        // NOTE: 入力用一時ファイル作成
        let temp_input_file = Builder::new()
            .prefix("temp_")
            .suffix(".jpeg")
            .tempfile()
            .map_err(|e| InfrastructureError::Io(e))?;
        temp_input_file
            .as_file()
            .write_all(image)
            .map_err(|e| InfrastructureError::Io(e))?;
        let input_file_path = temp_input_file.path();

        // NOTE: 出力用一時ファイル作成
        let temp_output_file = Builder::new()
            .prefix("converted_")
            .suffix(".dds")
            .tempfile()
            .map_err(|e| InfrastructureError::Io(e))?;
        let output_file_path = temp_output_file.path();

        self.convert(input_file_path, output_file_path).await?;

        // NOTE: 出力用一時ファイルからデータを読み込む
        let dds_data = fs::read(output_file_path)
            .await
            .map_err(|e| InfrastructureError::Io(e))?;

        Ok(dds_data)
    }

    async fn convert(&self, input_path: &Path, output_path: &Path) -> InfrastructureResult<()> {
        let crunch_path: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/bin/crunch");

        if !crunch_path.exists() {
            return Err(InfrastructureError::Converter(format!(
                "crunch command not found: {}",
                crunch_path.display()
            )));
        }

        let output = Command::new(crunch_path)
            .arg("-file")
            .arg(input_path)
            .arg("-fileformat")
            .arg("dds")
            .arg("-dxt1")
            .arg("-quality")
            .arg("255")
            .arg("-out")
            .arg(output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| InfrastructureError::from(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(InfrastructureError::Converter(format!(
                "Crunch command failed with status {}: {}",
                output.status, stderr
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Converter, DefaultConverter};
    use tokio::fs;

    #[tokio::test]
    async fn 画像が空ならエラーを返す() {
        let converter = DefaultConverter::new();
        let result = converter.jpeg_to_dds(&[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn 入力画像が存在する場合に成功を返す() {
        let converter = DefaultConverter::new();
        let input = fs::read("resources/test.jpg").await.unwrap();
        let result = converter.jpeg_to_dds(&input).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
