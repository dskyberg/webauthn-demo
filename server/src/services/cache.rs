//! Wrapper for Redis cache connections.
//!
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;
use std::fmt::Write as _;

#[derive(Clone)]
pub struct Cache {
    pub client: Client,
    pub connection_manager: ConnectionManager,
}

impl Cache {
    pub async fn new() -> Self {
        let redis_uri = Self::connection();

        let client = Client::open(redis_uri).expect("Failed to open Redis client");

        let connection_manager = client
            .get_tokio_connection_manager()
            .await
            .expect("Can't create Redis connection manager");

        Self {
            client,
            connection_manager,
        }
    }

    /// The URL format is redis://[<username>][:<password>@]<hostname>[:port][/<db>]
    pub fn connection() -> String {
        let host = env::var("REDIS_URI").ok(); //.unwrap_or_else(|_| "localhost:6937".to_owned());
        let scheme = env::var("REDIS_SCHEME").ok(); //.unwrap_or_else(|_| "redis".to_owned());
        let username = env::var("REDIS_PASSWORD").ok();
        let password = env::var("REDIS_PASSWORD").ok();
        let database = env::var("REDIS_DATABASE").ok();

        let mut conn = "".to_string();
        if let Some(s) = scheme {
            conn += &s;
        } else {
            conn += "redis";
        }

        conn += "://";

        if let Some(h) = host {
            conn += &h;
        } else {
            conn += "127.0.0.1";
        }

        if username.is_some() || password.is_some() || database.is_some() {
            conn += "?";
            if username.is_some() {
                let _ = write!(conn, "username={}", username.unwrap());
            }
            if password.is_some() {
                let _ = write!(conn, "password={}", password.unwrap());
            }
            if database.is_some() {
                let _ = write!(conn, "password={}", database.unwrap());
            }
        }
        log::info!("Redis connection string: {}", &conn);
        conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let s = Cache::connection();
        dbg!(&s);
    }
}
