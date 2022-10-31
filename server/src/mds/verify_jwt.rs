use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use x509_parser::prelude::*;

pub fn verify_jwt(token: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // Pull the algorithm from the alg claim and the
    // X509 cert list from the x5c claim
    let header = decode_header(token)?;
    // This should be RS256. Grab it and use it for the Decode
    let alg = header.alg;

    // x509_parser is kind enough to provide a helper function to
    // grab the x5c list in DER format, rather than PEM.  Thanks!!
    let x5c_list = header.x5c_der()?.unwrap_or_default();

    let mut validation = Validation::new(alg);
    validation.validate_exp = false;
    validation.required_spec_claims = std::collections::HashSet::new();

    // Since order isn't guaranteed in the cert chain, try them all
    // until one succeeds.
    for der in x5c_list {
        // Parse the X.509
        let (_, cert) = X509Certificate::from_der(&der)?;
        // Get the public key in SPKI format
        let public_key_bytes = cert.subject_pki.subject_public_key.as_ref();
        // Create a key from the SPKI
        let key = DecodingKey::from_rsa_der(public_key_bytes);

        // Decode the JWT.
        let result = decode::<HashMap<String, Value>>(token, &key, &validation);

        if let Ok(token_data) = result {
            let blob = token_data.claims;
            return Ok(blob);
        }
    }
    Err(super::errors::Error::InvalidSignature.into())
}
