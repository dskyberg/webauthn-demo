use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::make_id;

#[derive(Error, Debug)]
pub enum UserIdentityError {
    #[error("UserIdentityBuild error")]
    BuildError,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserIdentity {
    #[serde(with = "serde_stuff::base64")]
    pub id: Vec<u8>,
    pub displayName: String,
    pub name: String,
}

impl UserIdentity {
    pub fn new(name: &str, display_name: &str) -> Self {
        Self {
            id: make_id(32).unwrap(),
            name: name.to_owned(),
            displayName: display_name.to_owned(),
        }
    }
    pub fn builder() -> UserIdentityBuilder {
        UserIdentityBuilder::default()
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UserIdentityBuilder {
    id: Option<Vec<u8>>,
    name: Option<String>,
    displayName: Option<String>,
}

impl Default for UserIdentityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserIdentityBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            displayName: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    #[allow(non_snake_case)]
    pub fn with_displayName(mut self, displayName: &str) -> Self {
        self.displayName = Some(displayName.to_owned());
        self
    }

    pub fn with_id(mut self, id: &[u8]) -> Self {
        self.id = Some(id.to_vec());
        self
    }

    pub fn build(self) -> Result<UserIdentity> {
        if self.name.is_none() || self.displayName.is_none() {
            return Err(UserIdentityError::BuildError.into());
        }

        let id = match self.id {
            Some(id) => id,
            None => make_id(32)?,
        };

        Ok(UserIdentity {
            id,
            name: self.name.unwrap(),
            displayName: self.displayName.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let user = UserIdentity::builder()
            .with_name("Bob Smith")
            .with_displayName("bob@email.com")
            .build();
        dbg!(&user);
    }
}
