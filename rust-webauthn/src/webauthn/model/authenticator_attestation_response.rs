//! The AuthenticatorAttestationResponse will either contain an attestationObject
//! (for creation responses) or authenticatorData (for authentication responses).
//! One of the two must be present.
use base64urlsafedata::Base64UrlSafeData;
use openssl::sha::sha256;
use serde::Deserialize;

use crate::{cbor::algs::verify, errors::Error};

use super::*;

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
    pub attestation_object: Base64UrlSafeData,
    pub authenticator_data: Option<Base64UrlSafeData>,
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64UrlSafeData,
    // Bogus baggage
    pub get_authenticator_data: GetAuthenticatorData,
    pub get_public_key: GetPublicKey,
    pub get_public_key_algorithm: GetPublicKeyAlgorithm,
    pub get_transports: GetTransports,
}

impl AuthenticatorAttestationResponse {
    pub fn get_client_data(&self) -> Result<ClientData, Error> {
        serde_json::from_slice::<ClientData>(self.client_data_json.as_ref())
            .map_err(Error::ClientDataParseError)
    }

    /// Throws an error if no attStmt was provided
    pub fn attestation(&self) -> Result<Attestation, Error> {
        Attestation::try_from(self.attestation_object.as_ref())
    }

    /// The challenge should be provided from the session.
    /// The origin is the RP url, such as "http://localhost:3000"
    pub fn verify(
        &self,
        origin: &str,
        challenge: &Base64UrlSafeData,
    ) -> Result<AuthenticatorData, Error> {
        let attestation = self.attestation()?;
        match attestation.fmt {
            AttestationFormatIdentifier::Packed => self.verify_packed(origin, challenge),
            _ => Err(Error::AttestationFormatTypeError),
        }
    }

    /// Verify the response provided in packed format.
    fn verify_packed(
        &self,
        origin: &str,
        challenge: &Base64UrlSafeData,
    ) -> Result<AuthenticatorData, Error> {
        let client_data = self.get_client_data()?;

        // Compare the challenges
        if client_data.challenge != *challenge {
            dbg!(&challenge);
            dbg!(&client_data.challenge);
            return Err(Error::BadChallenge);
        }

        // Verify the origin
        if client_data.origin != origin {
            return Err(Error::BadOrigin);
        }

        // 7.1 step 7: Verify the type
        if client_data.client_data_type != ClientDataType::Create {
            // Wrong type
            return Err(Error::AssertionVerificationError(
                "Not a credential assertion".to_string(),
            ));
        }
        let attestation = &self.attestation()?;

        // Verify the rp_id hash
        // If no RP ID is sent by the RP, then the origin domain is used.
        // ( just the domain.  No scheme or port)
        let rp_id_hash = sha256("localhost".as_bytes());
        if rp_id_hash != attestation.auth_data.rp_id_hash {
            return Err(Error::AssertionVerificationError(
                "RP ID Hash does not match".to_string(),
            ));
        }

        //------------- Verify the signature --------------

        // 7.1 Step 11: Perform a sha256 hash of the client data
        // Construct the signature base by concatenating the auth_data and
        // the SHA256 hash of the JSON formatted client data.
        let verification_data: Vec<u8> = attestation
            .auth_data_bytes
            .iter()
            .chain(sha256(self.client_data_json.as_ref()).iter())
            .copied()
            .collect();

        let pub_key = attestation
            .auth_data
            .get_public_key(attestation.att_stmt.alg)?;
        let result = verify(
            attestation.att_stmt.alg as i32,
            &pub_key,
            &verification_data,
            &attestation.att_stmt.sig(),
        )
        .map_err(|_| Error::AttestationObjectError("Failed".to_string()))?;

        if !result {
            return Err(Error::AssertionVerificationError(
                "Assertion signature did not verify".to_string(),
            ));
        }
        Ok(attestation.auth_data.clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::{errors::Error, webauthn::model::AuthenticatorAttestationResponse};

    use base64urlsafedata::Base64UrlSafeData;
    use serde_json;

    #[test]
    fn test_it() -> Result<(), Error> {
        // The challenge would be provided from a persistent source, such as the session
        let challenge = Base64UrlSafeData(vec![
            239, 136, 194, 248, 21, 126, 34, 40, 3, 39, 28, 78, 243, 196, 218, 40, 34, 68, 122,
            134, 178, 243, 62, 135, 74, 78, 9, 215, 222, 53, 44, 18, 0,
        ]);
        let origin = "http://localhost:3000";

        let json = r#"{
            "attestationObject": "o2NmbXRmcGFja2VkZ2F0dFN0bXSiY2FsZyZjc2lnWEYwRAIgIEalxmKbAUAS7MpqUGaUDkSMCPLGJwEUVnq7zNYxY_YCIGkDeWLzdTYt2G287fYvtdN5b7aW1MomLHdON5y8XbLaaGF1dGhEYXRhWM1Jlg3liA6MaHQ0Fw9kdmBbj-SuuaKGMseZXPO6gx2XY0UAAAAArc4AAjW8xgpkiwsl8fBVAwBJSp48IN0iSogTuP_8VyjBVDbQwUE6SJpQP-pcxvlrwS5WM2S4xc1sMQ-hQ1o9pTEx5jW_in-vQ-yN8EzGv94-A4CfdQM1r-D9NqUBAgMmIAEhWCCGpyMiBThl0gi2R1amkHlL6x5A2ejQxelDPd32w7VUECJYICPVkYjScnZYEOmO9W8fHQlrzcxSgV_A4eUoHcjQ8kIt",
            "getAuthenticatorData": {},
            "getPublicKey": {},
            "getPublicKeyAlgorithm": {},
            "getTransports": {},
            "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiNzRqQy1CVi1JaWdESnh4Tzg4VGFLQ0pFZW9heTh6NkhTazRKMTk0MUxCSUEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJjcm9zc09yaWdpbiI6ZmFsc2V9"
        }"#;

        let response: AuthenticatorAttestationResponse =
            serde_json::from_str(json).expect("not yet");
        //dbg!(&response);
        let result = response.verify(origin, &challenge);
        assert!(result.is_ok());
        Ok(())
    }
}
