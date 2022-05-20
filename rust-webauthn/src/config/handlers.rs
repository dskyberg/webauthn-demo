use crate::{config::AppConfig, errors, DataServices};
use actix_web::{web, HttpResponse};

pub async fn get_config(_service: web::Data<DataServices>) -> Result<HttpResponse, errors::Error> {
    let config: AppConfig = Default::default();

    Ok(HttpResponse::Ok().json(&config))
}

pub async fn post_config(
    _service: web::Data<DataServices>,
    config: web::Json<AppConfig>,
) -> Result<HttpResponse, errors::Error> {
    Ok(HttpResponse::Ok().json(&config))
}
