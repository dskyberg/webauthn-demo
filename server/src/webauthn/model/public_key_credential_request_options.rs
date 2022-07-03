use base64urlsafedata::Base64UrlSafeData;
use serde::{Deserialize, Serialize};

use super::*;
use crate::errors::Error;
use crate::utils::make_id;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRequestOptions {
    pub challenge: Base64UrlSafeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<Vec<PublicKeyCredentialDiscriptor>>,
    pub user_verification: UserVerificationRequirement,
}

impl PublicKeyCredentialRequestOptions {
    pub fn builder() -> PublicKeyCredentialRequestOptionsBuilder {
        PublicKeyCredentialRequestOptionsBuilder::default()
    }
}

impl TryFrom<(&WebauthnPolicy, &Credential)> for PublicKeyCredentialRequestOptions {
    type Error = Error;
    fn try_from(input: (&WebauthnPolicy, &Credential)) -> Result<Self, Self::Error> {
        let policy = input.0;
        let credential = input.1;
        let allow_credentials = vec![PublicKeyCredentialDiscriptor::try_from(credential)?];
        PublicKeyCredentialRequestOptions::builder()
            .with_allow_credentials(allow_credentials)
            .with_user_verification(policy.user_verification.clone())
            .build()
    }
}

pub struct PublicKeyCredentialRequestOptionsBuilder {
    challenge: Option<Base64UrlSafeData>,
    timeout: Option<usize>,
    rp_id: Option<String>,
    allow_credentials: Option<Vec<PublicKeyCredentialDiscriptor>>,
    user_verification: Option<UserVerificationRequirement>,
}

impl Default for PublicKeyCredentialRequestOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicKeyCredentialRequestOptionsBuilder {
    pub fn new() -> Self {
        Self {
            challenge: None,
            timeout: None,
            rp_id: None,
            allow_credentials: None,
            user_verification: None,
        }
    }

    pub fn with_challenge(&mut self, challenge: &Base64UrlSafeData) -> &mut Self {
        self.challenge = Some(challenge.clone());
        self
    }

    pub fn with_timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_allow_credentials(
        &mut self,
        allow_credentials: Vec<PublicKeyCredentialDiscriptor>,
    ) -> &mut Self {
        self.allow_credentials = Some(allow_credentials);
        self
    }

    pub fn with_user_verification(
        &mut self,
        user_verification: UserVerificationRequirement,
    ) -> &mut Self {
        self.user_verification = Some(user_verification);
        self
    }

    pub fn build(&self) -> Result<PublicKeyCredentialRequestOptions, Error> {
        let challenge = self
            .challenge
            .clone()
            .unwrap_or_else(|| Base64UrlSafeData(make_id(32).unwrap()));
        let user_verification = self
            .user_verification
            .clone()
            .unwrap_or(UserVerificationRequirement::Discouraged);
        Ok(PublicKeyCredentialRequestOptions {
            challenge,
            timeout: self.timeout,
            rp_id: self.rp_id.clone(),
            allow_credentials: self.allow_credentials.clone(),
            user_verification,
        })
    }
}
