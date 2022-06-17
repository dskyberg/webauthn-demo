//use crate::webauthn::model::UserEntity;

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use base64urlsafedata::Base64UrlSafeData;

use crate::{
    errors::Error,
    webauthn::model::{AssertionPublicKeyCredential, PublicKeyCredentialType},
    DataServices,
};

pub async fn assertion_response(
    session: Session,
    service: web::Data<DataServices>,
    credential: web::Json<AssertionPublicKeyCredential>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    //log::info!("PublicKeyCredential: {:?}", &credential);
    let config = service.config().await?;

    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    // The challenge should have been stored as Base64.  Decode it
    let challenge = b64_from_session(&session, "challenge");
    if challenge.is_err() {
        log::info!("Error getting challenge");
        return Ok(
            HttpResponse::InternalServerError().json(r#"{ "message": "Error getting challenge" }"#)
        );
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

fn b64_from_session(session: &Session, name: &str) -> Result<Base64UrlSafeData, Error> {
    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    match session
        .get::<String>(name)
        .map_err(|_| Error::SessionError(format!("Failed to get {} from session", name)))?
    {
        Some(val) => {
            let x = Base64UrlSafeData::try_from(val.as_str())
                .map_err(|_| Error::Base64UrlSafeDataError)?;
            Ok(x)
        }
        None => Err(Error::SessionError(format!(
            "Failed to get {} from session",
            name
        ))),
    }
}
