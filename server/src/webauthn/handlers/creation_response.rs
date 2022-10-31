use actix_web::{web, HttpRequest, HttpResponse};
use base64urlsafedata::Base64UrlSafeData;

use crate::{
    errors::Error,
    webauthn::model::{CreationPublicKeyCredential, PublicKeyCredentialType},
    DataServices, Session,
};

pub async fn creation_response(
    service: web::Data<DataServices>,
    credential: web::Json<CreationPublicKeyCredential>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let config = service.get_config().await?;

    // Get the session from the request header
    let session = Session::from_request(&service, &req).await.map_err(|e| {
        log::trace!("Failed to get session from data service");
        e
    })?;

    if session.is_empty() {
        log::trace!("Session is invalid.  No entries");
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "Error getting session" }"#)
        );
    }

    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    // The challenge should have been stored as Base64.  Decode it
    let challenge = session.as_b64("challenge")?;
    if let Err(err) = service.use_challenge(&challenge).await {
        match err {
            Error::ChallengeNotFound => {
                log::trace!("Provided challenge was not found");
                return Ok(HttpResponse::NotFound().json(r#"{ "message": "Challenge not found" }"#));
            }
            Error::ChallengeUsed => {
                log::trace!("Provided challenge was not valid");
                return Ok(
                    HttpResponse::Forbidden().json(r#"{ "message": "Challenge is already used" }"#)
                );
            }
            _ => {
                return Ok(HttpResponse::InternalServerError()
                    .json(r#"{ "message": "Error getting session" }"#))
            }
        }
    }

    let name = session.as_str("name")?;

    // ------------ 7.1 RP verification ----------------//
    // Steps 1 - 6 are either performed in javascript before
    // postint.  Start with step 7

    // 7.1 Step 7
    if credential.type_ != PublicKeyCredentialType::PublicKey {
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "PublicKeyCredentialTyep type must be 'public-key" }"#));
    }

    let result = credential.response.verify(&config.webauthn, &challenge);
    if let Err(err) = result {
        match err {
            Error::BadChallenge => {
                log::trace!("Challenge mismatch");
                return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad challenge" }"#));
            }
            Error::BadOrigin => {
                log::trace!("Origin mismatch");
                return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad origin" }"#));
            }
            _ => {
                log::trace!("Challenge: unexpected error: {}", &err.to_string());
                return Err(err);
            }
        }
    }

    let auth_data = result.unwrap();

    // The response is valid.
    // The authData is returned from the verify function
    let id = Base64UrlSafeData(auth_data.credential_data()?.credential_id);
    let cache_response = service.get_credential(&id).await?;

    if let Some(_creds) = cache_response {
        log::trace!("Credential ID is already used");
        return Ok(HttpResponse::Unauthorized().json(r#"{ "message": "credentialId in use" }"#));
    }
    // Save the credential
    let cred = auth_data.as_credential();
    service.add_credential_for_user(&name, &id, &cred).await?;

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}
