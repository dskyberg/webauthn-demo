use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResidentKeyRequirement {
    Discouraged,
    Preferred,
    Required,
}

impl Default for ResidentKeyRequirement {
    fn default() -> Self {
        Self::Discouraged
    }
}
