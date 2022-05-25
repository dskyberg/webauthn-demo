use serde::Deserialize;
use serde_cbor::Value;

use super::COSEAlgorithm;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationStatement {
    pub alg: COSEAlgorithm,
    pub sig: Value,
    pub x5c: Option<Value>,
    pub ecdaa_key_id: Option<Value>,
}
