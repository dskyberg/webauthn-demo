use crate::{errors::Error, DataServices};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn check_user(
    path: web::Path<(String,)>,
    service: web::Data<DataServices>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let (name,) = path.into_inner();
    log::info!("Check User Request: {}", &name);

    // See if this user already exists.  If so, return 403
    let result = service.check_user(&name).await.map_err(|_| {
        log::info!("Failed getting user: {}", &name);
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
