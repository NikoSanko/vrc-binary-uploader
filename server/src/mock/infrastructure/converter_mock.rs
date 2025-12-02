use async_trait::async_trait;
use std::sync::Arc;

use crate::infrastructure::Converter;
use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

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
    async fn png_to_dds(&self, image: &[u8]) -> InfrastructureResult<Vec<u8>> {
        (self.responder)(image)
    }
}
