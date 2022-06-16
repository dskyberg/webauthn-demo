use serde::{Deserialize, Serialize};

use super::*;
use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorSelectionCriteria {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment: Option<AuthenticatorAttachment>,
    /// If no value is given then the effective value is required
    /// if requireResidentKey is true or discouraged if it is
    /// false or absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resident_key: Option<ResidentKeyRequirement>,
    /// true if, and only if, residentKey is set to required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_resident_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<UserVerificationRequirement>,
}

impl AuthenticatorSelectionCriteria {
    /// Chainable builder
    pub fn builder() -> AuthenticatorSelectionCriteriaBuilder {
        AuthenticatorSelectionCriteriaBuilder::default()
    }

    /// New instance with defaults
    pub fn new() -> Self {
        Self {
            authenticator_attachment: Some(AuthenticatorAttachment::default()),
            resident_key: Some(ResidentKeyRequirement::default()),
            require_resident_key: None,
            user_verification: Some(UserVerificationRequirement::default()),
        }
    }
}

impl Default for AuthenticatorSelectionCriteria {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AuthenticatorSelectionCriteriaBuilder {
    authenticator_attachment: Option<AuthenticatorAttachment>,
    resident_key: Option<ResidentKeyRequirement>,
    require_resident_key: Option<bool>,
    user_verification: Option<UserVerificationRequirement>,
}

impl AuthenticatorSelectionCriteriaBuilder {
    pub fn new() -> Self {
        Self {
            authenticator_attachment: None,
            resident_key: None,
            require_resident_key: None,
            user_verification: None,
        }
    }
    pub fn with_authenticator_attachment(
        &mut self,
        authenticator_attachment: AuthenticatorAttachment,
    ) -> &mut Self {
        self.authenticator_attachment = Some(authenticator_attachment);
        self
    }

    pub fn with_resident_key(&mut self, resident_key: ResidentKeyRequirement) -> &mut Self {
        self.resident_key = Some(resident_key);
        self
    }

    pub fn with_user_verification(
        &mut self,
        user_verification: UserVerificationRequirement,
    ) -> &mut Self {
        self.user_verification = Some(user_verification);
        self
    }

    /// Note: if require_resident_key is Some(true) then
    /// resident_key must either be None or
    pub fn build(&self) -> Result<AuthenticatorSelectionCriteria, Error> {
        Ok(AuthenticatorSelectionCriteria {
            authenticator_attachment: self.authenticator_attachment.clone(),
            resident_key: self.resident_key.clone(),
            require_resident_key: self.require_resident_key,
            user_verification: self.user_verification.clone(),
        })
    }
}

impl Default for AuthenticatorSelectionCriteriaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]

    fn test_it() -> Result<(), crate::errors::Error> {
        let asc = AuthenticatorSelectionCriteria::builder()
            .with_authenticator_attachment(AuthenticatorAttachment::Platform)
            .with_resident_key(ResidentKeyRequirement::Discouraged)
            .with_user_verification(UserVerificationRequirement::Preferred)
            .build()?;
        dbg!(&asc);
        let result = serde_json::to_string(&asc).map_err(Error::SerdeJsonError)?;

        dbg!(&result);
        Ok(())
    }
}
