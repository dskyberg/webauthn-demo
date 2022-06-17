use serde::{Deserialize, Serialize};

use crate::webauthn::model::WebauthnPolicy;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub webauthn: WebauthnPolicy,
}
