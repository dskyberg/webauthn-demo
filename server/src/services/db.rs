//! Wrapper for MongoDB connections.
//!
use base64urlsafedata::Base64UrlSafeData;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, options::ClientOptions, results::InsertOneResult, Client, Collection, Database,
};
use std::env;

use super::User;
use crate::{
    config::AppConfig,
    errors::Error,
    webauthn::model::{Credential, UserEntity},
};

const CRED_COLLECTION: &str = "credentials";
const USER_COLLECTION: &str = "users";
const APP_CONFIG_COLLECTION: &str = "appconfig";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub database: Database,
}

impl DB {
    pub async fn new() -> Self {
        // Read the config from either the environment or a .env file.
        let mongo_uri =
            env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".to_string());
        let database_name = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "demo".to_string());
        let app_name = env::var("MONGODB_APP_NAME").unwrap_or_else(|_| "demo".to_string());

        // Create the ClientOptions and set the app_name
        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        // Create the client and grab a database handle
        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
        let database = client.database(&database_name);
        Self { client, database }
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

    pub async fn fetch_config(&self) -> Result<Option<AppConfig>, Error> {
        let cursor = self.app_config().find_one(None, None).await?;
        if let Some(config) = cursor {
            return Ok(Some(config));
        }
        Ok(None)
    }

    pub async fn put_config(&self, config: &AppConfig) -> Result<(), Error> {
        let _result = self.app_config().insert_one(config, None).await?;
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
        log::info!("Deleting user: {}", name);
        let result = self.users().delete_one(doc! {"name": name}, None).await?;
        log::info!("{:?}", &result);
        Ok(())
    }

    pub async fn fetch_credential_by_id(&self, id: &str) -> Result<Option<Credential>, Error> {
        let cursor_result = self
            .credentials()
            .find_one(doc! {"id": id}, None)
            .await
            .map_err(Error::DatabaseError);

        match cursor_result {
            Ok(cursor) => match cursor {
                Some(cred) => Ok(Some(cred)),
                None => Ok(None),
            },
            Err(e) => Err(e),
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
        log::info!("Deleting credential: {}", &id.to_string());
        let result = self
            .credentials()
            .delete_one(doc! {"id": id.to_string()}, None)
            .await?;
        log::info!("{:?}", &result);
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
        log::info!("Deleting {}", name);
        let result = self.fetch_user_by_name(name).await?;
        if result.is_none() {
            log::info!("Deleting {} - user not found", name);
            return Err(Error::NotFound);
        }
        let user = result.unwrap();

        // Delete each credential
        if let Some(ids) = user.credentials {
            for id in ids {
                self.delete_credential(&id).await?;
            }
        }
        self.delete_user(name).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
