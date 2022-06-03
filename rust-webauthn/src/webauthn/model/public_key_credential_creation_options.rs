use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::*;
use crate::{errors::Error, utils::make_id};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    pub rp: RpEntity,
    pub user: UserEntity,
    #[serde(with = "serde_stuff::base64")]
    pub challenge: Vec<u8>,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation: Option<AttestationConveyancePreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_selection: Option<AuthenticatorSelectionCriteria>,
}

impl PublicKeyCredentialCreationOptions {
    pub fn builder() -> PublicKeyCredentialCreationOptionsBuilder {
        PublicKeyCredentialCreationOptionsBuilder::default()
    }

    pub fn new() -> Self {
        Self {
            rp: RpEntity::default(),
            user: UserEntity::default(),
            challenge: make_id(32).unwrap(),
            pub_key_cred_params: vec![PublicKeyCredentialParameters::default()],
            timeout: None,
            attestation: None, // Some(AttestationConveyancePreference::default()),
            authenticator_selection: Some(AuthenticatorSelectionCriteria::default()),
        }
    }

    /// Generate default options, using the provided [UserEntity].
    pub fn from_user_entity(user: &UserEntity) -> Result<Self> {
        let authenticator_selection: AuthenticatorSelectionCriteria = Default::default();

        // Build a user, with an ID, from the provided user.
        let user = UserEntity::builder()
            .with_name(&user.name)
            .with_display_name(&user.display_name)
            .build()?;

        let rp: RpEntity = Default::default();
        let attestation: AttestationConveyancePreference = Default::default();
        let pub_key_cred_params: PublicKeyCredentialParameters = Default::default();

        let options = PublicKeyCredentialCreationOptions::builder()
            .with_user(user)
            .with_rp(rp)
            .with_attestation(attestation)
            .with_authenticator_selection(authenticator_selection)
            .with_pub_key_cred_params(pub_key_cred_params)
            .with_timeout(360000)
            .build()?;

        Ok(options)
    }
}

impl Default for PublicKeyCredentialCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PublicKeyCredentialCreationOptionsBuilder {
    rp: Option<RpEntity>,
    user: Option<UserEntity>,
    challenge: Option<Vec<u8>>,
    timeout: Option<usize>,
    pub_key_cred_params: Option<Vec<PublicKeyCredentialParameters>>,
    attestation: Option<AttestationConveyancePreference>,
    authenticator_selection: Option<AuthenticatorSelectionCriteria>,
}

impl Default for PublicKeyCredentialCreationOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicKeyCredentialCreationOptionsBuilder {
    pub fn new() -> Self {
        Self {
            rp: None,
            user: None,
            challenge: None,
            pub_key_cred_params: None,
            timeout: None,
            attestation: None,
            authenticator_selection: None,
        }
    }

    pub fn with_user(&mut self, user: UserEntity) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn with_rp(&mut self, rp: RpEntity) -> &mut Self {
        self.rp = Some(rp);
        self
    }

    pub fn with_attestation(&mut self, attestation: AttestationConveyancePreference) -> &mut Self {
        self.attestation = Some(attestation);
        self
    }

    pub fn with_authenticator_selection(
        &mut self,
        asc: AuthenticatorSelectionCriteria,
    ) -> &mut Self {
        self.authenticator_selection = Some(asc);
        self
    }

    pub fn with_pub_key_cred_params(&mut self, params: PublicKeyCredentialParameters) -> &mut Self {
        self.pub_key_cred_params = Some(vec![params]);
        self
    }

    pub fn with_timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(&self) -> Result<PublicKeyCredentialCreationOptions> {
        if self.user.is_none() {
            return Err(Error::RegistrationChallengResponseBuildError.into());
        }

        let challenge = match &self.challenge {
            Some(challenge) => challenge.clone(),
            None => make_id(32)?,
        };

        Ok(PublicKeyCredentialCreationOptions {
            user: self.user.as_ref().unwrap().clone(),
            rp: self.rp.as_ref().unwrap().clone(),
            challenge,
            pub_key_cred_params: self.pub_key_cred_params.as_ref().unwrap().clone(),
            attestation: self.attestation.clone(),
            authenticator_selection: self.authenticator_selection.clone(),
            timeout: self.timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json;

    #[test]
    fn test_it() -> Result<()> {
        let challenge = PublicKeyCredentialCreationOptions::builder()
            .with_user(
                UserEntity::builder()
                    .with_display_name("Bob Smith")
                    .with_name("bob@email.com")
                    .build()?,
            )
            .with_rp(RpEntity::new("Swankymutt"))
            .with_attestation(AttestationConveyancePreference::Direct)
            .with_pub_key_cred_params(PublicKeyCredentialParameters::default())
            .with_timeout(360000)
            .with_authenticator_selection(AuthenticatorSelectionCriteria::default())
            .build()?;
        dbg!(&challenge);
        let result = serde_json::to_string(&challenge).expect("Oops");
        dbg!(&result);
        Ok(())
    }

    #[test]
    fn test_defaults() -> Result<()> {
        let mut challenge: PublicKeyCredentialCreationOptions = Default::default();
        challenge.user.id = Some(make_id(32).unwrap());
        dbg!(&challenge);
        Ok(())
    }
}
