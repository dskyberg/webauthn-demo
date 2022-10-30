use serde::{Deserialize, Serialize};
use url::Url;

use super::*;
use crate::errors::Error;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebauthnPolicy {
    pub rp_id: String,
    pub rp_name: String,
    pub key_type: PublicKeyCredentialType,
    pub alg: COSEAlgorithm,
    pub authenticator_attachment: AuthenticatorAttachment,
    pub resident_key: ResidentKeyRequirement,
    pub user_verification: UserVerificationRequirement,
    pub origin: Url,
    pub attestation: AttestationConveyancePreference,
    pub timeout: usize,
    pub validate_sign_count: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_transports: Option<Vec<AuthenticatorTransport>>,
}

impl WebauthnPolicy {
    pub fn update(&mut self, builder: WebauthnPolicyBuilder) -> Result<(), Error> {
        if builder.rp_id.is_some() {
            self.rp_id = builder.rp_id.unwrap();
        }
        if builder.rp_name.is_some() {
            self.rp_name = builder.rp_name.unwrap();
        }
        if builder.key_type.is_some() {
            self.key_type = builder.key_type.unwrap();
        }
        if builder.alg.is_some() {
            self.alg = builder.alg.unwrap();
        }
        if builder.authenticator_attachment.is_some() {
            self.authenticator_attachment = builder.authenticator_attachment.unwrap()
        }
        if builder.resident_key.is_some() {
            self.resident_key = builder.resident_key.unwrap();
        }
        if builder.user_verification.is_some() {
            self.user_verification = builder.user_verification.unwrap();
        }
        if builder.origin.is_some() {
            self.origin = builder.origin.unwrap();
        }
        if builder.attestation.is_some() {
            self.attestation = builder.attestation.unwrap();
        }
        if builder.timeout.is_some() {
            self.timeout = builder.timeout.unwrap();
        }
        if builder.validate_sign_count.is_some() {
            self.validate_sign_count = builder.validate_sign_count.unwrap();
        }
        if builder.authenticator_transports.is_some() {
            self.authenticator_transports = builder.authenticator_transports
        }
        Ok(())
    }
}

/// WebauthnPolicyBuilder represents a modification of the builder
/// pattern that ensures every element of WebauthnPolicy is explicitly
/// defined before building.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebauthnPolicyBuilder {
    pub rp_id: Option<String>,
    pub rp_name: Option<String>,
    pub key_type: Option<PublicKeyCredentialType>,
    pub alg: Option<COSEAlgorithm>,
    pub authenticator_attachment: Option<AuthenticatorAttachment>,
    pub resident_key: Option<ResidentKeyRequirement>,
    pub user_verification: Option<UserVerificationRequirement>,
    pub origin: Option<Url>,
    pub attestation: Option<AttestationConveyancePreference>,
    pub timeout: Option<usize>,
    pub validate_sign_count: Option<bool>,
    pub authenticator_transports: Option<Vec<AuthenticatorTransport>>,
}

impl Default for WebauthnPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WebauthnPolicyBuilder {
    pub fn new() -> Self {
        Self {
            rp_id: None,
            rp_name: None,
            key_type: None,
            alg: None,
            authenticator_attachment: None,
            resident_key: None,
            user_verification: None,
            origin: None,
            attestation: None,
            timeout: None,
            validate_sign_count: None,
            authenticator_transports: None,
        }
    }

    pub fn build(self) -> Result<WebauthnPolicy, Error> {
        if self.rp_id.is_none() {
            return Err(Error::EmptyWebauthnPolicy("rp_id".to_string()));
        }
        if self.rp_name.is_none() {
            return Err(Error::EmptyWebauthnPolicy("rp_name".to_string()));
        }
        if self.key_type.is_none() {
            return Err(Error::EmptyWebauthnPolicy("key_type".to_string()));
        }
        if self.alg.is_none() {
            return Err(Error::EmptyWebauthnPolicy("alg".to_string()));
        }
        if self.authenticator_attachment.is_none() {
            return Err(Error::EmptyWebauthnPolicy(
                "authenticator_attachment".to_string(),
            ));
        }
        if self.resident_key.is_none() {
            return Err(Error::EmptyWebauthnPolicy("resident_key".to_string()));
        }
        if self.user_verification.is_none() {
            return Err(Error::EmptyWebauthnPolicy("user_verification".to_string()));
        }
        if self.origin.is_none() {
            return Err(Error::EmptyWebauthnPolicy("origin".to_string()));
        }
        if self.attestation.is_none() {
            return Err(Error::EmptyWebauthnPolicy("attestation".to_string()));
        }
        if self.timeout.is_none() {
            return Err(Error::EmptyWebauthnPolicy("timeout".to_string()));
        }
        if self.validate_sign_count.is_none() {
            return Err(Error::EmptyWebauthnPolicy(
                "validate_sign_count".to_string(),
            ));
        }
        if self.authenticator_transports.is_none() {
            return Err(Error::EmptyWebauthnPolicy(
                "authenticator_transports".to_string(),
            ));
        }

        Ok(WebauthnPolicy {
            rp_id: self.rp_id.unwrap(),
            rp_name: self.rp_name.unwrap(),
            key_type: self.key_type.unwrap(),
            alg: self.alg.unwrap(),
            authenticator_attachment: self.authenticator_attachment.unwrap(),
            resident_key: self.resident_key.unwrap(),
            user_verification: self.user_verification.unwrap(),
            origin: self.origin.unwrap(),
            attestation: self.attestation.unwrap(),
            timeout: self.timeout.unwrap(),
            validate_sign_count: self.validate_sign_count.unwrap(),
            authenticator_transports: self.authenticator_transports,
        })
    }

    pub fn with_rp_id(mut self, rp_id: String) -> Self {
        self.rp_id = Some(rp_id);
        self
    }

    pub fn with_rp_name(mut self, rp_name: String) -> Self {
        self.rp_name = Some(rp_name);
        self
    }

    pub fn with_key_type(mut self, key_type: PublicKeyCredentialType) -> Self {
        self.key_type = Some(key_type);
        self
    }

    pub fn with_alg(mut self, alg: COSEAlgorithm) -> Self {
        self.alg = Some(alg);
        self
    }

    pub fn with_authenticator_attachment(
        mut self,
        authenticator_attachment: AuthenticatorAttachment,
    ) -> Self {
        self.authenticator_attachment = Some(authenticator_attachment);
        self
    }

    pub fn with_resident_key(mut self, resident_key: ResidentKeyRequirement) -> Self {
        self.resident_key = Some(resident_key);
        self
    }

    pub fn with_user_verification(
        mut self,
        user_verification: UserVerificationRequirement,
    ) -> Self {
        self.user_verification = Some(user_verification);
        self
    }

    pub fn with_origin(mut self, origin: Url) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn with_attestation(mut self, attestation: AttestationConveyancePreference) -> Self {
        self.attestation = Some(attestation);
        self
    }

    pub fn with_timeout(mut self, timeout: usize) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_validate_sign_count(mut self, validate_sign_count: bool) -> Self {
        self.validate_sign_count = Some(validate_sign_count);
        self
    }
    pub fn with_authenticator_transports(
        mut self,
        authenticator_transports: Option<Vec<AuthenticatorTransport>>,
    ) -> Self {
        self.authenticator_transports = authenticator_transports;
        self
    }
}
