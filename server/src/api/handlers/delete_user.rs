use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn delete_user(
    path: web::Path<(String,)>,
    service: web::Data<DataServices>,
) -> Result<HttpResponse, Error> {
    let (name,) = path.into_inner();
    log::trace!("Delete User Request: {}", &name);

    service
        .db
        .delete_user_and_credentials(&name)
        .await
        .map_err(|_| {
            log::trace!("Failed deleting user: {}", &name);
            Error::InternalServiceError("Failed deleting user".to_string())
        })?;

    Ok(HttpResponse::Ok().finish())
}
