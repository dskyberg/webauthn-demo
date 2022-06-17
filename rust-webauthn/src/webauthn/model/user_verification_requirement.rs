use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserVerificationRequirement {
    Discouraged,
    Preferred,
    Required,
}
