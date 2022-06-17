use crate::webauthn::model::{PublicKeyCredentialRequestOptions, UserEntity};
use crate::{errors::Error, DataServices};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn assertion_challenge(
    session: Session,
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Registration Request: {:?}", &request);
    let config = service.get_config().await?;

    // Get the user by name.  If not found, return 403
    let user = service
        .get_user(&request.name)
        .await
        .map_err(|_| Error::InternalServiceError("Failed getting user".to_string()))?;
    if user.is_none() {
        // Return already registered
        log::info!("User not found: {}", request.name);
        return Ok(HttpResponse::Forbidden().body(format!("User not found: {}", request.name)));
    }
    let user = user.unwrap();
    // Get the credential id's for this user
    let credential = service.get_user_credential(&user.name).await.map_err(|_| {
        Error::InternalServiceError("Failed getting credential for user".to_string())
    })?;
    if credential.is_none() {
        // Return: user has no credential registered
        log::info!("Credential not found for user: {}", request.name);
        return Ok(
            HttpResponse::Forbidden().body(format!("Credential not found: {}", request.name))
        );
    }
    let credential = credential.unwrap();

    // Create the PublicKey Creation Options
    let pk_options = PublicKeyCredentialRequestOptions::try_from((&config.webauthn, &credential))
        .map_err(|_| {
        log::info!("Failed to create options from UserEntity");
        Error::InternalServiceError("Failure".to_string())
    })?;

    // Update the session for the next step (response).
    session.clear();
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
