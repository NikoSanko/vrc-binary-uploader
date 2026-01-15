use async_trait::async_trait;
use log::info;
use std::path::Path;
use std::sync::Arc;

use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};
use crate::infrastructure::Converter;

type ConverterFn = dyn Fn(&[u8]) -> InfrastructureResult<Vec<u8>> + Send + Sync;

#[derive(Clone)]
pub struct MockConverter {
    responder: Arc<ConverterFn>,
}

impl MockConverter {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&[u8]) -> InfrastructureResult<Vec<u8>> + Send + Sync + 'static,
    {
        Self {
            responder: Arc::new(handler),
        }
    }

    pub fn succeed() -> Self {
        Self::new(|image| Ok(image.to_vec()))
    }

    pub fn fail(message: impl Into<String>) -> Self {
        let msg = message.into();
        Self::new(move |_| Err(InfrastructureError::Converter(msg.clone())))
    }
}

#[async_trait]
impl Converter for MockConverter {
    async fn jpeg_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>> {
        (self.responder)(image)
    }

    async fn convert(&self, input_path: &Path, output_path: &Path) -> InfrastructureResult<()> {
        info!(
            "Converting image to DDS format (input_path: {}, output_path: {})",
            input_path.display(),
            output_path.display()
        );
        Ok(())
    }
}
