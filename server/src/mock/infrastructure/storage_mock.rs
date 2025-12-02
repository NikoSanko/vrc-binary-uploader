use async_trait::async_trait;
use std::sync::Arc;

use crate::infrastructure::Storage;
use crate::infrastructure::error::{InfrastructureError, InfrastructureResult};

type StorageFn = dyn Fn(&str, &[u8]) -> InfrastructureResult<()> + Send + Sync;

#[derive(Clone)]
pub struct MockStorage {
    responder: Arc<StorageFn>,
}

impl MockStorage {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&str, &[u8]) -> InfrastructureResult<()> + Send + Sync + 'static,
    {
        Self {
            responder: Arc::new(handler),
        }
    }

    pub fn succeed() -> Self {
        Self::new(|_, _| Ok(()))
    }

    pub fn fail(message: impl Into<String>) -> Self {
        let msg = message.into();
        Self::new(move |_, _| Err(InfrastructureError::Storage(msg.clone())))
    }
}

#[async_trait]
impl Storage for MockStorage {
    async fn upload_file(&self, signed_url: &str, file_data: &[u8]) -> InfrastructureResult<()> {
        (self.responder)(signed_url, file_data)
    }
}
