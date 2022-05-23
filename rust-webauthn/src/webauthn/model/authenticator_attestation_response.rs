//! The AuthenticatorAttestationResponse will either contain an attestationObject
//! (for creation responses) or authenticatorData (for authentication responses).
//! One of the two must be present.
use serde::Deserialize;

/*
{
    "attestationObject": "o2NmbXRmcGFja2VkZ2F0dFN0bXSiY2FsZyZjc2lnWEcwRQIgAcYBbyUKMhl5TWTP7OEJ43yx0mq58Ic3j0uLr-k-IsUCIQDvIi_Zj498HjemL9fAms9GzuNnnaiJzv99LmDYNWbRD2hhdXRoRGF0YVjPSZYN5YgOjGh0NBcPZHZgW4_krrmihjLHmVzzuoMdl2NFAAAAAK3OAAI1vMYKZIsLJfHwVQMAS4sR4L3f8P9FzQKTZNiFqZfl0I4-2OfOeJWqUCdhGG3OoVVGSQoUqH2Egwvi6fc7RfHz0wGQCpOMn13khV4-1a7NigENcr46Vyy55qUBAgMmIAEhWCDxxLwUIL40MDlYfKw41NFdNFKEMd3jqw3lt7KoP4wKsCJYIOjfaaIksuYT_mGpQBJ14cyKPWeqC85LABs1FdkUGLgR",
    "getAuthenticatorData": {},
    "getPublicKey": {},
    "getPublicKeyAlgorithm": {},
    "getTransports": {},
    "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiTG9wejdDWk8za1ZVZ25WZGkzdGREcHJxOEtSM3psb3Jva0E3RkpxZlRaRUEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJjcm9zc09yaWdpbiI6ZmFsc2V9"
}
*/

#[derive(Debug, Deserialize)]
pub struct GetAuthenticatorData {}
#[derive(Debug, Deserialize)]
pub struct GetPublicKey {}
#[derive(Debug, Deserialize)]
pub struct GetPublicKeyAlgorithm {}
#[derive(Debug, Deserialize)]
pub struct GetTransports {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorAttestationResponse {
    #[serde(default, with = "serde_stuff::option_base64")]
    pub attestation_object: Option<Vec<u8>>,
    #[serde(default, with = "serde_stuff::option_base64")]
    pub authenticator_data: Option<Vec<u8>>,
    pub get_authenticator_data: GetAuthenticatorData,
    pub get_public_key: GetPublicKey,
    pub get_public_key_algorithm: GetPublicKeyAlgorithm,
    pub get_transports: GetTransports,
    #[serde(default, rename = "clientDataJSON", with = "serde_stuff::base64")]
    pub client_data_json: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_it() {
        let json = r#"{
            "attestationObject": "o2NmbXRmcGFja2VkZ2F0dFN0bXSiY2FsZyZjc2lnWEcwRQIgAcYBbyUKMhl5TWTP7OEJ43yx0mq58Ic3j0uLr-k-IsUCIQDvIi_Zj498HjemL9fAms9GzuNnnaiJzv99LmDYNWbRD2hhdXRoRGF0YVjPSZYN5YgOjGh0NBcPZHZgW4_krrmihjLHmVzzuoMdl2NFAAAAAK3OAAI1vMYKZIsLJfHwVQMAS4sR4L3f8P9FzQKTZNiFqZfl0I4-2OfOeJWqUCdhGG3OoVVGSQoUqH2Egwvi6fc7RfHz0wGQCpOMn13khV4-1a7NigENcr46Vyy55qUBAgMmIAEhWCDxxLwUIL40MDlYfKw41NFdNFKEMd3jqw3lt7KoP4wKsCJYIOjfaaIksuYT_mGpQBJ14cyKPWeqC85LABs1FdkUGLgR",
            "getAuthenticatorData": {},
            "getPublicKey": {},
            "getPublicKeyAlgorithm": {},
            "getTransports": {},
            "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiTG9wejdDWk8za1ZVZ25WZGkzdGREcHJxOEtSM3psb3Jva0E3RkpxZlRaRUEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJjcm9zc09yaWdpbiI6ZmFsc2V9"
        }        
        "#;

        let response: AuthenticatorAttestationResponse =
            serde_json::from_str(json).expect("not yet");
        dbg!(&response);
    }
}
