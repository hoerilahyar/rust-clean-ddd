use std::sync::Arc;

use crate::config::Config;

use super::{local::LocalStorage, minio::MinioStorage, provider::StorageProvider, s3::S3Storage};

pub struct Uploader {
    provider: Arc<dyn StorageProvider>,
}

impl Uploader {
    pub fn new(config: &Config) -> Self {
        let provider: Arc<dyn StorageProvider> = match config.storage.provider.as_str() {
            "local" => Arc::new(LocalStorage),
            "minio" => Arc::new(MinioStorage),
            "s3" => Arc::new(S3Storage),
            _ => Arc::new(LocalStorage),
        };

        Self { provider }
    }

    pub async fn upload(&self, path: &str, bytes: Vec<u8>) -> anyhow::Result<String> {
        self.provider.upload(path, bytes).await
    }

    pub async fn delete(&self, path: &str) -> anyhow::Result<()> {
        self.provider.delete(path).await
    }

    pub async fn exists(&self, path: &str) -> anyhow::Result<bool> {
        self.provider.exists(path).await
    }

    pub async fn url(&self, path: &str) -> anyhow::Result<String> {
        self.provider.url(path).await
    }
}
