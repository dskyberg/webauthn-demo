//! Use discoverable credentials
//!
//! Discoverable credentials allow the client to call
//! `navigator.credentials.get()` without identifying the
//! user. Meaning only an RP ID is provided.
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResidentKeyRequirement {
    Discouraged,
    Preferred,
    Required,
}
