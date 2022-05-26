//use crate::webauthn::model::UserEntity;

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Result;
use serde_json;

use crate::{
    errors::Error,
    utils::from_b64,
    webauthn::model::{
        Attestation, AuthenticatorAttestationResponse, AuthenticatorData, ClientData,
        PublicKeyCredential, PublicKeyCredentialType,
    },
    DataServices,
};

pub async fn register_challenge_response(
    session: Session,
    _service: web::Data<DataServices>,
    credential: web::Json<PublicKeyCredential>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Get the session info
    let challenge = session
        .get::<String>("challenge")
        .map_err(|_| Error::SessionError("Failed to get challenge from session".to_string()))?;
    if challenge.is_none() {
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "No challenge in session" }"#)
        );
    }
    let challenge = from_b64(&challenge.unwrap());
    if challenge.is_err() {
        return Ok(HttpResponse::InternalServerError()
            .json(r#"{ "message": "Error decoding challenge" }"#));
    }
    let challenge = challenge.unwrap();

    let name = session
        .get::<String>("name")
        .map_err(|_| Error::SessionError("Failed to get user name from session".to_string()))?;
    if name.is_none() {
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "No name in session" }"#)
        );
    }
    let name = name.unwrap();

    if credential.key_type != PublicKeyCredentialType::PublicKey {
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "response type must be 'public-key" }"#));
    }

    // Decode the clientData from base64
    if credential.response.attestation_object.is_some() {
        return process_attestation_response(&challenge, &name, &credential.response);
    }

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}

use openssl::sha::sha256;

/// Process an attestation response
pub fn process_attestation_response(
    challenge: &[u8],
    _name: &str,
    response: &AuthenticatorAttestationResponse,
) -> Result<HttpResponse, Error> {
    let auth_data_vec = response.authenticator_data.as_ref().ok_or_else(|| {
        Error::AttestationObjectError("No authData present in Attestation Obiect".to_string())
    })?;
    let _auth_data = AuthenticatorData::deserialize(auth_data_vec)?;

    let attestation_vec = response.attestation_object.as_ref().ok_or_else(|| {
        Error::AttestationObjectError("No attStmt present in Attestation Obiect".to_string())
    })?;
    let _attestation = Attestation::deserialize(attestation_vec)?;

    let client_data: ClientData = serde_json::from_slice(&response.client_data_json.to_owned())
        .map_err(Error::ClientDataParseError)?;

    // Compare the challenges
    if client_data.challenge != challenge {
        return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad challenge" }"#));
    }

    // Verify the origin
    if client_data.origin != "http://localhost:3000" {
        return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad origin" }"#));
    }

    //------------- Verify the signature --------------
    // Construct the signature base by concatenating the auth_data and
    let client_data_json = crate::utils::to_b64(&response.client_data_json);

    // Perform a sha256 hash of the client data
    let hash = sha256(client_data_json.as_bytes());
    let mut _signature_base = auth_data_vec.to_owned().extend(&hash.to_vec());

    // Make the cert from the

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}

pub fn process_authentication_response() {}
