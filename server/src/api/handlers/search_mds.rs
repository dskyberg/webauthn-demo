use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn search_mds(
    service: web::Data<DataServices>,
    request: web::Json<serde_json::Map<String, serde_json::Value>>,
) -> Result<HttpResponse, Error> {
    log::trace!("Search FIDO MDS");
    let result = service.search_mds(&request).await?;

    Ok(HttpResponse::Ok().json(result))
}
