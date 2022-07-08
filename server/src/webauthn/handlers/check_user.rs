use crate::webauthn::model::UserEntity;
use crate::{errors::Error, DataServices};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn check_user(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Check User Request: {:?}", &request);

    // See if this user already exists.  If so, return 403
    let result = service.check_user(&request.name).await.map_err(|_| {
        log::info!("Failed getting user: {}", &request.name);
        Error::InternalServiceError("Failed getting user".to_string())
    })?;
    if !result {
        // Return already registered
        return Ok(HttpResponse::NotFound().body(format!(
            r#"{{"message": "User not found: {}"}}"#,
            request.name
        )));
    }

    // Return the PK Options
    Ok(HttpResponse::Ok().finish())
}
