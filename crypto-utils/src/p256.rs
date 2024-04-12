use p256::{ecdsa::SigningKey, elliptic_curve::ALGORITHM_OID, NistP256, SecretKey};
use pkcs8::{der::EncodePem, AssociatedOid, EncodePublicKey, LineEnding, PrivateKeyInfo};
use rand::rngs::OsRng;

use crate::{KeyPair, KeyPairTrait, Result};

pub struct P256 {
    secret_key: SecretKey,
    // public_key:
}

impl KeyPairTrait for KeyPair<P256> {
    fn generate() -> Result<Self> {
        Ok(KeyPair {
            key_pair: P256 {
                secret_key: SecretKey::from(SigningKey::random(&mut OsRng::default())),
            },
        })
    }

    fn get_private_key_pem(&self) -> Result<String> {
        let algorithm_identifier = pkcs8::AlgorithmIdentifierRef {
            oid: ALGORITHM_OID,
            parameters: Some((&NistP256::OID).into()),
        };

        let ec = self.key_pair.secret_key.to_sec1_der()?;
        let prikey = PrivateKeyInfo::new(algorithm_identifier, &ec);

        Ok(prikey.to_pem(LineEnding::default())?)
    }

    fn get_public_key_pem(&self) -> Result<String> {
        Ok(self
            .key_pair
            .secret_key
            .public_key()
            .to_public_key_pem(LineEnding::default())?)
    }

    fn get_jwks(&self) -> serde_json::Value {
        serde_json::to_value(self.key_pair.secret_key.public_key().to_jwk()).unwrap()
    }
}
