use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebauthnPolicy {
    pub rp_id: String,
    pub rp_name: String,
    pub key_type: PublicKeyCredentialType,
    pub alg: COSEAlgorithm,
    pub authenticator_attachment: AuthenticatorAttachment,
    pub resident_key: ResidentKeyRequirement,
    pub user_verification: UserVerificationRequirement,
    pub origin: String,
    pub attestation: AttestationConveyancePreference,
    pub timeout: usize,
    pub default_user_display_name: String,
    pub default_user_name: String,
}

impl Default for WebauthnPolicy {
    fn default() -> Self {
        Self {
            rp_id: "localhost".to_owned(),
            rp_name: "swankymutt".to_owned(),
            key_type: PublicKeyCredentialType::PublicKey,
            alg: COSEAlgorithm::ES256,
            authenticator_attachment: AuthenticatorAttachment::Platform,
            resident_key: ResidentKeyRequirement::Discouraged,
            user_verification: UserVerificationRequirement::Required,
            origin: "http://localhost:3000".to_string(),
            attestation: AttestationConveyancePreference::Direct,
            timeout: 360000,
            default_user_display_name: "Faky McFakerson".to_owned(),
            default_user_name: "faky.mcfakerson@mail.do".to_owned(),
        }
    }
}
