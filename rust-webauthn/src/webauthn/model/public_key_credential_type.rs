use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PublicKeyCredentialType {
    #[serde(rename = "public-key")]
    PublicKey,
}

impl Default for PublicKeyCredentialType {
    fn default() -> Self {
        Self::PublicKey
    }
}
