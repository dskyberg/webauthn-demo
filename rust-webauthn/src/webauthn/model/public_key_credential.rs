//! Response returned from the Authenticator for creation and assertion functions
//!
use super::{AuthenticatorAttestationResponse, PublicKeyCredentialType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetClientExtensionResults {}

/*
{
    "rawId": "ixHgvd_w_0XNApNk2IWpl-XQjj7Y5854lapQJ2EYbc6hVUZJChSofYSDC-Lp9ztF8fPTAZAKk4yfXeSFXj7Vrs2KAQ1yvjpXLLnm",
    "response": {
        "attestationObject": "o2NmbXRmcGFja2VkZ2F0dFN0bXSiY2FsZyZjc2lnWEcwRQIgAcYBbyUKMhl5TWTP7OEJ43yx0mq58Ic3j0uLr-k-IsUCIQDvIi_Zj498HjemL9fAms9GzuNnnaiJzv99LmDYNWbRD2hhdXRoRGF0YVjPSZYN5YgOjGh0NBcPZHZgW4_krrmihjLHmVzzuoMdl2NFAAAAAK3OAAI1vMYKZIsLJfHwVQMAS4sR4L3f8P9FzQKTZNiFqZfl0I4-2OfOeJWqUCdhGG3OoVVGSQoUqH2Egwvi6fc7RfHz0wGQCpOMn13khV4-1a7NigENcr46Vyy55qUBAgMmIAEhWCDxxLwUIL40MDlYfKw41NFdNFKEMd3jqw3lt7KoP4wKsCJYIOjfaaIksuYT_mGpQBJ14cyKPWeqC85LABs1FdkUGLgR",
        "getAuthenticatorData": {},
        "getPublicKey": {},
        "getPublicKeyAlgorithm": {},
        "getTransports": {},
        "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiTG9wejdDWk8za1ZVZ25WZGkzdGREcHJxOEtSM3psb3Jva0E3RkpxZlRaRUEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJjcm9zc09yaWdpbiI6ZmFsc2V9"
    },
    "authenticatorAttachment": "platform",
    "getClientExtensionResults": {},
    "id": "ixHgvd_w_0XNApNk2IWpl-XQjj7Y5854lapQJ2EYbc6hVUZJChSofYSDC-Lp9ztF8fPTAZAKk4yfXeSFXj7Vrs2KAQ1yvjpXLLnm",
    "type": "public-key"
}
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredential {
    #[serde(default, with = "serde_stuff::base64")]
    pub id: Vec<u8>,
    #[serde(default, with = "serde_stuff::base64")]
    pub raw_id: Vec<u8>,
    pub response: AuthenticatorAttestationResponse,
    pub get_client_extension_results: GetClientExtensionResults,
    #[serde(rename = "type")]
    pub key_type: PublicKeyCredentialType, // this wil always be "public-key"
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_it() {
        let pk = include_str!("../../../test_data/platform-attestation-response.json");

        let pub_key_cred: PublicKeyCredential = serde_json::from_str(pk).expect("Not yet");
        dbg!(&pub_key_cred);
    }
}
