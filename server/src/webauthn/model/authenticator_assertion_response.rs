//! The AuthenticatorAssertionResponse is sent as a resul of client authentication.
//!
use base64urlsafedata::Base64UrlSafeData;
use chrono::Utc;
use openssl::sha::sha256;
use serde::Deserialize;

use crate::{cose::algs::verify, errors::Error};

use super::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GetAuthenticatorData {}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GetPublicKey {}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GetPublicKeyAlgorithm {}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GetTransports {}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
    /// Verify returns the updated credential, so that counters can be tracked appropriately.
    pub fn verify(
        &self,
        policy: &WebauthnPolicy,
        challenge: &Base64UrlSafeData,
        credential: &Credential,
    ) -> Result<Credential, Error> {
        self.verify_packed(policy, challenge, credential)
    }

    /// Verify the response provided in packed format.
    fn verify_packed(
        &self,
        policy: &WebauthnPolicy,
        challenge: &Base64UrlSafeData,
        credential: &Credential,
    ) -> Result<Credential, Error> {
        log::trace!("Verify start");
        let client_data = self.get_client_data()?;

        // 7.2 step 11: Verify the type
        if client_data.client_data_type != ClientDataType::Get {
            // Wrong type
            log::trace!("Verify: Not a credential assertion");
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
        if client_data.origin != policy.origin {
            return Err(Error::BadOrigin);
        }

        // 7.2 step 14; Verify token binding
        if let Some(token_binding) = client_data.token_binding {
            log::trace!("Token binding status: {:?}", token_binding.status);
        }

        // 7.2 step 15: Verify the rp_id hash
        // If no RP ID is sent by the RP, then the origin domain is used.
        // ( just the domain.  No scheme or port)
        let rp_id_hash = sha256(policy.rp_id.as_bytes());
        let auth_data =
            AuthenticatorData::try_from(self.authenticator_data.as_ref()).map_err(|e| {
                log::trace!("Failed to decode AuthenticatorData");
                e
            })?;
        if rp_id_hash != auth_data.rp_id_hash {
            return Err(Error::AssertionVerificationError(
                "RP ID Hash does not match".to_string(),
            ));
        }

        // 7.2 step 16; Verify userPresent flag
        if !auth_data.is_user_present() {
            log::trace!("Verify: User not present");
            return Err(Error::AssertionVerificationError(
                "userPresent flag not set".to_string(),
            ));
        }
        // 7.2 step 17; Verify userPresent flag
        if !auth_data.is_user_verified() {
            log::trace!("Verify: User not verified");
            return Err(Error::AssertionVerificationError(
                "userVerified flag not set".to_string(),
            ));
        }

        // 7.2 step 21; Verify signCount is greater
        // Note: Passkey changes this behavior by not providing a counter.  Thus,
        // this should be a matter of policy.
        let mut new_cred = credential.clone();
        new_cred.last = Utc::now();
        if policy.validate_sign_count && auth_data.counter <= credential.counter {
            log::trace!("ERROR!!!  Bad signCount {:}", &auth_data.counter);
            return Err(Error::BadSignCounter);
        } else {
            new_cred.counter = auth_data.counter;
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
        log::trace!("Signature validated");
        Ok(new_cred)
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
