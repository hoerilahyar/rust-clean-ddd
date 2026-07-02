use async_trait::async_trait;

use super::provider::StorageProvider;

pub struct LocalStorage;

#[async_trait]
impl StorageProvider for LocalStorage {
    async fn upload(&self, path: &str, bytes: Vec<u8>) -> anyhow::Result<String> {
        tokio::fs::write(path, bytes).await?;

        Ok(path.to_string())
    }

    async fn delete(&self, path: &str) -> anyhow::Result<()> {
        if tokio::fs::try_exists(path).await? {
            tokio::fs::remove_file(path).await?;
        }

        Ok(())
    }

    async fn exists(&self, path: &str) -> anyhow::Result<bool> {
        Ok(tokio::fs::try_exists(path).await?)
    }

    async fn url(&self, path: &str) -> anyhow::Result<String> {
        Ok(path.to_string())
    }
}
