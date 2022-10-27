//! Model for stored credential
use base64urlsafedata::Base64UrlSafeData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::*;
use crate::cose::keys::CoseKey;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    pub id: Base64UrlSafeData,
    #[serde(rename = "type")]
    pub type_: PublicKeyCredentialType,
    pub counter: u32,
    pub aaguid: [u8; 16],
    pub credential_public_key: CoseKey,
    pub flags: u8,
    pub last: DateTime<Utc>,
}

impl Credential {
    /// Isolating time functions for simplicity
    pub fn now(&self) -> Self {
        let mut new = self.clone();
        new.last = Utc::now();
        new
    }
}
