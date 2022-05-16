use serde::{Deserialize, Serialize};

use super::UserIdentity;
use crate::utils::make_id;
/*
use serde::Serializer;
impl Serialize for Vec<u8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = base64::encode_config(self, base64::URL_SAFE)
        serializer.serialize_string(&encoded)
    }
}
*/

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RegistrationChallengeRequest {
    pub name: String,
    pub displayName: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticatorAttachment {
    Platform,
    CrossPlatform,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AuthenticatorSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticatorAttachment: Option<AuthenticatorAttachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelyingParty {
    pub name: String,
}

/*
{
    "user": {
        "id": "kaOsrqtUgIgQEdf_4OI2wZnUfNWPDphLsP2CbDNIDX0",
        "displayName": "Bob Smith",
        "name": "Bob Smith"
    },
    "attestation": "none",
    "authenticatorSelection": {
        "authenticatorAttachment": "cross-platform"
    },
    "rp": {
        "name": "Stranger Labs, Inc."
    },
    "pubKeyCredParams": [
        {
            "type": "public-key",
            "alg": -7
        }
    ],
    "status": "ok",
    "challenge": "xqfhaZynnZZZICqOQdxzCjt-3cOJ6eNPZArO-j7iCo4"
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationChallengeResponse {
    #[serde(with = "serde_stuff::base64")]
    challenge: Vec<u8>,
    user: UserIdentity,
}

impl RegistrationChallengeResponse {
    pub fn new(name: &str, display_name: &str) -> Self {
        Self {
            challenge: make_id(32).unwrap(),
            user: UserIdentity::new(name, display_name),
        }
    }
}

struct RegistrationChallengBuilder {
    challenge: Vec<u8>,
    user: Option<UserIdentity>,
    //attestation: Option<Attestation
}
