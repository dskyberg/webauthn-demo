//! Wrapper for Redis cache connections.
//!
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

#[derive(Clone)]
pub struct Cache {
    pub client: Client,
    pub connection_manager: ConnectionManager,
}

impl Cache {
    pub async fn new() -> Self {
        let redis_uri =
            env::var("REDIS_URI").unwrap_or("redis://localhost?password=StrongPassword".to_owned());

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
}
