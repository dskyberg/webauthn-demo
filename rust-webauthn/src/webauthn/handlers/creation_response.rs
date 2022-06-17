//use crate::webauthn::model::UserEntity;

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use base64urlsafedata::Base64UrlSafeData;

use crate::{
    errors::Error,
    webauthn::model::{CreationPublicKeyCredential, PublicKeyCredentialType},
    DataServices,
};

pub async fn creation_response(
    session: Session,
    service: web::Data<DataServices>,
    credential: web::Json<CreationPublicKeyCredential>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    //log::info!("PublicKeyCredential: {:?}", &credential);
    let config = service.get_config().await?;

    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    let challenge = session
        .get::<String>("challenge")
        .map_err(|_| Error::SessionError("Failed to get challenge from session".to_string()))?;
    if challenge.is_none() {
        log::error!("Failed to get challenge from session");
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "No challenge in session" }"#)
        );
    }
    // The challenge should have been stored as Base64.  Decode it
    let challenge = Base64UrlSafeData::try_from(challenge.unwrap().as_str());
    if challenge.is_err() {
        log::info!("Error decoding challenge");
        return Ok(HttpResponse::InternalServerError()
            .json(r#"{ "message": "Error decoding challenge" }"#));
    }
    let challenge = challenge.unwrap();

    log::info!("Session challenge");

    let name = session
        .get::<String>("name")
        .map_err(|_| Error::SessionError("Failed to get user name from session".to_string()))?;
    if name.is_none() {
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "No name in session" }"#)
        );
    }
    let name = name.unwrap();

    log::info!("Session name");

    // ------------ 7.1 RP verification ----------------//
    // Steps 1 - 6 are either performed in javascript before
    // postint.  Start with step 7

    // 7.1 Step 7
    if credential.type_ != PublicKeyCredentialType::PublicKey {
        log::info!("PublicKeyCredentialTyep is bad");
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "response type must be 'public-key" }"#));
    }

    log::info!("Calling verify");
    let result = credential.response.verify(&config.webauthn, &challenge);
    if let Err(err) = result {
        match err {
            Error::BadChallenge => {
                log::info!("Challenge mismatch");
                return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad challenge" }"#));
            }
            Error::BadOrigin => {
                log::info!("Origin mismatch");
                return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad origin" }"#));
            }
            _ => return Err(err),
        }
    }

    let auth_data = result.unwrap();

    // The response is valid.
    // Step 22: Verify that the credentialId is not being used
    // The authData is returnef from the verify function
    let id = Base64UrlSafeData(auth_data.credential_data()?.credential_id);
    let cache_response = service
        .get_credential(&id)
        .await
        .map_err(|_| Error::GeneralError)?;

    if let Some(_creds) = cache_response {
        log::info!("Credential ID is already used");
        return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "credentialId in use" }"#));
    }
    // Save the credential

    let _ = service
        .add_credential_for_user(&name, &id, &auth_data.as_credential())
        .await?;

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}