use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::utils::make_id;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    #[serde(default, with = "serde_stuff::option_base64")]
    pub id: Option<Vec<u8>>,
    pub name: String,
    pub display_name: String,
}

impl UserEntity {
    pub fn builder() -> UserEntityBuilder {
        UserEntityBuilder::default()
    }

    pub fn new(name: &str, display_name: &str) -> UserEntity {
        UserEntity {
            id: None,
            name: name.to_owned(),
            display_name: display_name.to_owned(),
        }
    }
}
impl Default for UserEntity {
    fn default() -> Self {
        Self {
            display_name: "Faky McFakerson".to_owned(),
            name: "faky.mcfakerson@mail.do".to_owned(),
            id: Some(make_id(32).unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct UserEntityBuilder {
    id: Option<Vec<u8>>,
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
    pub fn with_display_name(&mut self, displayName: &str) -> &mut Self {
        self.display_name = Some(displayName.to_owned());
        self
    }

    pub fn with_id(&mut self, id: &[u8]) -> &mut Self {
        self.id = Some(id.to_vec());
        self
    }

    pub fn build(&self) -> Result<UserEntity> {
        if self.name.is_none() || self.display_name.is_none() {
            return Err(Error::UserEntityBuildError.into());
        }

        let id = match &self.id {
            Some(id) => Some(id.to_owned()),
            None => Some(make_id(32)?),
        };

        Ok(UserEntity {
            id,
            name: self.name.as_ref().unwrap().to_owned(),
            display_name: self.display_name.as_ref().unwrap().to_owned(),
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
            .with_display_name("bob@email.com")
            .build();
        dbg!(&user);
    }
}
