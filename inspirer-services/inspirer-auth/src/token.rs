use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub sub: Uuid,
    pub aud: Uuid,
    pub scope: String,
    pub iat: usize,
    pub exp: usize,
}

impl AccessToken {
    pub fn token(&self) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap()
    }
}

pub trait GetToken: Serialize {
    fn get_token(&self) -> String {

        // EncodingKey::from_ec_pem(key)
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap()
    }
}