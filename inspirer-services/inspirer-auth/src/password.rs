use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};

pub fn password_hash<P: AsRef<[u8]>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::DEFAULT);

    Ok(argon2.hash_password(password.as_ref(), &salt)?.to_string())
}

pub fn password_verify<P: AsRef<[u8]>, H: AsRef<str>>(password: P, hashed: H) -> eyre::Result<()> {
    let parsed = PasswordHash::new(hashed.as_ref())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::DEFAULT);

    Ok(argon2.verify_password(password.as_ref(), &parsed)?)
}

