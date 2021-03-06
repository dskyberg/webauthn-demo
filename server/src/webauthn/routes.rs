use actix_web::web;

use crate::webauthn::handlers;
/*
/webauthn
    /users
    /assertion
        /challenge
        /response
    /credential
        /challenge
        /response
 */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webauthn")
            //.service(web::resource("/user").route(web::post().to(handlers::get_user)))
            .service(web::resource("/user/{name}").route(web::head().to(handlers::check_user)))
            .service(
                web::resource("/user/credentials")
                    .route(web::post().to(handlers::get_user_credentials)),
            )
            .service(
                web::scope("/credential")
                    .service(
                        web::resource("/challenge")
                            .route(web::post().to(handlers::creation_challenge)),
                    )
                    .service(
                        web::resource("/response")
                            .route(web::post().to(handlers::creation_response)),
                    ),
            )
            .service(
                web::scope("/assertion")
                    .service(
                        web::resource("/challenge")
                            .route(web::post().to(handlers::assertion_challenge)),
                    )
                    .service(
                        web::resource("/response")
                            .route(web::post().to(handlers::assertion_response)),
                    ),
            ),
    );
}
