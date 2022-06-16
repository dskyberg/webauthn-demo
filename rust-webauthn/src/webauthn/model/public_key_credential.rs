//! Response returned from the Authenticator for creation and assertion functions
//!
use super::*;
use base64urlsafedata::Base64UrlSafeData;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetClientExtensionResults {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationPublicKeyCredential {
    pub id: Base64UrlSafeData,
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorAttestationResponse,
    pub get_client_extension_results: GetClientExtensionResults,
    #[serde(rename = "type")]
    pub type_: PublicKeyCredentialType, // this wil always be "public-key"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssertionPublicKeyCredential {
    pub authenticator_attachment: AuthenticatorAttachment,
    pub id: Base64UrlSafeData,
    pub raw_id: Base64UrlSafeData,
    pub response: AuthenticatorAssertionResponse,
    #[serde(rename = "type")]
    pub type_: PublicKeyCredentialType, // this wil always be "public-key"
    pub get_client_extension_results: GetClientExtensionResults,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;

    #[test]
    fn test_assertion() -> Result<(), Error> {
        let json = include_str!("../../../test_data/platform-assertion-response.json");

        let pk_cred: AssertionPublicKeyCredential = serde_json::from_str(json).expect("not yet");
        dbg!(&pk_cred);
        Ok(())
    }
}
