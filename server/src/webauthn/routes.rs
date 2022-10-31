use actix_web::web;

use crate::webauthn::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webauthn")
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
