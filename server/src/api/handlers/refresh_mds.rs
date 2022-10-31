use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn refresh_mds(service: web::Data<DataServices>) -> Result<HttpResponse, Error> {
    log::trace!("Fetching FIDO MDS");
    service.refresh_mds().await?;

    Ok(HttpResponse::Ok().finish())
}
