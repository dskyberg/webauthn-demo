use crate::config::handlers;
use actix_web::web;

/*
   GET /config
   POST /config
*/
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/config")
            .route(web::get().to(handlers::get_config))
            .route(web::post().to(handlers::post_config)),
    );
}
