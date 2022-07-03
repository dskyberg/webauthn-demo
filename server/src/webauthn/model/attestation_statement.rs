//! The values present in the AttestationStatement are determined
//! by the attestation type.  In the case of Passkey, the attestation
//! type is "none", and the AttestationStatement is empty.
//!
use serde::Deserialize;
use serde_cbor::Value;

use super::COSEAlgorithm;
use crate::errors::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationStatement {
    pub alg: Option<COSEAlgorithm>,
    pub sig: Option<Value>,
    pub x5c: Option<Value>,
    pub ecdaa_key_id: Option<Value>,
}

impl AttestationStatement {
    pub fn sig(&self) -> Result<Vec<u8>, Error> {
        match self
            .sig
            .clone()
            .ok_or(Error::AttestationStatementMissingSig)?
        {
            Value::Bytes(bytes) => Ok(bytes),
            _ => Err(Error::AttestationStatementBadSigFormat),
        }
    }

    pub fn alg(&self) -> Result<COSEAlgorithm, Error> {
        self.alg.ok_or(Error::AttestationStatementMissingAlg)
    }
}
