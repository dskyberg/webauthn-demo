use crate::webauthn::model::UserEntity;
use crate::{errors, DataServices};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn register_challenge_response(
    _session: Session,
    _service: web::Data<DataServices>,
    response: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, errors::Error> {
    log::info!("Registration Request: {:?}", &response);

    Ok(HttpResponse::Ok().json(&response))
}
