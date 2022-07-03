use crate::webauthn::model::{PublicKeyCredentialRequestOptions, UserEntity};
use crate::{errors::Error, services::Session, DataServices};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn assertion_challenge(
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

    // Create a session for the next step (response).
    let mut session = Session::default();

    session.insert("name", &user.name);
    session.insert("challenge", &pk_options.challenge.to_string());
    session.put_session(&service).await?;
    let header = session.to_header();
    // Return the PK Options
    Ok(HttpResponse::Ok().insert_header(header).json(pk_options))
}
