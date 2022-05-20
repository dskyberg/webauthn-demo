use crate::errors::Error;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct RpEntity {
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
impl Default for RpEntity {
    fn default() -> Self {
        Self::new("Swankymutt")
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

    pub fn build(&self) -> Result<RpEntity> {
        if self.name.is_none() && self.id.is_none() {
            bail!(Error::MissingIdAndName);
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
    use anyhow::Result;
    //use serde_json;
    use super::*;

    #[test]
    fn test_id() -> Result<()> {
        let rp = RpEntity::builder().with_name("Swankymutt").build()?;
        dbg!(&rp);

        Ok(())
    }
}
