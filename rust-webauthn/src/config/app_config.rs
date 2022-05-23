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
    pub origin: String,
}

impl Default for WebauthnConfig {
    fn default() -> Self {
        let pkcco: PublicKeyCredentialCreationOptions = Default::default();
        let asc = pkcco.authenticator_selection.unwrap();
        let cred_params = pkcco.pub_key_cred_params.get(0).unwrap().to_owned();
        Self {
            rp_name: pkcco.rp.name.unwrap(),
            key_type: cred_params.key_type,
            alg: cred_params.alg,
            authenticator_attachment: asc.authenticator_attachment.unwrap(),
            resident_key: asc.resident_key.unwrap(),
            user_verification: asc.user_verification.unwrap(),
            origin: "http://localhost:3000".to_string(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    webauthn: WebauthnConfig,
}
