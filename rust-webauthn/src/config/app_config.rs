use serde::{Deserialize, Serialize};

use crate::webauthn::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnConfig {
    pub rp_name: String,
    pub key_type: PublicKeyCredentialType,
    pub alg: COSEAlgorithm,
    pub authenticator_attachment: AuthenticatorAttachment,
    pub resident_key: ResidentKeyRequirement,
    pub user_verification: UserVerificationRequirement,
}

impl Default for WebauthnConfig {
    fn default() -> Self {
        let pkcco: PublicKeyCredentialCreationOptions = Default::default();
        let asc = pkcco.authenticator_selection.unwrap();
        Self {
            rp_name: pkcco.rp.name.unwrap(),
            key_type: pkcco.pub_key_cred_params.key_type,
            alg: pkcco.pub_key_cred_params.alg,
            authenticator_attachment: asc.authenticator_attachment.unwrap(),
            resident_key: asc.resident_key.unwrap(),
            user_verification: asc.user_verification.unwrap(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    webauthn: WebauthnConfig,
}
