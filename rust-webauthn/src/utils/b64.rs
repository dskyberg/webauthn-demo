//! Wrappers around [base64] for consistent use. This crate will
//! always use `URL_SAFE`
//!
use anyhow::{anyhow, Result};
use base64;

pub fn to_b64(value: &[u8]) -> String {
    base64::encode_config(value, base64::URL_SAFE)
}

pub fn from_b64(value: &[u8]) -> Result<Vec<u8>> {
    base64::decode_config(value, base64::URL_SAFE).map_err(|x| anyhow!(x))
}
