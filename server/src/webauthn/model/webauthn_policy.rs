use serde::{Deserialize, Serialize};
use url::Url;

use super::*;
use crate::errors::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub default_user_display_name: String,
    pub default_user_name: String,
    pub validate_sign_count: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_transports: Option<Vec<AuthenticatorTransport>>,
}

impl WebauthnPolicy {
    pub fn new() -> Self {
        let origin = Url::try_from("http://localhost:3000").expect("Bad URL");
        Self {
            rp_id: "localhost".to_owned(),
            rp_name: "swankymutt".to_owned(),
            key_type: PublicKeyCredentialType::PublicKey,
            alg: COSEAlgorithm::ES256,
            authenticator_attachment: AuthenticatorAttachment::Platform,
            resident_key: ResidentKeyRequirement::Discouraged,
            user_verification: UserVerificationRequirement::Required,
            origin,
            attestation: AttestationConveyancePreference::Direct,
            timeout: 360000,
            default_user_display_name: "Faky McFakerson".to_owned(),
            default_user_name: "faky.mcfakerson@mail.do".to_owned(),
            validate_sign_count: false,
            authenticator_transports: None,
        }
    }

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
        if builder.default_user_display_name.is_some() {
            self.default_user_display_name = builder.default_user_display_name.unwrap();
        }
        if builder.default_user_name.is_some() {
            self.default_user_name = builder.default_user_name.unwrap();
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

impl Default for WebauthnPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub default_user_display_name: Option<String>,
    pub default_user_name: Option<String>,
    pub validate_sign_count: Option<bool>,
    pub authenticator_transports: Option<Vec<AuthenticatorTransport>>,
}

impl WebauthnPolicyBuilder {
    pub fn set_rp_id(&mut self, rp_id: String) -> Result<(), Error> {
        self.rp_id = Some(rp_id);
        Ok(())
    }

    pub fn set_rp_name(&mut self, rp_name: String) -> Result<(), Error> {
        self.rp_name = Some(rp_name);
        Ok(())
    }

    pub fn set_key_type(&mut self, key_type: PublicKeyCredentialType) -> Result<(), Error> {
        self.key_type = Some(key_type);
        Ok(())
    }

    pub fn set_alg(&mut self, alg: COSEAlgorithm) -> Result<(), Error> {
        self.alg = Some(alg);
        Ok(())
    }

    pub fn set_authenticator_attachment(
        &mut self,
        authenticator_attachment: AuthenticatorAttachment,
    ) -> Result<(), Error> {
        self.authenticator_attachment = Some(authenticator_attachment);
        Ok(())
    }

    pub fn set_resident_key(&mut self, resident_key: ResidentKeyRequirement) -> Result<(), Error> {
        self.resident_key = Some(resident_key);
        Ok(())
    }

    pub fn set_user_verification(
        &mut self,
        user_verification: UserVerificationRequirement,
    ) -> Result<(), Error> {
        self.user_verification = Some(user_verification);
        Ok(())
    }

    pub fn set_origin(&mut self, origin: Url) -> Result<(), Error> {
        self.origin = Some(origin);
        Ok(())
    }

    pub fn set_attestation(
        &mut self,
        attestation: AttestationConveyancePreference,
    ) -> Result<(), Error> {
        self.attestation = Some(attestation);
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: usize) -> Result<(), Error> {
        self.timeout = Some(timeout);
        Ok(())
    }

    pub fn set_default_user_display_name(
        &mut self,
        default_user_display_name: String,
    ) -> Result<(), Error> {
        self.default_user_display_name = Some(default_user_display_name);
        Ok(())
    }

    pub fn set_default_user_name(&mut self, default_user_name: String) -> Result<(), Error> {
        self.default_user_name = Some(default_user_name);
        Ok(())
    }

    pub fn set_validate_sign_count(&mut self, validate_sign_count: bool) -> Result<(), Error> {
        self.validate_sign_count = Some(validate_sign_count);
        Ok(())
    }
    pub fn set_authenticator_transports(
        &mut self,
        authenticator_transports: Vec<AuthenticatorTransport>,
    ) -> Result<(), Error> {
        self.authenticator_transports = Some(authenticator_transports);
        Ok(())
    }
}
