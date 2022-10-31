//use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::webauthn::model::{PublicKeyCredentialCreationOptions, UserEntity};
use crate::{errors::Error, services::Session, DataServices};

pub async fn creation_challenge(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::trace!("Registration Request: {:?}", &request);
    let config = service.get_config().await?;

    // See if this user already exists.  If so, return 403
    if service.check_user(&request.name).await? {
        // Return already registered
        log::trace!("User already exists: {}", request.name);
        return Ok(
            HttpResponse::Forbidden().body(format!("User already registered: {}", request.name))
        );
    }

    let user = request.into_inner();

    // Create a challenge, and save it.
    let challenge = service.create_new_challenge().await?;

    // Create the PublicKey Creation Options
    let pk_options =
        PublicKeyCredentialCreationOptions::try_from((&config.webauthn, &user, &challenge.value))?;

    // Save the user
    log::trace!("Saving user entity: {:?}", &pk_options.user);

    service.add_user(&pk_options.user).await?;

    // Create a session for the next step (response).
    let session = Session::default()
        .with("name", &user.name)
        .with("challenge", &pk_options.challenge.to_string());
    session.put_session(&service).await?;
    let header = session.to_header();
    // Return the PK Options
    Ok(HttpResponse::Ok().insert_header(header).json(pk_options))
}
