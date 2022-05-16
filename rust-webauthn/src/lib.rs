use std::env;
use std::net::SocketAddr;

use actix_files::NamedFile;
use actix_web::{
    http::{Method, StatusCode},
    Either, HttpResponse, Responder, Result,
};

pub use data_services::DataServices;
pub mod data_services;
pub mod utils;
pub mod webauthn;

/// Get the machine IP Address
/// Get the IP from a non-loopback interface and return as a string.
pub fn get_machine_ip() -> String {
    let addrs = get_if_addrs::get_if_addrs().unwrap();
    let ips = addrs
        .into_iter()
        .filter(|n| n.name != "lo0")
        .collect::<Vec<_>>();

    format!(" {:?}", ips[0].addr.ip())
}

/// Get addresses from ENV
///
/// This doesn't really havea ny value.  But fun to play with. We could just
/// as easily pass the string from env::var into the HttpServer.bind func.
pub fn get_ip_addresses() -> (SocketAddr, SocketAddr, String) {
    let api_address: SocketAddr = env::var("API_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3001".to_string())
        .parse()
        .expect("API_ADDRESS is invalid");

    let tls_address: SocketAddr = env::var("TLS_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3443".to_string())
        .parse()
        .expect("TLS_ADDRESS is invalid");

    // Get the local IP address of the non-loopback interface. This is just for
    // displaying at startup.
    let ip = get_machine_ip();

    (api_address, tls_address, ip)
}

pub async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open_async("./static/404.html")
                .await?
                .set_status_code(StatusCode::NOT_FOUND);
            log::trace!("Handling with default GET handler");
            Ok(Either::Left(file))
        }
        _ => {
            log::trace!("Handling with default UNMATCHED handler");
            Ok(Either::Right(HttpResponse::MethodNotAllowed().finish()))
        }
    }
}
