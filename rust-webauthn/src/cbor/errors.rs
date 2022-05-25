//! Errors returned by the module, including
//! [rust-openssl](https://docs.rs/openssl/0.10.35/openssl/index.html) and
//! [cbor-codec](https://twittner.gitlab.io/cbor-codec/cbor/) errors.
use cbor::decoder::DecodeError;
use cbor::encoder::EncodeError;
use std::io;

/// Errors that don't return anything.
pub type CoseResult = Result<(), CoseError>;
/// Results that return something.
pub type CoseResultWithRet<A> = Result<A, CoseError>;

/// Errors returned.
#[derive(Debug)]
pub enum CoseError {
    InvalidAlgorithmForContext(String),
    InvalidAlgorithm(),
    KeyDoesntSupportEncryption(),
    KeyDoesntSupportDecryption(),
    KeyUnableToEncryptOrDecrypt(),
    KeyUnableToSignOrVerify(),
    KeyDoesntSupportSigning(),
    KeyDoesntSupportVerification(),
    PrivateKeyNotPresent(),
    PublicKeyNotPresent(),
    DuplicateLabel(i32),
    InvalidLabel(i32),
    InvalidCounterSignature(),
    MissingRecipient(),
    MissingKey(),
    FunctionOnlyAvailableForContext(String),
    InvalidOperationForContext(String),
    InvalidContext(),
    MissingSignature(),
    MissingCiphertext(),
    MissingTag(),
    MissingPayload(),
    InvalidCoseStructure(),
    MissingParameter(String),
    InvalidParameter(String),
    NotImplemented(String),
    InvalidTag(),
    AlgorithmOnlySupportsOneRecipient(String),
    MissingAlgorithm(),
    IoError(io::Error),
    EncodeError(EncodeError),
    DecodeError(DecodeError),
}

impl std::fmt::Display for CoseError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for CoseError {
    fn from(err: io::Error) -> CoseError {
        CoseError::IoError(err)
    }
}

impl From<EncodeError> for CoseError {
    fn from(err: EncodeError) -> CoseError {
        CoseError::EncodeError(err)
    }
}

impl From<DecodeError> for CoseError {
    fn from(err: DecodeError) -> CoseError {
        CoseError::DecodeError(err)
    }
}
