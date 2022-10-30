//! Wrapper for Redis cache connections.
//!
use base64urlsafedata::Base64UrlSafeData;
use redis::{aio::ConnectionManager, AsyncCommands, Client, Value};
use std::env;

use super::SessionData;
use crate::errors::Error;

const SESSIONS_KEY: &str = "sessions";

#[derive(Clone)]
pub struct Cache {
    pub client: Client,
    pub connection_manager: ConnectionManager,
}

impl Cache {
    pub async fn create() -> Result<Self, Error> {
        let redis_uri = Self::connection();

        let client = Client::open(redis_uri)
            .map_err(|_| Error::ServiceError("Redis: Failed to create client".to_string()))?;

        let connection_manager = client.get_tokio_connection_manager().await.map_err(|_| {
            Error::ServiceError("Redis: Failed to create connection manager".to_string())
        })?;

        Ok(Self {
            client,
            connection_manager,
        })
    }

    /// The URL format is redis://[<username>][:<password>@]<hostname>[:port][/<db>]
    pub fn connection() -> String {
        let conn = env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1".to_owned());
        log::trace!("Redis connection string: {}", &conn);
        conn
    }

    pub async fn put_session(
        &self,
        id: &Base64UrlSafeData,
        data: &SessionData,
    ) -> Result<(), Error> {
        let cache_key = format!("{}:{}", SESSIONS_KEY, id).to_owned();
        let mut con = self.client.get_async_connection().await?;
        let data = serde_json::to_vec(data).map_err(Error::SerdeJsonError)?;
        con.set(&cache_key, data).await?;

        Ok(())
    }

    pub async fn fetch_session(
        &self,
        id: &Base64UrlSafeData,
    ) -> Result<Option<SessionData>, Error> {
        let cache_key = format!("{}:{}", SESSIONS_KEY, id).to_owned();
        let mut con = self.client.get_async_connection().await?;
        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => Ok(None),
            Value::Data(val) => Ok(serde_json::from_slice(&val).map_err(Error::SerdeJsonError)?),
            _ => Err(Error::GeneralError),
        }
    }
}
