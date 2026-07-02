use async_trait::async_trait;

use super::provider::StorageProvider;

pub struct S3Storage;

#[async_trait]
impl StorageProvider for S3Storage {
    async fn upload(&self, _path: &str, _bytes: Vec<u8>) -> anyhow::Result<String> {
        todo!()
    }

    async fn delete(&self, _path: &str) -> anyhow::Result<()> {
        todo!()
    }

    async fn exists(&self, _path: &str) -> anyhow::Result<bool> {
        todo!()
    }

    async fn url(&self, _path: &str) -> anyhow::Result<String> {
        todo!()
    }
}
