//! Model for stored credential
use serde::{Deserialize, Serialize};

use crate::cbor::keys::CoseKey;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Credential {
    pub count: u32,
    pub aaguid: [u8; 16],
    pub credential_public_key: CoseKey,
}
