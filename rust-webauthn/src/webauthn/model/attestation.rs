use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AttestationFormatIdentifier {
    #[serde(rename = "packed")]
    Packed,
    #[serde(rename = "tpm")]
    Tpm,
    #[serde(rename = "android-key")]
    AndroidKey,
    #[serde(rename = "android-safetynet")]
    AndroidSafetyNet,
    #[serde(rename = "fido-u2f")]
    FidoU2F,
    #[serde(rename = "apple")]
    AppleAnonymous,
    #[serde(rename = "none")]
    None,
}

pub struct AttestationStatement {
    pub alg: String,
    pub sig: String,
    pub x5c: String,
}

pub struct Attestation {
    fmt: AttestationFormatIdentifier,
    att_stmt: AttestationStatement,
}
