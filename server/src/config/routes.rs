use crate::config::handlers;
use actix_web::web;

/*
   GET /config
   POST /config
*/
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/policy")
            .route(web::get().to(handlers::get_policy))
            .route(web::patch().to(handlers::patch_policy)),
    );
}
