//! The AuthenticatorAssertionResponse is sent as a resul of client authentication.
//!
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
pub struct AuthenticatorAssertionResponse {
    pub authenticator_data: Base64UrlSafeData,
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64UrlSafeData,
    pub signature: Base64UrlSafeData,
    pub user_handle: Option<String>,
}

impl AuthenticatorAssertionResponse {
    pub fn get_client_data(&self) -> Result<ClientData, Error> {
        serde_json::from_slice::<ClientData>(self.client_data_json.as_ref())
            .map_err(Error::ClientDataParseError)
    }

    /// The challenge should be provided from the session.
    /// The origin is the RP url, such as "http://localhost:3000"
    pub fn verify(
        &self,
        origin: &str,
        challenge: &Base64UrlSafeData,
        credential: &Credential,
    ) -> Result<(), Error> {
        self.verify_packed(origin, challenge, credential)
    }

    /// Verify the response provided in packed format.
    fn verify_packed(
        &self,
        origin: &str,
        challenge: &Base64UrlSafeData,
        credential: &Credential,
    ) -> Result<(), Error> {
        log::info!("Verify start");
        let client_data = self.get_client_data()?;

        // 7.2 step 11: Verify the type
        if client_data.client_data_type != ClientDataType::Get {
            // Wrong type
            log::info!("Verify: Not a credential assertion");
            return Err(Error::AssertionVerificationError(
                "Not a credential attestation".to_string(),
            ));
        }

        // 7.2 step 12; Compare the challenges
        if client_data.challenge != *challenge {
            dbg!(&challenge);
            dbg!(&client_data.challenge);
            return Err(Error::BadChallenge);
        }

        // 7.2 step 13; Verify the origin
        if client_data.origin != origin {
            return Err(Error::BadOrigin);
        }

        // 7.2 step 14; Verify token binding
        if let Some(token_binding) = client_data.token_binding {
            log::info!("Token binding status: {:?}", token_binding.status);
        }

        // 7.2 step 15: Verify the rp_id hash
        // If no RP ID is sent by the RP, then the origin domain is used.
        // ( just the domain.  No scheme or port)
        // TODO: Stop hard coding localhost
        let rp_id_hash = sha256("localhost".as_bytes());
        let auth_data =
            AuthenticatorData::try_from(self.authenticator_data.as_ref()).map_err(|e| {
                log::info!("Failed to decode AuthenticatorData");
                e
            })?;
        if rp_id_hash != auth_data.rp_id_hash {
            return Err(Error::AssertionVerificationError(
                "RP ID Hash does not match".to_string(),
            ));
        }

        // 7.2 step 16; Verify userPresent flag
        if !auth_data.is_user_present() {
            log::info!("Verify: User not present");
            return Err(Error::AssertionVerificationError(
                "userPresent flag not set".to_string(),
            ));
        }
        // 7.2 step 17; Verify userPresent flag
        if !auth_data.is_user_verified() {
            log::info!("Verify: User not verified");
            return Err(Error::AssertionVerificationError(
                "userVerified flag not set".to_string(),
            ));
        }

        // 7.2 step 21; Verify signCount is greater
        if auth_data.counter <= credential.count {
            log::info!("ERROR!!!  Bad signCount {:}", &auth_data.counter);
        }
        let alg = credential.credential_public_key.alg.unwrap();
        let pub_key = credential
            .credential_public_key
            .get_pub_key(alg)
            .map_err(Error::CoseKeyError)?;

        // 7.2 step 19; Compute the hash of clientDataJSON
        let hash = sha256(self.client_data_json.as_ref());

        // 7.2 step 20; Verify the signature of authData+hash
        let verification_data: Vec<u8> = self
            .authenticator_data
            .as_ref()
            .iter()
            .chain(hash.iter())
            .copied()
            .collect();

        //------------- Verify the signature --------------
        let result = verify(alg, &pub_key, &verification_data, self.signature.as_ref())
            .map_err(|_| Error::AttestationObjectError("Failed".to_string()))?;

        if !result {
            return Err(Error::AssertionVerificationError(
                "Assertion signature did not verify".to_string(),
            ));
        }
        log::info!("Signature validated");
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::Error;
    use serde_json;

    #[test]
    fn test_attestation() -> Result<(), Error> {
        let json = include_str!("../../../test_data/platform-assertion-response.json");

        let pk_cred: AssertionPublicKeyCredential = serde_json::from_str(json).expect("not yet");
        dbg!(&pk_cred);
        Ok(())
    }
}
