use crate::webauthn::model::{PublicKeyCredentialCreationOptions, UserEntity};
use crate::{errors::Error, DataServices};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Result;

pub async fn assertion_challenge(
    session: Session,
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Registration Request: {:?}", &request);

    // See if this user already exists.  If so, return 403
    let user = service
        .get_user(&request.name)
        .await
        .map_err(|_| Error::InternalServiceError("Failed getting user".to_string()))?;
    if user.is_some() {
        // Return already registered
        log::info!("User already exists: {}", request.name);
        return Ok(
            HttpResponse::Forbidden().body(format!("User already registered: {}", request.name))
        );
    }

    // Create the PublicKey Creation Options
    let pk_options =
        PublicKeyCredentialCreationOptions::from_user_entity(&request).map_err(|_| {
            log::info!("Failed to create options from UserEntity");
            Error::InternalServiceError("Failure".to_string())
        })?;

    // Save the user
    log::info!("Saving user entity: {:?}", &pk_options.user);

    service
        .add_user(&pk_options.user)
        .await
        .map_err(|_| Error::SessionError("Failed to add user to cache".to_string()))?;

    // Update the session for the next step (response).
    session
        .insert("name", &request.name)
        .map_err(|_| Error::SessionError("Failed to update name in session".to_string()))?;

    log::info!("storing to session: {}", &pk_options.challenge);

    session
        .insert("challenge", &pk_options.challenge)
        .map_err(|_| {
            log::info!("Failed to insert challenge in session");
            Error::GeneralError
        })?;

    // Return the PK Options
    Ok(HttpResponse::Ok().json(pk_options))
}
