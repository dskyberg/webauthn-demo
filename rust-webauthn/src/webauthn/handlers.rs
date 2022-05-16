use crate::webauthn::model::{RegistrationChallengeRequest, RegistrationChallengeResponse};
use crate::DataServices;
use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse};

/// Initiate a grant transaction
pub async fn register_challenge_request(
    session: Session,
    _service: web::Data<DataServices>,
    registration: web::Json<RegistrationChallengeRequest>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::trace!("model: {:?}", &registration);
    // Create a challenge
    let challenge =
        RegistrationChallengeResponse::new(&registration.name, &registration.displayName);
    // Create a session token
    session.insert("username", &registration.name)?;
    Ok(HttpResponse::Ok().json(challenge))
}
