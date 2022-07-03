use actix_session::Session;
use base64urlsafedata::Base64UrlSafeData;

use crate::errors::Error;

pub fn b64_from_session(session: &Session, name: &str) -> Result<Base64UrlSafeData, Error> {
    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    match session.get::<String>(name).map_err(|_| {
        log::info!("Failed to get {} from session", name);
        Error::SessionError(format!("Failed to get {} from session", name))
    })? {
        Some(val) => {
            let x = Base64UrlSafeData::try_from(val.as_str())
                .map_err(|_| Error::Base64UrlSafeDataError)?;
            Ok(x)
        }
        None => Err(Error::SessionError(format!(
            "Failed to get {} from session",
            name
        ))),
    }
}

pub fn str_from_session(session: &Session, name: &str) -> Result<String, Error> {
    // Get the challenge and name that was placed in the session
    // by register_challenge_request
    match session.get::<String>(name).map_err(|_| {
        log::info!("Failed to get {} from session", name);
        Error::SessionError(format!("Failed to get {} from session", name))
    })? {
        Some(val) => Ok(val),
        None => Err(Error::SessionError(format!(
            "Failed to get {} from session",
            name
        ))),
    }
}
