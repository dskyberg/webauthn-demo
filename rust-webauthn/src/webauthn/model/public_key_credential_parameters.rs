use serde::{Deserialize, Serialize};

use super::{COSEAlgorithm, PublicKeyCredentialType};
use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicKeyCredentialParameters {
    #[serde(rename = "type")]
    pub key_type: PublicKeyCredentialType,
    pub alg: COSEAlgorithm,
}

impl PublicKeyCredentialParameters {
    pub fn builder() -> PublicKeyCredentialParametersBuilder {
        PublicKeyCredentialParametersBuilder::default()
    }
    pub fn new() -> Self {
        Self {
            key_type: PublicKeyCredentialType::PublicKey,
            alg: COSEAlgorithm::ES256,
        }
    }
}
impl Default for PublicKeyCredentialParameters {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PublicKeyCredentialParametersBuilder {
    key_type: Option<PublicKeyCredentialType>,
    alg: Option<COSEAlgorithm>,
}

impl PublicKeyCredentialParametersBuilder {
    pub fn new() -> Self {
        Self {
            key_type: None,
            alg: None,
        }
    }

    pub fn with_key_type(&mut self, key_type: PublicKeyCredentialType) -> &mut Self {
        self.key_type = Some(key_type);
        self
    }

    pub fn with_alg(&mut self, alg: COSEAlgorithm) -> &mut Self {
        self.alg = Some(alg);
        self
    }

    pub fn build(&self) -> Result<PublicKeyCredentialParameters, Error> {
        Ok(PublicKeyCredentialParameters {
            key_type: self.key_type.clone().ok_or_else(|| {
                Error::BuildError(
                    "PublicKeyCredentialParametersBuilder: missing key_type".to_owned(),
                )
            })?,
            alg: self.alg.ok_or_else(|| {
                Error::BuildError("PublicKeyCredentialParametersBuilder: missing alg".to_owned())
            })?,
        })
    }
}

impl Default for PublicKeyCredentialParametersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_it() {
        let pkcp = PublicKeyCredentialParameters::builder()
            .with_key_type(PublicKeyCredentialType::PublicKey)
            .with_alg(COSEAlgorithm::ECDH_SS__HKDF_256)
            .build()
            .expect("oope");

        let result = serde_json::to_string(&pkcp).expect("oops");
        dbg!(&result);
        let result: PublicKeyCredentialParameters = serde_json::from_str(&result).expect("oops");
        dbg!(&result);
    }
}
