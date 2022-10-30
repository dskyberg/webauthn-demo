/// Manage the application config.  This includes default policy for
/// WebAuthn.
///
/// The method `default_webauthn_policy` will attempt to read defaults from
/// the env.  See [example.env](../../example.env) to see how to format the
/// env values.  See the individual `env_` methods here for guidance on how
/// to set yur defaults without using the env.  Each `env_` method returns a
/// default value if there is no env value specified.
///
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

use crate::webauthn::model::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub webauthn: WebauthnPolicy,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::new()
    }
}

impl AppConfig {
    pub fn new() -> AppConfig {
        Self {
            webauthn: default_webauthn_policy(),
        }
        // This is where the default WebauthnPolicy gets created
    }
}

fn env_transports() -> Option<Vec<AuthenticatorTransport>> {
    if let Ok(result) = env::var("WEBAUTHN_AUTHENTICATOR_TRANSPORTS") {
        let transports: Vec<AuthenticatorTransport> = serde_json::from_str(&result)
            .expect("Failed parsing WEBAUTHN_AUTHENTICATOR_TRANSPORTS from env");
        Some(transports)
    } else {
        None
    }
}

fn env_alg() -> COSEAlgorithm {
    if let Ok(result) = env::var("WEBAUTHN_ALG") {
        let alg: COSEAlgorithm =
            serde_json::from_str(&result).expect("Failed to parse WEBAUTHN_ALG from env");
        alg
    } else {
        COSEAlgorithm::ES256
    }
}

fn env_authenticator_attachment() -> AuthenticatorAttachment {
    if let Ok(result) = env::var("WEBAUTHN_AUTHENTICATOR_ATTACHMENT") {
        let attachment: AuthenticatorAttachment = serde_json::from_str(&result)
            .expect("Failed to parse WEBAUTHN_AUTHENTICATOR_ATTACHMENT from env");
        attachment
    } else {
        AuthenticatorAttachment::MultiPlatform
    }
}

fn env_resident_key() -> ResidentKeyRequirement {
    if let Ok(result) = env::var("WEBAUTHN_RESIDENT_KEY") {
        let resident_key: ResidentKeyRequirement =
            serde_json::from_str(&result).expect("Failed to parse WEBAUTHN_RESIDENT_KEY from env");
        resident_key
    } else {
        ResidentKeyRequirement::Discouraged
    }
}

fn env_user_verification() -> UserVerificationRequirement {
    if let Ok(result) = env::var("WEBAUTHN_USER_VERIFICATION") {
        let resident_key: UserVerificationRequirement = serde_json::from_str(&result)
            .expect("Failed to parse WEBAUTHN_USER_VERIFICATION from env");
        resident_key
    } else {
        UserVerificationRequirement::Required
    }
}

fn env_attestation() -> AttestationConveyancePreference {
    if let Ok(result) = env::var("WEBAUTHN_CONVEYANCE_PREFERENCE") {
        let attestation: AttestationConveyancePreference = serde_json::from_str(&result)
            .expect("Failed to parse WEBAUTHN_CONVEYANCE_PREFERENCE from env");
        attestation
    } else {
        AttestationConveyancePreference::Direct
    }
}

fn env_timeout() -> usize {
    if let Ok(result) = env::var("WEBAUTHN_TIMEOUT") {
        result
            .parse::<usize>()
            .expect("Failed to parse WEBAUTHN_TIMEOUT from env")
    } else {
        360000
    }
}

fn env_validate_sign_count() -> bool {
    if let Ok(result) = env::var("WEBAUTHN_VALIDATE_SIGN_COUNT") {
        result
            .parse::<bool>()
            .expect("Failed to parse WEBAUTHN_VALIDATE_SIGN_COUNT from env")
    } else {
        false
    }
}

fn env_origin() -> Url {
    if let Ok(url) = env::var("WEBAUTHN_ORIGIN") {
        Url::try_from(url.as_str()).expect("Failed to parse WEBAUTHN_ORIGIN from env")
    } else {
        Url::try_from("http://localhost:3000").expect("Url parse error")
    }
}

fn default_webauthn_policy() -> WebauthnPolicy {
    let rp_id = env::var("WEBAUTHN_RP_ID").unwrap_or_else(|_| "localhost".to_string());
    let rp_name = env::var("WEBAUTHN_RP_NAME").unwrap_or_else(|_| "swankymutt".to_string());
    let key_type = PublicKeyCredentialType::PublicKey;

    let origin = env_origin();
    let authenticator_transports = env_transports();
    let alg = env_alg();
    let authenticator_attachment = env_authenticator_attachment();
    let resident_key = env_resident_key();
    let user_verification = env_user_verification();
    let attestation = env_attestation();
    let timeout = env_timeout();
    let validate_sign_count = env_validate_sign_count();

    // The builder will fail if every policy element is not explicitly set.  There
    // are no defaults inside [WebAuthnPolicy] or the builder itself.
    WebauthnPolicyBuilder::default()
        .with_origin(origin)
        .with_rp_id(rp_id)
        .with_rp_name(rp_name)
        .with_key_type(key_type)
        .with_alg(alg)
        .with_authenticator_attachment(authenticator_attachment)
        .with_resident_key(resident_key)
        .with_user_verification(user_verification)
        .with_attestation(attestation)
        .with_timeout(timeout)
        .with_validate_sign_count(validate_sign_count)
        .with_authenticator_transports(authenticator_transports)
        .build()
        .expect("Failed to build WebauthnPolicy.  Likely due to missing policy statements")
}
