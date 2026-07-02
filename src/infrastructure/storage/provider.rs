use async_trait::async_trait;

#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn upload(&self, path: &str, bytes: Vec<u8>) -> anyhow::Result<String>;

    async fn delete(&self, path: &str) -> anyhow::Result<()>;

    async fn exists(&self, path: &str) -> anyhow::Result<bool>;

    async fn url(&self, path: &str) -> anyhow::Result<String>;
}
