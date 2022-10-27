use crate::cose::errors::CoseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("Not found")]
    NotFound,
    #[error("Base64UrlSafeData try_from error")]
    Base64UrlSafeDataError,
    #[error("Cache error: {0}")]
    CacheError(#[from] redis::RedisError),
    #[error("{0}")]
    SerdeJsonError(serde_json::Error),
    #[error("invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("missing attribute: {0}")]
    MissingAttribute(String),
    #[error("Error building element: {0}")]
    BuildError(String),
    #[error("UserEntityBuild error")]
    UserEntityBuildError,
    #[error("Error deserializing AuthenticatorData: {0}")]
    AuthenticatorDataDeserialize(String),
    #[error("Error deserializing public key")]
    AuthenticatorDataPublicKeyError,
    #[error("AuthenticatorSelectionCriteriaBuilder error")]
    AuthenticatorSelectionCriteriaBuildError,
    #[error("RegistrationChallengResponseBuilder error")]
    RegistrationChallengResponseBuildError,
    #[error("{0}")]
    CoseKeyError(CoseError),
    #[error("Error parsing Attestation: {0}")]
    AttestationParseError(serde_cbor::error::Error),
    #[error("Attestation Object error: {0}")]
    AttestationObjectError(String),
    #[error("Client data failed to deserialize: {0}")]
    ClientDataParseError(serde_json::Error),
    #[error("No id or name.  Must have at least one of them")]
    MissingIdAndName,
    #[error("RpEntityBuild error")]
    RpEntityBuildError,
    #[error("Base64 error: {0}")]
    Base64Error(base64::DecodeError),
    #[error("General error")]
    GeneralError,
    #[error("{0}")]
    SessionError(String),
    #[error("Internal Service Error: {0}")]
    InternalServiceError(String),
    #[error("Assertion verification error: {0}")]
    AssertionVerificationError(String),
    #[error("Unsupported attestation format")]
    AttestationFormatTypeError,
    #[error("Challenge does not match")]
    BadChallenge,
    #[error("Origin does not match")]
    BadOrigin,
    #[error("Bad credential counter")]
    BadSignCounter,
    #[error("Attribute not found: {0}")]
    AttributeNotFound(String),
    #[error("Bad url: {0}")]
    UrlError(url::ParseError),
    #[error("Error inserting {0} into session")]
    SessionInsertError(String),
    #[error("Bad header value for Session header")]
    BadSessionHeader,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Missing alg in AttestationStatement")]
    AttestationStatementMissingAlg,
    #[error("Missing sig in AttestationStatement")]
    AttestationStatementMissingSig,
    #[error("Bad sig format in AttestationStatement")]
    AttestationStatementBadSigFormat,
}

use actix_web::{http::StatusCode, HttpResponse};

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json("WebAuthn Error")
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
