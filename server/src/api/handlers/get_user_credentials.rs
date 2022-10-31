use actix_web::{web, HttpResponse};

use crate::{errors::Error, webauthn::model::UserEntity, DataServices};

pub async fn get_user_credentials(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
) -> Result<HttpResponse, Error> {
    // See if this user already exists.  If so, return 403
    let cred = service.get_user_credential(&request.name).await?;
    if cred.is_none() {
        // Return already registered
        return Ok(HttpResponse::NotFound().body(format!(
            r#"{{"message": "User credential not found: {}"}}"#,
            request.name
        )));
    }

    // Return the PK Options
    Ok(HttpResponse::Ok().json(cred))
}
