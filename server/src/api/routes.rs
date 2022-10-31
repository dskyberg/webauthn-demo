use super::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
            .service(web::resource("/mds/refresh").route(web::get().to(handlers::refresh_mds)))
            .service(web::resource("/mds/search").route(web::post().to(handlers::search_mds)))
            .service(
                web::resource("/policy")
                    .route(web::get().to(handlers::get_policy))
                    .route(web::patch().to(handlers::patch_policy)),
            )
            .service(
                web::resource("/credentials/user")
                    .route(web::post().to(handlers::get_user_credentials)),
            )
            .service(
                web::resource("/users")
                    .route(web::get().to(handlers::get_users))
                    .route(web::post().to(handlers::get_user)),
            )
            .service(web::resource("/users/check").route(web::post().to(handlers::check_user)))
            .service(web::resource("/users/delete").route(web::post().to(handlers::delete_user)))
            .service(web::resource("/users/logout").route(web::post().to(handlers::logout_user))),
    );
}
