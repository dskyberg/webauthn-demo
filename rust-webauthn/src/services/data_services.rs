use anyhow::Result;
use redis::{AsyncCommands, Value};

use super::Cache;
use crate::{errors::Error, webauthn::model::UserEntity};

// Service wrapper for cache and database
///
/// The data persistence is managed via MongoDB. The dao lib provides an
/// abstraction level between the REST handlers and the database.
/// The dao lib defines a Service that encapsulates the [GnapDB] and the [GnapCache].
///
#[derive(Clone)]
pub struct DataServices {
    /// Represents the Redis cache client
    pub cache: Cache,
}

impl DataServices {
    /// Establishes the client connections to the database and cache.
    ///
    /// This should be called only once in the crate main.
    pub async fn create() -> DataServices {
        let cache = Cache::new().await;
        DataServices { cache }
    }

    pub async fn get_user(&self, user_name: &str) -> Result<Option<UserEntity>> {
        let cache_key = format!("{}:{}", "users", user_name);
        let mut con = self.cache.client.get_async_connection().await?;
        let cache_response = con.get(&cache_key).await?;
        match cache_response {
            Value::Nil => Ok(None),
            Value::Data(val) => Ok(serde_json::from_slice(&val)?),
            _ => Err(Error::GeneralError.into()),
        }
    }

    /// Dynamically create a client
    pub async fn add_user(&self, user: &UserEntity) -> Result<()> {
        let mut con = self.cache.client.get_async_connection().await?;
        let cache_key = format!("{}:{}", "users", user.name).to_owned();
        let _: () = redis::pipe()
            .atomic()
            .set(&cache_key, serde_json::to_vec(&user)?)
            .query_async(&mut con)
            .await?;

        Ok(())
    }
}
