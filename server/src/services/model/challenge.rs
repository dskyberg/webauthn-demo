/// This module enables tracking Challenge values to ensure
/// they follow a 'use once' strategy.  
///
/// The expectation is that [DataServices] provides methods for atomically
/// creating and storing a new challenge.  And subsequently 'using' the challenge.
///
use crate::utils::make_id;
use base64urlsafedata::Base64UrlSafeData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    /// The Base64 challenge value
    pub value: Base64UrlSafeData,
    /// Used suggests that a client has returned the challenge
    pub used: bool,
    /// Not currently used
    pub ttl: isize,
    /// The time the challenge was 'used'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_time: Option<DateTime<Utc>>,
}

impl Default for Challenge {
    fn default() -> Self {
        Self::new()
    }
}

impl Challenge {
    /// Generate a new challenge value from a cryptographically random value.
    pub fn new() -> Self {
        Self {
            value: Base64UrlSafeData(make_id(32).unwrap()),
            used: false,
            ttl: 0,
            used_time: None,
        }
    }

    /// Mark the challenge as used.  Includes setting the UTC time that this happens.
    pub fn mark_used(&self) -> Self {
        let mut new = self.clone();
        new.used_time = Some(Utc::now());
        new.used = true;
        new
    }
}

/// Create a new Challenge from an existing randome value
impl From<&Base64UrlSafeData> for Challenge {
    fn from(value: &Base64UrlSafeData) -> Self {
        Self {
            value: value.clone(),
            used: false,
            ttl: 0,
            used_time: None,
        }
    }
}
