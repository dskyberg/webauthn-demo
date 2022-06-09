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
            .service(web::resource("/user").route(web::post().to(handlers::check_user)))
            .service(
                web::scope("/credential")
                    .service(
                        web::resource("/challenge")
                            .route(web::post().to(handlers::credential_challenge)),
                    )
                    .service(
                        web::resource("/response")
                            .route(web::post().to(handlers::credential_response)),
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
