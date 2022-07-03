use actix_web::{web, HttpResponse};

use super::*;
use crate::{errors, webauthn::model::WebauthnPolicyBuilder, DataServices};

pub async fn get_policy(service: web::Data<DataServices>) -> Result<HttpResponse, errors::Error> {
    let config: AppConfig = service.get_config().await?;

    Ok(HttpResponse::Ok().json(&config.webauthn))
}

pub async fn patch_policy(
    service: web::Data<DataServices>,
    policy: web::Json<WebauthnPolicyBuilder>,
) -> Result<HttpResponse, errors::Error> {
    let policy = service.patch_policy(policy.into_inner()).await?;
    Ok(HttpResponse::Ok().json(policy))
}
