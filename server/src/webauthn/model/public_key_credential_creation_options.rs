use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use super::*;
use crate::{errors::Error, utils::make_id};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    pub rp: RpEntity,
    pub user: UserEntity,
    pub challenge: Base64UrlSafeData,
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
}
/*
impl From<&WebauthnPolicy> for PublicKeyCredentialCreationOptions {
    fn from(policy: &WebauthnPolicy) -> Self {
        Self {
            rp: RpEntity::from(policy),
            user: UserEntity::from(policy),
            challenge: Base64UrlSafeData(make_id(32).unwrap()),
            pub_key_cred_params: vec![PublicKeyCredentialParameters::from(policy)],
            timeout: Some(policy.timeout),
            attestation: Some(policy.attestation.clone()),
            authenticator_selection: Some(AuthenticatorSelectionCriteria::from(policy)),
        }
    }
}
*/

/// Generate options from policy
/// Leveages [PublicKeyCredentialCreationOptionsBuilder]
impl TryFrom<(&WebauthnPolicy, &UserEntity, &Base64UrlSafeData)>
    for PublicKeyCredentialCreationOptions
{
    type Error = Error;

    /// Generate default options, using the provided [UserEntity].
    fn try_from(
        input: (&WebauthnPolicy, &UserEntity, &Base64UrlSafeData),
    ) -> Result<Self, Self::Error> {
        let policy = input.0;
        let user = input.1;
        let challenge = input.2;

        let user = UserEntity::builder()
            .with_name(&user.name)
            .with_display_name(&user.display_name)
            .build()?;

        let options = PublicKeyCredentialCreationOptionsBuilder::from(policy)
            .with_challenge(challenge)
            .with_user(user)
            .build()?;
        Ok(options)
    }
}

pub struct PublicKeyCredentialCreationOptionsBuilder {
    rp: Option<RpEntity>,
    user: Option<UserEntity>,
    challenge: Option<Base64UrlSafeData>,
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

/// Build the options builder from WebAuthnPolicy.
impl From<&WebauthnPolicy> for PublicKeyCredentialCreationOptionsBuilder {
    fn from(policy: &WebauthnPolicy) -> Self {
        let rp = RpEntity::from(policy);
        let pub_key_cred_params = PublicKeyCredentialParameters::from(policy);
        let attestation = policy.attestation.clone();
        let timeout = policy.timeout;
        let authenticator_selection = AuthenticatorSelectionCriteria::from(policy);

        PublicKeyCredentialCreationOptionsBuilder::default()
            .with_attestation(attestation)
            .with_timeout(timeout)
            .with_rp(rp)
            .with_pub_key_cred_params(pub_key_cred_params)
            .with_authenticator_selection(authenticator_selection)
    }
}

/// Chainable builder pattern
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

    pub fn with_challenge(mut self, challenge: &Base64UrlSafeData) -> Self {
        self.challenge = Some(challenge.clone());
        self
    }
    pub fn with_user(mut self, user: UserEntity) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_rp(mut self, rp: RpEntity) -> Self {
        self.rp = Some(rp);
        self
    }

    pub fn with_attestation(mut self, attestation: AttestationConveyancePreference) -> Self {
        self.attestation = Some(attestation);
        self
    }

    pub fn with_authenticator_selection(mut self, asc: AuthenticatorSelectionCriteria) -> Self {
        self.authenticator_selection = Some(asc);
        self
    }

    pub fn with_pub_key_cred_params(mut self, params: PublicKeyCredentialParameters) -> Self {
        self.pub_key_cred_params = Some(vec![params]);
        self
    }

    pub fn with_timeout(mut self, timeout: usize) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(&self) -> Result<PublicKeyCredentialCreationOptions, Error> {
        if self.user.is_none() {
            return Err(Error::RegistrationChallengResponseBuildError);
        }

        let challenge = match &self.challenge {
            Some(challenge) => challenge.clone(),
            None => Base64UrlSafeData(make_id(32)?),
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
