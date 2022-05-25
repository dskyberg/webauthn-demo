use crate::cbor::errors::CoseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Cache error: {0}")]
    CacheError(#[from] redis::RedisError),
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
    #[error("AuthenticatorSelectionCriteriaBuilder error")]
    AuthenticatorSelectionCriteriaBuildError,
    #[error("RegistrationChallengResponseBuilder error")]
    RegistrationChallengResponseBuildError,
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
    #[error("{0}")]
    CoseKeyError(CoseError),
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
