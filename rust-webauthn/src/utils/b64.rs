//! Wrappers around [base64] for consistent use. This crate will
//! always use `URL_SAFE`
//!
use anyhow::{anyhow, Result};
use base64;

pub fn to_b64(value: &[u8]) -> String {
    // base64::encode(value)
    base64::encode_config(value, base64::URL_SAFE_NO_PAD)
}

pub fn from_b64(value: &str) -> Result<Vec<u8>> {
    // base64::decode(value).map_err(|x| anyhow!(x))
    base64::decode_config(value, base64::URL_SAFE_NO_PAD).map_err(|x| anyhow!(x))
}
