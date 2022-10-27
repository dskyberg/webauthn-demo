use actix_web::{web, HttpResponse};

use crate::{errors::Error, webauthn::model::WebauthnPolicyBuilder, DataServices};

pub async fn patch_policy(
    service: web::Data<DataServices>,
    policy: web::Json<WebauthnPolicyBuilder>,
) -> Result<HttpResponse, Error> {
    let policy = service.patch_policy(policy.into_inner()).await?;
    Ok(HttpResponse::Ok().json(policy))
}
