use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};

use rust_webauthn::{webauthn, DataServices};

pub async fn app_state() -> web::Data<DataServices> {
    let services = DataServices::create().await;
    // App::app_data will wrap the app state in an Arc, so it is sharable
    web::Data::new(services)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let secret_key = Key::generate();
    let app_state = app_state().await;

    let (api_address, tls_address, ip) = rust_webauthn::get_ip_addresses();
    log::info!(
        "\nHTTP is running on {:?}\nHTTPS is running on {:?}\nIP address is {}",
        &api_address,
        &tls_address,
        &ip
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // cookie session middleware
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(webauthn::routes)
            .default_service(web::to(rust_webauthn::default_handler))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
