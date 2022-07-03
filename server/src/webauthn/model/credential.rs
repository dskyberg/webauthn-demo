//! Model for stored credential
use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use super::*;
use crate::cbor::keys::CoseKey;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Credential {
    pub id: Base64UrlSafeData,
    #[serde(rename = "type")]
    pub type_: PublicKeyCredentialType,
    pub count: u32,
    pub aaguid: [u8; 16],
    pub credential_public_key: CoseKey,
}
