//! Wrapper for Redis cache connections.
//!
use redis::{aio::ConnectionManager, AsyncCommands, Client, Value};
use std::env;
use std::fmt::Write as _;

use crate::{config::AppConfig, errors::Error};

const APP_CONFIG: &str = "appconfig";
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

    fn format_user() -> String {
        let mut conn = "".to_string();
        let username = env::var("REDIS_USER");
        let password = env::var("REDIS_PASSWORD");
        if let Ok(user) = username {
            let _ = write!(conn, "{}", user);
            if let Ok(pwd) = password {
                let _ = write!(conn, ":{}", pwd);
            }
            let _ = write!(conn, "@");
        }
        conn
    }

    fn format_db() -> String {
        let mut conn = "".to_string();
        let database = env::var("REDIS_DATABASE");
        if let Ok(db) = database {
            let _ = write!(conn, "/{}", db);
        }
        conn
    }

    fn format_port() -> String {
        let mut conn = "".to_string();

        if let Ok(port) = env::var("REDIS_PORT") {
            let _ = write!(conn, ":{}", port);
        }
        conn
    }
    /// The URL format is redis://[<username>][:<password>@]<hostname>[:port][/<db>]
    pub fn connection() -> String {
        let scheme = env::var("REDIS_SCHEME").unwrap_or_else(|_| "redis".to_owned());
        let host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
        let conn = format!(
            "{}://{}{}{}{}",
            scheme,
            Self::format_user(),
            host,
            Self::format_port(),
            Self::format_db()
        );

        log::info!("Redis connection string: {}", &conn);
        conn
    }

    pub async fn fetch_config(&self) -> Result<AppConfig, Error> {
        let cache_key = APP_CONFIG;
        let con_result = self.client.get_async_connection().await;
        if let Err(err) = con_result {
            log::info!("Connection error: {}", &err.to_string());
            return Err(err.into());
        }

        let mut con = con_result.unwrap();

        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => {
                // No config in the cache.  Store the default
                let config = AppConfig::default();
                self.put_config(&config).await?;
                Ok(config)
            }
            Value::Data(val) => Ok(serde_json::from_slice(&val).map_err(|e| {
                log::info!("Failed to parse policy: {:?}", &e);
                Error::SerdeJsonError(e)
            })?),
            _ => Err(Error::GeneralError),
        }
    }

    pub async fn put_config(&self, config: &AppConfig) -> Result<(), Error> {
        let cache_key = APP_CONFIG;
        let mut con = self.client.get_async_connection().await?;
        let data = serde_json::to_vec(&config).map_err(Error::SerdeJsonError)?;
        con.set(&cache_key, data).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    #[test]
    fn test_it() {
        dotenv().ok();
        let s = Cache::connection();
        dbg!(&s);
    }
}
