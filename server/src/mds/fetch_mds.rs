use super::verify_jwt;
use serde_json::Value;
use std::{collections::HashMap, env, error::Error};

async fn fetch(url: &str) -> Result<String, super::errors::Error> {
    let body = reqwest::get(url)
        .await
        .map_err(super::errors::Error::FetchFailed)?
        .text()
        .await
        .map_err(super::errors::Error::FetchFailed)?;
    log::info!("Fetched {}", url);
    Ok(body)
}

/// Default url for metadata
static FIDO_METADATA_URL: &str = "https://mds.fidoalliance.org";

pub async fn fetch_fido_mds() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let url = env::var("FIDO_METADATA_URL").unwrap_or_else(|_| FIDO_METADATA_URL.to_string());
    let body = fetch(&url).await?;
    let metadata = verify_jwt(&body)?;
    Ok(metadata)
}
