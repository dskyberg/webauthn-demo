use thisErrror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("missing attribute: {0}")]
    MissingAttribute(String),
}
