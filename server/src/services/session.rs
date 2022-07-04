//! Session Management
//!
//! Since Safari has issues with setting cookies on XHR, we use
//! our own session management.
//!
//! This works because the client is configured to copy the X-SESSION
//! header on XDR calls.
//!
use actix_web::HttpRequest;
use base64urlsafedata::Base64UrlSafeData;
use std::collections::HashMap;

use crate::{errors::Error, services::DataServices, utils::make_id};

/// This header MUST be managed by the client!!
const DEFAULT_HEADER_NAME: &str = "x-session";

pub type SessionData = HashMap<String, String>;
pub struct Session {
    pub id: Base64UrlSafeData,
    pub entries: SessionData,
}

impl Session {
    pub fn new() -> Self {
        let id = Base64UrlSafeData(make_id(16).unwrap());
        let entries = HashMap::<String, String>::new();
        Self { id, entries }
    }

    pub fn get(&self, item: &str) -> Option<String> {
        if self.entries.contains_key(item) {
            self.entries.get(item).cloned()
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.entries.insert(key.to_owned(), value.to_owned());
    }

    pub fn to_header(&self) -> (String, String) {
        let name = DEFAULT_HEADER_NAME.to_owned();
        let value = self.id.clone().to_string();
        (name, value)
    }

    pub fn as_b64(&self, name: &str) -> Result<Base64UrlSafeData, Error> {
        // Get the challenge and name that was placed in the session
        // by register_challenge_request
        match self.get(name) {
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

    pub fn as_str(&self, name: &str) -> Result<String, Error> {
        // Get the challenge and name that was placed in the session
        // by register_challenge_request
        match self.get(name) {
            Some(val) => Ok(val),
            None => Err(Error::SessionError(format!(
                "Failed to get {} from session",
                name
            ))),
        }
    }

    pub async fn put_session(&self, service: &DataServices) -> Result<(), Error> {
        log::trace!("Saving session to storage");
        service.put_session(&self.id, &self.entries).await
    }

    pub async fn get_session(
        service: &DataServices,
        id: &Base64UrlSafeData,
    ) -> Result<Self, Error> {
        if let Some(entries) = service.get_session(id).await? {
            log::trace!(
                "Retrieving session from storage: {} - {:?}",
                id.to_string(),
                &entries
            );
            return Ok(Self {
                id: id.to_owned(),
                entries,
            });
        }
        log::trace!("Failed to retrieve session: {}", id.to_string());
        Err(Error::SessionNotFound)
    }

    pub async fn from_request(
        service: &DataServices,
        request: &HttpRequest,
    ) -> Result<Self, Error> {
        if let Some(header_value) = request.headers().get(DEFAULT_HEADER_NAME) {
            log::trace!("Found sesion header.  Fetching sesion");
            let val = header_value.to_str().map_err(|_| Error::BadSessionHeader)?;
            let id = Base64UrlSafeData::try_from(val).map_err(|_| Error::Base64UrlSafeDataError)?;
            return Self::get_session(service, &id).await;
        }
        log::trace!("Session header not found");
        Err(Error::Base64UrlSafeDataError)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
