//! Wrapper for MongoDB connections.
//!
use base64urlsafedata::Base64UrlSafeData;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::ClientOptions, results::InsertOneResult, Client, Collection, Database,
};
use std::env;

use super::{Challenge, User};
use crate::{
    config::AppConfig,
    errors::Error,
    webauthn::model::{Credential, UserEntity},
};

const CRED_COLLECTION: &str = "credentials";
const USER_COLLECTION: &str = "users";
const APP_CONFIG_COLLECTION: &str = "appconfig";
const WEBAUTHN_CHALLENGE_COLLECTION: &str = "webauthn_challenge";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub database: Database,
}

impl DB {
    pub async fn create() -> Result<Self, Error> {
        // Read the config from either the environment or a .env file.
        let mongo_uri =
            env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".to_string());
        let database_name = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "demo".to_string());
        let app_name = env::var("MONGODB_APP_NAME").unwrap_or_else(|_| "demo".to_string());

        // Create the ClientOptions and set the app_name
        let mut client_options = ClientOptions::parse(mongo_uri).await.map_err(|_| {
            Error::ServiceError("MongoDB: failed to parse client options".to_string())
        })?;
        client_options.app_name = Some(app_name);

        // Create the client and grab a database handle
        let client = Client::with_options(client_options)
            .map_err(|_| Error::ServiceError("Failed to create MongoDB client".to_string()))?;
        let database = client.database(&database_name);
        Ok(Self { client, database })
    }

    fn users(&self) -> Collection<User> {
        self.database.collection::<User>(USER_COLLECTION)
    }

    fn credentials(&self) -> Collection<Credential> {
        self.database.collection::<Credential>(CRED_COLLECTION)
    }

    fn app_config(&self) -> Collection<AppConfig> {
        self.database.collection::<AppConfig>(APP_CONFIG_COLLECTION)
    }
    fn challenges(&self) -> Collection<Challenge> {
        self.database
            .collection::<Challenge>(WEBAUTHN_CHALLENGE_COLLECTION)
    }

    pub async fn fetch_config(&self) -> Result<Option<AppConfig>, Error> {
        let result = self.app_config().find_one(None, None).await?;
        if let Some(config) = result {
            return Ok(Some(config));
        }
        Ok(None)
    }

    pub async fn put_config(&self, config: &AppConfig) -> Result<(), Error> {
        self.app_config().insert_one(config, None).await?;
        Ok(())
    }

    pub async fn check_user_by_name(&self, name: &str) -> Result<bool, Error> {
        let result = self
            .users()
            .count_documents(doc! {"name": name}, None)
            .await?;
        if result > 0 {
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn fetch_user_by_name(&self, name: &str) -> Result<Option<User>, Error> {
        self.users()
            .find_one(doc! {"name": name}, None)
            .await
            .map_err(Error::DatabaseError)
    }

    /// Fetch all user ids.
    /// Gets a cursor and loops through with `try_next()`
    pub async fn fetch_user_ids(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self.users().find(None, None).await?;
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }

    pub async fn add_user(&self, entity: &UserEntity) -> Result<(), Error> {
        let user = User::from(entity);
        let _result = self.users().insert_one(&user, None).await?;
        Ok(())
    }

    /// Caution!  Deleting a user, wihtout deleting the user's creds is bad!
    pub async fn delete_user(&self, name: &str) -> Result<(), Error> {
        self.users().delete_one(doc! {"name": name}, None).await?;
        Ok(())
    }

    pub async fn fetch_credential_by_id(&self, id: &str) -> Result<Option<Credential>, Error> {
        let cursor_result = self.credentials().find_one(doc! {"id": id}, None).await?;

        match cursor_result {
            Some(cred) => Ok(Some(cred)),
            None => Ok(None),
        }
    }

    pub async fn add_credential(&self, cred: &Credential) -> Result<InsertOneResult, Error> {
        let result = self.credentials().insert_one(cred, None).await?;
        Ok(result)
    }

    pub async fn update_credential(&self, cred: &Credential) -> Result<Credential, Error> {
        let new_cred = cred.now();
        self.credentials()
            .update_one(
                doc! {"id": cred.id.to_string()},
                doc! {"$set": {"counter": new_cred.counter, "last": new_cred.last.to_string()}},
                None,
            )
            .await?;
        Ok(new_cred)
    }

    /// Caution!  Deleting a user, wihtout deleting the user's creds is bad!
    pub async fn delete_credential(&self, id: &Base64UrlSafeData) -> Result<(), Error> {
        self.credentials()
            .delete_one(doc! {"id": id.to_string()}, None)
            .await?;
        Ok(())
    }

    /// Update a stored user with the id of a stored credential
    pub async fn add_credential_for_user(
        &self,
        name: &str,
        cred: &Credential,
    ) -> Result<(), Error> {
        self.add_credential(cred).await?;

        let id = cred.id.to_string();
        self.users()
            .update_one(
                doc! {"name": name},
                doc! {"$addToSet":{"credentials": id}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn delete_user_and_credentials(&self, name: &str) -> Result<(), Error> {
        match self.fetch_user_by_name(name).await? {
            Some(user) => {
                if let Some(ids) = user.credentials {
                    for id in ids {
                        self.delete_credential(&id).await?;
                    }
                }
                self.delete_user(name).await?;
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    //---------------------------------------------------------------
    // Challenge management.  Ensure challenges are only used once
    //----------------------------------------------------------------
    pub async fn add_challenge(&self, challenge: &Challenge) -> Result<InsertOneResult, Error> {
        Ok(self.challenges().insert_one(challenge, None).await?)
    }

    pub async fn fetch_challenge(
        &self,
        value: &Base64UrlSafeData,
    ) -> Result<Option<Challenge>, Error> {
        match self
            .challenges()
            .find_one(doc! {"value": value.to_string()}, None)
            .await?
        {
            Some(challenge) => Ok(Some(challenge)),
            None => Ok(None),
        }
    }

    pub async fn check_challenge(&self, value: &Base64UrlSafeData) -> Result<bool, Error> {
        let result = self
            .challenges()
            .count_documents(doc! {"value": value.to_string()}, None)
            .await?;
        if result > 0 {
            return Ok(true);
        }
        Ok(false)
    }

    /// the only update-able fields are the used and used_time fields
    pub async fn update_challenge(&self, challenge: &Challenge) -> Result<Challenge, Error> {
        let new = challenge.mark_used();

        self.challenges()
            .update_one(
                doc! {"value": new.value.to_string()},
                doc! {"$set": {"used": new.used, "used_time": new.used_time.unwrap().to_string()}},
                None,
            )
            .await?;
        Ok(new)
    }

    pub async fn delete_challenge(&self, value: &Base64UrlSafeData) -> Result<(), Error> {
        self.challenges()
            .delete_one(doc! {"value": value.to_string()}, None)
            .await?;
        Ok(())
    }
}
