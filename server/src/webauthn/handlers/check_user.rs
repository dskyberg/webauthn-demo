use crate::webauthn::model::UserEntity;
use crate::{errors::Error, DataServices};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn check_user(
    session: Session,
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Check User Request: {:?}", &request);

    // See if this user already exists.  If so, return 403
    let user = service.get_user(&request.name).await.map_err(|_| {
        log::info!("Failed getting user: {}", &request.name);
        Error::InternalServiceError("Failed getting user".to_string())
    })?;
    if user.is_none() {
        // Return already registered
        return Ok(HttpResponse::NotFound().body(format!(
            r#"{{"message": "User not found: {}"}}"#,
            request.name
        )));
    }

    // Update the session for the next step (response).
    session
        .insert("name", &request.name)
        .map_err(|_| Error::SessionError("Failed to update name in session".to_string()))?;

    // Return the PK Options
    Ok(HttpResponse::Ok().json(user))
}
