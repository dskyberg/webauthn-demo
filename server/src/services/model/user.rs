/// This model is just for the DB, so that we can relate credential
/// IDs with the user.
use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::webauthn::model::UserEntity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<Base64UrlSafeData>,
    pub name: String,
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<Base64UrlSafeData>>,
}

impl User {
    pub fn as_user_entity(&self) -> UserEntity {
        UserEntity {
            id: self.id.clone(),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

impl From<&UserEntity> for User {
    fn from(user: &UserEntity) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name.clone(),
            display_name: user.display_name.clone(),
            credentials: None,
        }
    }
}
