use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use server::{api, webauthn, DataServices};

pub async fn app_state() -> web::Data<DataServices> {
    let services = DataServices::create()
        .await
        .expect("Services failed. Did you remember to start them?");
    web::Data::new(services)
    // App::app_data will wrap the app state in an Arc, so it is sharable
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_state = app_state().await;
    let (api_address, tls_address, ip) = server::get_ip_addresses();

    log::info!(
        "\nHTTP is running on {:?}\nHTTPS is running on {:?}\nIP address is {}",
        &api_address,
        &tls_address,
        &ip
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(webauthn::routes)
            .configure(api::routes)
            .default_service(web::to(server::default_handler))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
