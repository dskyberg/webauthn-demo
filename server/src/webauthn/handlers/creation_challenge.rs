//use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::webauthn::model::{PublicKeyCredentialCreationOptions, UserEntity};
use crate::{errors::Error, services::Session, DataServices};

pub async fn creation_challenge(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Registration Request: {:?}", &request);
    let config = service.get_config().await?;

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
    let user = request.into_inner();

    // Create the PublicKey Creation Options
    let pk_options = PublicKeyCredentialCreationOptions::try_from((&config.webauthn, &user))
        .map_err(|_| {
            log::info!("Failed to create options from UserEntity");
            Error::InternalServiceError("Failure".to_string())
        })?;

    // Save the user
    log::info!("Saving user entity: {:?}", &pk_options.user);

    service
        .add_user(&pk_options.user)
        .await
        .map_err(|_| Error::SessionError("Failed to add user to cache".to_string()))?;

    // Create a session for the next step (response).
    let mut session = Session::default();

    session.insert("name", &user.name);
    session.insert("challenge", &pk_options.challenge.to_string());
    session.put_session(&service).await?;
    let header = session.to_header();
    // Return the PK Options
    Ok(HttpResponse::Ok().insert_header(header).json(pk_options))
}
