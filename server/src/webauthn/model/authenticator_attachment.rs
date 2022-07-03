use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticatorAttachment {
    Platform,
    #[serde(rename = "cross-platform")]
    CrossPlatform,
}
