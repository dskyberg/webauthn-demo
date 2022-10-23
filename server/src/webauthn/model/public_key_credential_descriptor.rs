use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use super::*;
use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialDiscriptor {
    #[serde(rename = "type")]
    pub type_: PublicKeyCredentialType,
    pub id: Base64UrlSafeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<AuthenticatorTransport>>,
}

impl TryFrom<&Credential> for PublicKeyCredentialDiscriptor {
    type Error = Error;

    fn try_from(credential: &Credential) -> Result<Self, Error> {
        Ok(Self {
            type_: credential.type_.clone(),
            id: credential.id.clone(),
            transports: None,
        })
    }
}
