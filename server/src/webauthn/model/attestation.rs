use base64urlsafedata::Base64UrlSafeData;
use serde::Deserialize;
use serde_cbor::Value;

use crate::errors::Error;

use super::{AttestationFormatIdentifier, AttestationStatement, AuthenticatorData};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AttestationInner {
    pub(crate) fmt: AttestationFormatIdentifier,
    pub(crate) att_stmt: AttestationStatement,
    pub(crate) auth_data: serde_cbor::Value,
}

#[derive(Debug)]
pub struct Attestation {
    pub fmt: AttestationFormatIdentifier,
    pub att_stmt: AttestationStatement,
    pub auth_data: AuthenticatorData,
    pub auth_data_bytes: Vec<u8>,
}

impl TryFrom<&[u8]> for Attestation {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        log::info!("Attestation::try_from");
        let ai: AttestationInner = serde_cbor::from_slice(data).map_err(|e| {
            log::info!("serde_cbor failed");
            Error::AttestationParseError(e)
        })?;
        let auth_data_bytes = cbor_try_bytes(&ai.auth_data).map_err(|e| {
            log::info!("Attestation::try_from: cbor_try_bytes failed");
            e
        })?;
        let auth_data = AuthenticatorData::try_from(&ai.auth_data).map_err(|e| {
            log::info!("Attestation::try_from: AuthenticatorData::try_from failed");
            e
        })?;
        Ok(Attestation {
            fmt: ai.fmt,
            att_stmt: ai.att_stmt,
            auth_data,
            auth_data_bytes,
        })
    }
}

impl TryFrom<&Base64UrlSafeData> for Attestation {
    type Error = Error;
    fn try_from(b64: &Base64UrlSafeData) -> Result<Self, Self::Error> {
        Attestation::try_from(b64.as_ref())
    }
}

fn cbor_try_bytes(value: &Value) -> Result<Vec<u8>, Error> {
    match value {
        Value::Bytes(bytes) => Ok(bytes.to_owned()),
        _ => {
            log::info!("CBOR is not a Vec<u8> value");
            Err(Error::AttestationObjectError(
                "Not Value::Bytes".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64urlsafedata::Base64UrlSafeData;
    #[test]
    fn test_it() {
        let data: Vec<u8> = vec![
            163, 99, 102, 109, 116, 102, 112, 97, 99, 107, 101, 100, 103, 97, 116, 116, 83, 116,
            109, 116, 162, 99, 97, 108, 103, 38, 99, 115, 105, 103, 88, 72, 48, 70, 2, 33, 0, 186,
            255, 250, 233, 204, 53, 205, 105, 152, 34, 254, 243, 142, 53, 25, 119, 203, 146, 117,
            114, 108, 22, 116, 41, 222, 190, 216, 135, 155, 174, 164, 202, 2, 33, 0, 207, 208, 26,
            231, 177, 35, 14, 71, 135, 185, 88, 11, 55, 158, 119, 60, 78, 57, 117, 147, 195, 205,
            81, 95, 173, 165, 225, 237, 210, 82, 191, 140, 104, 97, 117, 116, 104, 68, 97, 116, 97,
            88, 205, 73, 150, 13, 229, 136, 14, 140, 104, 116, 52, 23, 15, 100, 118, 96, 91, 143,
            228, 174, 185, 162, 134, 50, 199, 153, 92, 243, 186, 131, 29, 151, 99, 69, 0, 0, 0, 0,
            173, 206, 0, 2, 53, 188, 198, 10, 100, 139, 11, 37, 241, 240, 85, 3, 0, 73, 95, 191,
            221, 204, 49, 132, 11, 135, 215, 52, 83, 99, 249, 106, 57, 239, 170, 41, 22, 128, 232,
            129, 36, 26, 37, 64, 217, 109, 81, 160, 84, 124, 103, 116, 230, 98, 67, 171, 79, 91,
            58, 47, 116, 82, 86, 198, 236, 187, 130, 204, 190, 54, 169, 117, 28, 139, 8, 78, 246,
            197, 6, 62, 242, 175, 3, 43, 138, 241, 179, 25, 96, 252, 93, 165, 1, 2, 3, 38, 32, 1,
            33, 88, 32, 12, 25, 245, 38, 7, 19, 192, 244, 246, 95, 19, 86, 171, 15, 70, 178, 5, 56,
            155, 189, 151, 83, 87, 71, 72, 39, 164, 2, 6, 246, 128, 204, 34, 88, 32, 158, 185, 45,
            166, 111, 16, 243, 155, 197, 31, 170, 192, 215, 0, 124, 161, 228, 58, 96, 182, 166, 82,
            58, 193, 120, 185, 239, 186, 189, 37, 23, 106,
        ];

        let attestation =
            Attestation::try_from(data.as_slice()).expect("Attestation failed to parse");
        dbg!(&attestation);
        let auth_data = attestation.auth_data;
        dbg!(&auth_data);
    }

    #[test]
    fn test_passkey() -> Result<(), Error> {
        let b64_data = Base64UrlSafeData::try_from("o2NmbXRkbm9uZWdhdHRTdG10oGhhdXRoRGF0YViYSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NdAAAAAAAAAAAAAAAAAAAAAAAAAAAAFISL9Nss+mNzBTJ5IBtQbJiQ01+OpQECAyYgASFYIGVmMntGKNKbK4XLrThk4AH2RZRUhfeFwbXs5C64NPG8IlggS1N0cs7yNZPP2oAYzy5NdMWh7oD+HyDd1Q4vC1Izm6c=").expect("oops");
        dbg!(&b64_data);
        let result = Attestation::try_from(&b64_data);
        assert!(result.is_ok());
        Ok(())
    }
}
