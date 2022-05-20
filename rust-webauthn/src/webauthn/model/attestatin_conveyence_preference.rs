use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttestationConveyancePreference {
    None,
    Indirect,
    Direct,
    Enterprise,
}

impl Default for AttestationConveyancePreference {
    fn default() -> Self {
        Self::Direct
    }
}
