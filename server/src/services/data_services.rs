use base64urlsafedata::Base64UrlSafeData;
use redis::{AsyncCommands, Value};

use super::{Cache, SessionData};
use crate::{
    config::AppConfig,
    errors::Error,
    webauthn::model::{Credential, UserEntity, WebauthnPolicy, WebauthnPolicyBuilder},
};

const USERS_KEY: &str = "users";
const CREDS_KEY: &str = "credentials";
const APP_CONFIG: &str = "appconfig";
const SESSIONS_KEY: &str = "sessions";

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

    pub async fn get_config(&self) -> Result<AppConfig, Error> {
        let cache_key = APP_CONFIG;
        let mut con = self
            .cache
            .client
            .get_async_connection()
            .await
            .map_err(|x| {
                log::info!("Redist connection failed: {:?}", &x);
                x
            })?;
        let cache_response = con.get(&cache_key).await.map_err(|x| {
            log::info!("Redis get failed: {:?}", &x);
            x
        })?;
        match cache_response {
            Value::Nil => {
                // No config in the cache.  Store the default
                let config = AppConfig::default();
                self.put_config(&config).await.map_err(|e| {
                    log::info!("Failed updating config");
                    e
                })?;
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
        let mut con = self.cache.client.get_async_connection().await?;
        let cache_key = APP_CONFIG;
        let _: () = redis::pipe()
            .atomic()
            .set(
                &cache_key,
                serde_json::to_vec(&config).map_err(Error::SerdeJsonError)?,
            )
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn patch_policy(
        &self,
        policy: WebauthnPolicyBuilder,
    ) -> Result<WebauthnPolicy, Error> {
        let mut config = self.get_config().await?;
        config.webauthn.update(policy)?;
        self.put_config(&config).await?;
        Ok(config.webauthn)
    }

    pub async fn get_user(&self, user_name: &str) -> Result<Option<UserEntity>, Error> {
        log::trace!("get_user - getting: {}", user_name);
        let cache_key = format!("{}:{}", USERS_KEY, user_name);
        let mut con = self
            .cache
            .client
            .get_async_connection()
            .await
            .map_err(|x| {
                log::info!("Redist connection failed: {:?}", &x);
                x
            })?;
        let cache_response = con.get(&cache_key).await.map_err(|x| {
            log::info!("Redis get failed: {:?}", &x);
            x
        })?;
        match cache_response {
            Value::Nil => {
                log::info!("get_user: not found");
                Ok(None)
            }
            Value::Data(val) => {
                log::info!("get_user: found {:?}", &val);
                Ok(serde_json::from_slice(&val).map_err(Error::SerdeJsonError)?)
            }
            _ => Err(Error::GeneralError),
        }
    }

    /// Create a user
    pub async fn add_user(&self, user: &UserEntity) -> Result<(), Error> {
        let mut con = self.cache.client.get_async_connection().await?;
        let cache_key = format!("{}:{}", USERS_KEY, user.name).to_owned();
        let _: () = redis::pipe()
            .atomic()
            .set(
                &cache_key,
                serde_json::to_vec(&user).map_err(Error::SerdeJsonError)?,
            )
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    async fn add_user_cred(&self, name: &str, id: &Base64UrlSafeData) -> Result<(), Error> {
        let cache_key = format!("{}:{}:{}", USERS_KEY, CREDS_KEY, name).to_owned();
        let mut con = self.cache.client.get_async_connection().await?;

        let _: () = redis::pipe()
            .atomic()
            .set(
                &cache_key,
                serde_json::to_vec(id).map_err(Error::SerdeJsonError)?,
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    pub async fn get_credential(
        &self,
        id: &Base64UrlSafeData,
    ) -> Result<Option<Credential>, Error> {
        let cache_key = format!("{}:{}", CREDS_KEY, id).to_owned();

        let mut con = self.cache.client.get_async_connection().await?;

        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => Ok(None),
            Value::Data(val) => Ok(serde_json::from_slice(&val).map_err(Error::SerdeJsonError)?),
            _ => Err(Error::GeneralError),
        }
    }

    /// Create a credential.
    pub async fn add_credential_for_user(
        &self,
        name: &str,
        id: &Base64UrlSafeData,
        cred: &Credential,
    ) -> Result<(), Error> {
        let mut con = self.cache.client.get_async_connection().await?;
        let cache_key = format!("{}:{}", CREDS_KEY, id).to_owned();

        let _: () = redis::pipe()
            .atomic()
            .set(
                &cache_key,
                serde_json::to_vec(&cred).map_err(Error::SerdeJsonError)?,
            )
            .query_async(&mut con)
            .await?;

        // Bind the key to the user
        self.add_user_cred(name, id).await?;

        Ok(())
    }

    pub async fn get_user_credential_id(
        &self,
        name: &str,
    ) -> Result<Option<Base64UrlSafeData>, Error> {
        let cache_key = format!("{}:{}:{}", USERS_KEY, CREDS_KEY, name).to_owned();
        let mut con = self.cache.client.get_async_connection().await?;

        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => Ok(None),
            Value::Data(val) => Ok(serde_json::from_slice(&val).map_err(Error::SerdeJsonError)?),
            _ => Err(Error::GeneralError),
        }
    }

    pub async fn get_user_credential(&self, name: &str) -> Result<Option<Credential>, Error> {
        match self.get_user_credential_id(name).await? {
            Some(credential_id) => self.get_credential(&credential_id).await,
            _ => Ok(None),
        }
    }

    pub async fn put_session(
        &self,
        id: &Base64UrlSafeData,
        data: &SessionData,
    ) -> Result<(), Error> {
        let cache_key = format!("{}:{}", SESSIONS_KEY, id).to_owned();
        let mut con = self.cache.client.get_async_connection().await?;

        let _: () = redis::pipe()
            .atomic()
            .set(
                &cache_key,
                serde_json::to_vec(data).map_err(Error::SerdeJsonError)?,
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    pub async fn get_session(&self, id: &Base64UrlSafeData) -> Result<Option<SessionData>, Error> {
        let cache_key = format!("{}:{}", SESSIONS_KEY, id).to_owned();

        let mut con = self.cache.client.get_async_connection().await?;

        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => Ok(None),
            Value::Data(val) => Ok(serde_json::from_slice(&val).map_err(Error::SerdeJsonError)?),
            _ => Err(Error::GeneralError),
        }
    }
}
