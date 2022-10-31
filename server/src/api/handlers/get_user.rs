use actix_web::{web, HttpResponse};

use crate::{errors::Error, webauthn::model::UserEntity, DataServices};

/// Get a user via HTTP Post
pub async fn get_user(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
) -> Result<HttpResponse, Error> {
    // See if this user already exists.  If so, return 403
    let user = service.get_user(&request.name).await?;
    if user.is_none() {
        // Return already registered
        return Ok(HttpResponse::NotFound().body(format!(
            r#"{{"message": "User not found: {}"}}"#,
            request.name
        )));
    }

    // Return the PK Options
    Ok(HttpResponse::Ok().json(user))
}
