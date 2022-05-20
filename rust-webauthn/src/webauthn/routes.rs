use crate::webauthn::handlers;
use actix_web::web;

/*
     credentialEndpoint: '/register',
     assertionEndpoint: '/login',
     challengeEndpoint: '/response',
     logoutEndpoint: '/logout',
*/
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webauthn")
            .service(
                web::resource("/register")
                    .route(web::post().to(handlers::register_challenge_request)),
            )
            .service(
                web::resource("/response")
                    .route(web::post().to(handlers::register_challenge_response)),
            ),
    );
}
