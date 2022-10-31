use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn get_users(service: web::Data<DataServices>) -> Result<HttpResponse, Error> {
    let result = service.get_users().await;
    match result {
        Ok(users) => {
            log::trace!("Sending users: {:?}", &users);
            Ok(HttpResponse::Ok().json(&users))
        }
        Err(e) => {
            log::trace!("Sending users - error: {:?}", e.to_string());
            Err(e)
        }
    }
}
