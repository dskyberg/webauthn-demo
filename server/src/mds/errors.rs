use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JWT deserialize error")]
    JSONDeserialize(#[from] serde_json::Error),
    #[error("Error fetching url")]
    FetchFailed(#[from] reqwest::Error),
    #[error("Invalid Signature")]
    InvalidSignature,
}
