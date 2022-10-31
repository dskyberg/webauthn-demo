/// Receives a WebAuthn assertion and verifies it.
/// The credential is looked up based on the credential ID passed
/// in the [AssertionPublicKeyCredential].  
///  
use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    errors::Error,
    webauthn::model::{AssertionPublicKeyCredential, PublicKeyCredentialType},
    DataServices, Session,
};

pub async fn assertion_response(
    service: web::Data<DataServices>,
    credential: web::Json<AssertionPublicKeyCredential>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let config = service.get_config().await?;

    // Get the session from the request header
    let mut session = Session::from_request(&service, &req).await.map_err(|e| {
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

    // let name = session.as_str("name")?;

    // ------------ 7.1 RP verification ----------------//
    // Steps 1 - 6 are either performed in javascript before
    // postint.  Start with step 7

    // 7.1 Step 7
    if credential.type_ != PublicKeyCredentialType::PublicKey {
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "response type must be 'public-key" }"#));
    }

    // Get the credential from the data store
    //let cred = service.get_user_credential(&name).await?.unwrap();
    let result = service.get_credential(&credential.id).await?;
    if result.is_none() {
        return Ok(HttpResponse::NotFound().json(r#"{ "message": "Credential not found" }"#));
    }
    let cred = result.unwrap();

    // Verify the response
    let result = credential
        .response
        .verify(&config.webauthn, &challenge, &cred);

    match result {
        Err(err) => match err {
            Error::BadChallenge => {
                log::trace!("Challenge mismatch");
                Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad challenge" }"#))
            }
            Error::BadOrigin => {
                log::trace!("Origin mismatch");
                Ok(HttpResponse::Unauthorized().json(r#"{ "message": "bad origin" }"#))
            }
            _ => Err(err),
        },
        Ok(credential) => {
            session.insert("authenticated", "true");

            // Update the credential so that the counter and date stuff is right.
            service.update_credential(&credential).await?;
            Ok(HttpResponse::Ok()
                .insert_header(session.to_header())
                .json(r#"{"status": "ok"}"#))
        }
    }
}
