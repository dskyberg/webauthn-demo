//use crate::webauthn::model::UserEntity;
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
    //log::info!("PublicKeyCredential: {:?}", &credential);
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
        // Bad type attribute
        return Ok(HttpResponse::BadRequest()
            .json(r#"{ "message": "response type must be 'public-key" }"#));
    }

    // Get the credential from the data store
    let cred = service.get_user_credential(&name).await?.unwrap();

    // Verify the response

    let result = credential
        .response
        .verify(&config.webauthn, &challenge, &cred);
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

    Ok(HttpResponse::Ok().json(r#"{"status": "ok"}"#))
}
