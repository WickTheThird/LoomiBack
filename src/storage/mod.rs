pub mod postgres;
pub mod memory;

pub struct MemoryStorage;

#[async_trait::async_trait]
pub trait StorageLayer: Send + Sync {
    async fn health_check(&self) -> bool;
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl StorageLayer for MemoryStorage {
    async fn health_check(&self) -> bool {
        true
    }
}
