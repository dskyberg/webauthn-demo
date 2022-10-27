use actix_web::{web, HttpResponse};

use crate::{errors::Error, DataServices};

pub async fn get_users(service: web::Data<DataServices>) -> Result<HttpResponse, Error> {
    let result = service.get_users().await;
    match result {
        Ok(users) => {
            log::info!("Sending users: {:?}", &users);
            Ok(HttpResponse::Ok().json(&users))
        }
        Err(e) => {
            log::info!("Sending users - error: {:?}", e.to_string());
            Err(e)
        }
    }
}
