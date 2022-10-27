//! The AuthenticatorAttestationResponse is sent as a result of credential creation
//! One of the two must be present.
use base64urlsafedata::Base64UrlSafeData;
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
pub struct AuthenticatorAttestationResponse {
    pub attestation_object: Base64UrlSafeData,
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64UrlSafeData,
    // Bogus baggage
    pub get_authenticator_data: Option<GetAuthenticatorData>,
    pub get_public_key: Option<GetPublicKey>,
    pub get_public_key_algorithm: Option<GetPublicKeyAlgorithm>,
    pub get_transports: Option<GetTransports>,
}

impl AuthenticatorAttestationResponse {
    pub fn get_client_data(&self) -> Result<ClientData, Error> {
        serde_json::from_slice::<ClientData>(self.client_data_json.as_ref())
            .map_err(Error::ClientDataParseError)
    }

    /// Throws an error if no attStmt was provided
    pub fn attestation(&self) -> Result<Attestation, Error> {
        Attestation::try_from(&self.attestation_object)
    }

    /// The challenge should be provided from the session.
    /// The origin is the RP url, such as "http://localhost:3000"
    pub fn verify(
        &self,
        policy: &WebauthnPolicy,
        challenge: &Base64UrlSafeData,
    ) -> Result<AuthenticatorData, Error> {
        log::info!("Verify: start");
        let attestation = self.attestation()?;
        match attestation.fmt {
            AttestationFormatIdentifier::Packed => self.verify_packed(policy, challenge),
            AttestationFormatIdentifier::None => self.verify_none(policy, challenge),
            _ => Err(Error::AttestationFormatTypeError),
        }
    }

    /// Verify the response provided in packed format.
    fn verify_packed(
        &self,
        policy: &WebauthnPolicy,
        challenge: &Base64UrlSafeData,
    ) -> Result<AuthenticatorData, Error> {
        log::info!("Packed Verify: start");

        let client_data = self.get_client_data()?;
        log::info!("Got client data: {:?}", &client_data);

        // Compare the challenges
        if client_data.challenge != *challenge {
            dbg!(&challenge);
            dbg!(&client_data.challenge);
            return Err(Error::BadChallenge);
        }

        log::info!("Verify: challenge matched");

        // Verify the origin
        if client_data.origin != policy.origin {
            return Err(Error::BadOrigin);
        }
        log::info!("Verify: origin matched");

        // 7.1 step 7: Verify the type
        if client_data.client_data_type != ClientDataType::Create {
            // Wrong type
            return Err(Error::AssertionVerificationError(
                "Not a credential attestation".to_string(),
            ));
        }
        let attestation = &self.attestation()?;

        // Get the pieces part of the attStmt
        let alg = attestation.att_stmt.alg()?;
        let sig = attestation.att_stmt.sig()?;

        log::info!("Verify: client data type is webauthn.get");

        // Verify the rp_id hash
        // If no RP ID is sent by the RP, then the origin domain is used.
        // ( just the domain.  No scheme or port)

        let rp_id_hash = sha256(policy.rp_id.as_bytes());
        if rp_id_hash != attestation.auth_data.rp_id_hash {
            return Err(Error::AssertionVerificationError(
                "RP ID Hash does not match".to_string(),
            ));
        }
        log::info!("Verify: rp_id_hash matched");

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

        let pub_key = attestation.auth_data.get_public_key(alg)?;
        let result = verify(alg as i32, &pub_key, &verification_data, &sig)
            .map_err(|_| Error::AttestationObjectError("Failed".to_string()))?;

        if !result {
            return Err(Error::AssertionVerificationError(
                "Assertion signature did not verify".to_string(),
            ));
        }
        Ok(attestation.auth_data.clone())
    }

    /// Verify the response provided in "none" format. This is likely a Passkey, with
    /// an empty attestation statement.
    fn verify_none(
        &self,
        policy: &WebauthnPolicy,
        challenge: &Base64UrlSafeData,
    ) -> Result<AuthenticatorData, Error> {
        log::info!("None Verify: start");

        let client_data = self.get_client_data()?;
        log::info!("Got client data: {:?}", &client_data);

        // Compare the challenges
        if client_data.challenge != *challenge {
            dbg!(&challenge);
            dbg!(&client_data.challenge);
            return Err(Error::BadChallenge);
        }

        log::info!("Verify: challenge matched");

        // Verify the origin
        if client_data.origin != policy.origin {
            return Err(Error::BadOrigin);
        }
        log::info!("Verify: origin matched");

        // 7.1 step 7: Verify the type
        if client_data.client_data_type != ClientDataType::Create {
            // Wrong type
            return Err(Error::AssertionVerificationError(
                "Not a credential attestation".to_string(),
            ));
        }
        let attestation = &self.attestation()?;

        log::info!("Verify: client data type is webauthn.get");

        // Verify the rp_id hash
        // If no RP ID is sent by the RP, then the origin domain is used.
        // ( just the domain.  No scheme or port)

        let rp_id_hash = sha256(policy.rp_id.as_bytes());
        if rp_id_hash != attestation.auth_data.rp_id_hash {
            return Err(Error::AssertionVerificationError(
                "RP ID Hash does not match".to_string(),
            ));
        }
        log::info!("Verify: rp_id_hash matched");

        // There is no signature in a Passkey
        log::info!("Verify: passkey verification complete");
        Ok(attestation.auth_data.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::Error;
    use serde_json;

    #[test]
    fn test_it() -> Result<(), Error> {
        let json = include_str!("../../../test_data/platform-attestation-response.json");
        let policy = WebauthnPolicy::default();

        let response: AuthenticatorAttestationResponse =
            serde_json::from_str(json).expect("not yet");
        //dbg!(&response);

        // The challenge would be provided from a persistent source, such as the session
        // For testing, just grab the one in the response.
        let challenge = response.get_client_data().expect("oops").challenge;

        let result = response.verify(&policy, &challenge);
        assert!(result.is_ok());
        Ok(())
    }
}
