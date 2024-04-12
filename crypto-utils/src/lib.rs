pub mod error;
pub mod p256;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct KeyPair<T> {
    key_pair: T,
}

pub trait KeyPairTrait: Sized {
    fn generate() -> Result<Self>;

    fn get_private_key_pem(&self) -> Result<String>;

    fn get_public_key_pem(&self) -> Result<String>;

    fn get_jwks(&self) -> serde_json::Value;
}
