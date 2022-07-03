use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::utils::make_id;

use super::WebauthnPolicy;
/*
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserName {
    pub name: String,
}
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub id: Option<Base64UrlSafeData>,
    pub name: String,
    pub display_name: Option<String>,
}

impl UserEntity {
    pub fn builder() -> UserEntityBuilder {
        UserEntityBuilder::default()
    }
}

impl From<&WebauthnPolicy> for UserEntity {
    fn from(policy: &WebauthnPolicy) -> Self {
        Self {
            id: None,
            name: policy.default_user_name.clone(),
            display_name: Some(policy.default_user_display_name.clone()),
        }
    }
}

#[derive(Debug)]
pub struct UserEntityBuilder {
    id: Option<Base64UrlSafeData>,
    name: Option<String>,
    display_name: Option<String>,
}

impl Default for UserEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserEntityBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            display_name: None,
        }
    }

    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    #[allow(non_snake_case)]
    pub fn with_display_name(&mut self, displayName: &Option<String>) -> &mut Self {
        self.display_name = displayName.clone();
        self
    }

    pub fn with_id(&mut self, id: &[u8]) -> &mut Self {
        self.id = Some(Base64UrlSafeData(id.to_vec()));
        self
    }

    pub fn build(&self) -> Result<UserEntity, Error> {
        if self.name.is_none() || self.display_name.is_none() {
            return Err(Error::UserEntityBuildError);
        }

        let id = match &self.id {
            Some(id) => Some(id.to_owned()),
            None => Some(Base64UrlSafeData(make_id(32)?)),
        };

        Ok(UserEntity {
            id,
            name: self.name.as_ref().unwrap().to_owned(),
            display_name: self.display_name.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let user = UserEntity::builder()
            .with_name("Bob Smith")
            .with_display_name(&Some("bob@email.com".to_owned()))
            .build();
        dbg!(&user);
    }
}
