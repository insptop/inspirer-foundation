use serde::de::DeserializeOwned;

pub mod db;

/// Component provider
#[async_trait::async_trait]
pub trait ComponentProvider: Sized {
    type Error: Into<crate::Error>;

    /// Config for the component.
    type Config: DeserializeOwned;

    /// We need the key to find configuration for the component from the full configuration.
    fn config_key() -> &'static str;

    async fn create(config: Self::Config) -> Result<Self, Self::Error>;
}