/// Logging does not actually do anything.  If this app used session tokens,
/// then logging out would clean up or delete the token.
///
use actix_web::{web, HttpResponse};

use crate::{errors::Error, webauthn::model::UserEntity, DataServices};

/// Get a user via HTTP Post
pub async fn logout_user(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
) -> Result<HttpResponse, Error> {
    // See if this user already exists.  If so, return 403
    let user = service.check_user(&request.name).await?;
    if !user {
        // Return already registered
        return Ok(HttpResponse::NotFound().body(format!(
            r#"{{"message": "User not found: {}"}}"#,
            request.name
        )));
    }

    // Return the PK Options
    Ok(HttpResponse::Ok().finish())
}
