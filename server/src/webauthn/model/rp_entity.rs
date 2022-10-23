use serde::{Deserialize, Serialize};

use super::*;
use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct RpEntity {
    /// The RP ID can only be the "effective domain" of the RP.
    /// The effective domain is the domain without scheme or port.
    /// Ie, the effective domain of `http://localhost:3000` is `localhost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RpEntity {
    pub fn builder() -> RpEntityBuilder {
        RpEntityBuilder::default()
    }
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: Some(name.to_owned()),
        }
    }
}

impl From<&WebauthnPolicy> for RpEntity {
    fn from(policy: &WebauthnPolicy) -> Self {
        Self::new(policy.rp_name.as_ref())
    }
}

pub struct RpEntityBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl RpEntityBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
        }
    }

    pub fn with_id(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_owned());
        self
    }

    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn build(&self) -> Result<RpEntity, Error> {
        if self.name.is_none() && self.id.is_none() {
            return Err(Error::MissingIdAndName);
        }

        Ok(RpEntity {
            id: self.id.clone(),
            name: self.name.clone(),
        })
    }
}

impl Default for RpEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;

    #[test]
    fn test_id() -> Result<(), Error> {
        let rp = RpEntity::builder().with_name("Swankymutt").build()?;
        dbg!(&rp);

        Ok(())
    }
}
