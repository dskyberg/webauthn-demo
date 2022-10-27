use base64urlsafedata::Base64UrlSafeData;
use redis::{AsyncCommands, Value};

use super::{Cache, SessionData, User, DB};
use crate::{
    config::AppConfig,
    errors::Error,
    webauthn::model::{Credential, UserEntity, WebauthnPolicy, WebauthnPolicyBuilder},
};

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
    pub db: DB,
}

impl DataServices {
    /// Establishes the client connections to the database and cache.
    ///
    /// This should be called only once in the crate main.
    pub async fn create() -> DataServices {
        let cache = Cache::new().await;
        let db = DB::new().await;
        DataServices { cache, db }
    }

    pub async fn get_config(&self) -> Result<AppConfig, Error> {
        let result = self.db.fetch_config().await?;
        match result {
            Some(config) => Ok(config),
            None => {
                let config = AppConfig::default();
                self.put_config(&config).await?;
                Ok(config)
            }
        }
    }

    pub async fn put_config(&self, config: &AppConfig) -> Result<(), Error> {
        self.db.put_config(config).await
    }

    pub async fn patch_policy(
        &self,
        policy: WebauthnPolicyBuilder,
    ) -> Result<WebauthnPolicy, Error> {
        let mut config = self.get_config().await?;
        log::info!("Patch set: {:?}", &policy);
        config.webauthn.update(policy)?;
        log::info!("Updated policy: {:?}", &config);
        self.put_config(&config).await?;
        Ok(config.webauthn)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let users = self.db.fetch_user_ids().await?;
        Ok(users)
    }

    pub async fn check_user(&self, name: &str) -> Result<bool, Error> {
        self.db.check_user_by_name(name).await
    }

    /// Fetch a [User](super::User) by name and convert to a [UserEntity].
    pub async fn get_user(&self, name: &str) -> Result<Option<UserEntity>, Error> {
        let user = self.db.fetch_user_by_name(name).await?;
        match user {
            Some(u) => Ok(Some(u.as_user_entity())),
            None => Ok(None),
        }
    }

    /// Add a user
    pub async fn add_user(&self, user: &UserEntity) -> Result<(), Error> {
        self.db.add_user(user).await?;
        Ok(())
    }

    /// Get a [Credential] by id.
    pub async fn get_credential(
        &self,
        id: &Base64UrlSafeData,
    ) -> Result<Option<Credential>, Error> {
        self.db.fetch_credential_by_id(&id.to_string()).await
    }

    /// Associate a stored credential with a stored user.
    /// For now, [User] has a `credentials` attribute which is a set of credential ids.
    /// Note: this is the [Base64UrlSafeData] from the Creential.  Not a Bson `_id`
    pub async fn add_credential_for_user(
        &self,
        name: &str,
        _id: &Base64UrlSafeData,
        cred: &Credential,
    ) -> Result<(), Error> {
        self.db.add_credential_for_user(name, cred).await?;
        Ok(())
    }

    pub async fn update_credential(&self, cred: &Credential) -> Result<(), Error> {
        self.db.update_credential(cred).await?;
        Ok(())
    }

    /// TODO: Reverse fetching - we need to be able to fetch a user from a credential,
    /// since a user may have more than 1.
    pub async fn get_user_credential(&self, name: &str) -> Result<Option<Credential>, Error> {
        let user_result = self.db.fetch_user_by_name(name).await?;

        if user_result.is_none() {
            return Err(Error::NotFound);
        }
        let user = user_result.unwrap();
        if user.credentials.is_none() {
            return Err(Error::NotFound);
        }
        let cred_ids = user.credentials.unwrap();
        if cred_ids.is_empty() {
            return Err(Error::NotFound);
        }
        self.get_credential(&cred_ids[0]).await
    }

    pub async fn put_session(
        &self,
        id: &Base64UrlSafeData,
        data: &SessionData,
    ) -> Result<(), Error> {
        let cache_key = format!("{}:{}", SESSIONS_KEY, id).to_owned();
        let mut con = self.cache.client.get_async_connection().await?;
        let data = serde_json::to_vec(data).map_err(Error::SerdeJsonError)?;
        con.set(&cache_key, data).await?;

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
