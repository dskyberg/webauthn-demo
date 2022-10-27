use actix_web::{web, HttpResponse};

use crate::{config::AppConfig, errors::Error, DataServices};

pub async fn get_policy(service: web::Data<DataServices>) -> Result<HttpResponse, Error> {
    let config: AppConfig = service.get_config().await?;
    log::trace!("Sending policy: {:?}", &config);

    Ok(HttpResponse::Ok().json(&config.webauthn))
}
