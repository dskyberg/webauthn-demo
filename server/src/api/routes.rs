use super::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
            .service(
                web::resource("/policy")
                    .route(web::get().to(handlers::get_policy))
                    .route(web::patch().to(handlers::patch_policy)),
            )
            .service(
                web::resource("/credentials/user")
                    .route(web::post().to(handlers::get_user_credentials)),
            )
            .service(web::resource("/users").route(web::get().to(handlers::get_users)))
            .service(
                web::resource("/users/{name}")
                    .route(web::head().to(handlers::check_user))
                    .route(web::delete().to(handlers::delete_user))
                    .route(web::get().to(handlers::get_user)),
            )
            .service(web::resource("/credentials")),
    );
}
