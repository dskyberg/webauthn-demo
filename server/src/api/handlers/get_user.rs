use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn get_user(
    path: web::Path<(String,)>,
    service: web::Data<DataServices>,
) -> Result<HttpResponse, Error> {
    let (name,) = path.into_inner();
    log::trace!("Get User Request: {}", &name);

    let result = service.db.fetch_user_by_name(&name).await.map_err(|_| {
        log::trace!("Failed getting user: {}", &name);
        Error::InternalServiceError("Failed getting user".to_string())
    })?;
    match result {
        Some(user) => Ok(HttpResponse::Ok().json(&user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
