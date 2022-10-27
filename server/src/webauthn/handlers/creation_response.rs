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
    log::info!("PublicKeyCredential: {:?}", &credential);
    let config = service.get_config().await?;

    // Get the session from the request header
    let session = Session::from_request(&service, &req).await.map_err(|e| {
        log::info!("Failed to get session from data service");
        e
    })?;

    if session.is_empty() {
        log::info!("Session is invalid.  No entries");
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "Error getting session" }"#)
        );
    }

    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    // The challenge should have been stored as Base64.  Decode it
    let challenge = session.as_b64("challenge")?;
    let name = session.as_str("name")?;

    // ------------ 7.1 RP verification ----------------//
    // Steps 1 - 6 are either performed in javascript before
    // postint.  Start with step 7

    // 7.1 Step 7
    if credential.type_ != PublicKeyCredentialType::PublicKey {
        log::info!(
            "PublicKeyCredentialType not supported: {:?}",
            credential.type_
        );
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "PublicKeyCredentialTyep type must be 'public-key" }"#));
    }

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
            _ => {
                log::info!("Challenge: unexpected error: {}", &err.to_string());
                return Err(err);
            }
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
    let cred = auth_data.as_credential();
    service.add_credential_for_user(&name, &id, &cred).await?;

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}
