#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DerError(#[from] der::Error),

    #[error(transparent)]
    SpkiError(#[from] spki::Error)
}
