use actix_web::{web, HttpResponse};

use super::*;
use crate::{
    errors,
    webauthn::model::{UserEntity, WebauthnPolicyBuilder},
    DataServices,
};

pub async fn get_policy(service: web::Data<DataServices>) -> Result<HttpResponse, errors::Error> {
    let config: AppConfig = service.get_config().await?;
    log::trace!("Sending policy: {:?}", &config);

    Ok(HttpResponse::Ok().json(&config.webauthn))
}

pub async fn patch_policy(
    service: web::Data<DataServices>,
    policy: web::Json<WebauthnPolicyBuilder>,
) -> Result<HttpResponse, errors::Error> {
    let policy = service.patch_policy(policy.into_inner()).await?;
    Ok(HttpResponse::Ok().json(policy))
}

pub async fn get_users(service: web::Data<DataServices>) -> Result<HttpResponse, errors::Error> {
    let users: Option<Vec<UserEntity>> = service.get_users().await?;
    log::trace!("Sending users: {:?}", &users);

    Ok(HttpResponse::Ok().json(&users))
}
