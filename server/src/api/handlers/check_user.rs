use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn check_user(
    path: web::Path<(String,)>,
    service: web::Data<DataServices>,
) -> Result<HttpResponse, Error> {
    let (name,) = path.into_inner();
    log::trace!("Check User Request: {}", &name);

    // See if this user already exists.  If so, return 403
    let result = service.check_user(&name).await.map_err(|_| {
        log::trace!("Failed getting user: {}", &name);
        Error::InternalServiceError("Failed getting user".to_string())
    })?;
    if !result {
        // Return already registered
        return Ok(
            HttpResponse::NotFound().body(format!(r#"{{"message": "User not found: {}"}}"#, &name))
        );
    }

    // Return the PK Options
    Ok(HttpResponse::Ok().finish())
}
