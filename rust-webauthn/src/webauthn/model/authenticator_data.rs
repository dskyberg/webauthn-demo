use base64urlsafedata::Base64UrlSafeData;
use serde_cbor::Value;
use std::io::Cursor;
use std::io::Read;

use crate::cbor::keys::CoseKey;
use crate::errors::Error;

use super::*;

/// ----- attestedCredentialData -------
/// - AAGUID: 16
/// - LENGTH: 2
/// - CREDENTIAL ID: LENGTH
/// - CREDENTIAL PUBLIC KEY: (remaining) COSE_Key

#[derive(Debug, Clone)]
pub struct CredentialData {
    pub aaguid: [u8; 16],
    pub credential_id: Vec<u8>,
    pub credential_public_key: CoseKey,
    pub extensions: Option<Base64UrlSafeData>,
}

impl CredentialData {
    pub fn get_public_key(&self, alg: COSEAlgorithm) -> Result<Vec<u8>, Error> {
        self.credential_public_key
            .get_pub_key(alg as i32)
            .map_err(|_| Error::AuthenticatorDataPublicKeyError)
    }
}

impl TryFrom<&[u8]> for CredentialData {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Error> {
        log::info!("CredentialData::try_from start");
        let data_len: usize = data.len();
        let mut remainder = data_len;
        let mut file = Cursor::new(data);

        let mut aaguid: [u8; 16] = [0; 16];
        let bytes_read = file
            .read(&mut aaguid)
            .map_err(|_| Error::AuthenticatorDataDeserialize("AAGUID".to_string()))?;
        remainder -= bytes_read;

        let mut length: [u8; 2] = [0; 2];
        let bytes_read = file
            .read(&mut length)
            .map_err(|_| Error::AuthenticatorDataDeserialize("LENGTH".to_string()))?;
        remainder -= bytes_read;

        let length = u16::from_be_bytes(length);

        let mut credential_id: Vec<u8> = vec![0; length as usize];
        let bytes_read = file
            .read(&mut credential_id)
            .map_err(|_| Error::AuthenticatorDataDeserialize("CREDENTIAL ID".to_string()))?;

        remainder -= bytes_read;

        let mut credential_public_key_bytes: Vec<u8> = vec![0; remainder];
        let _ = file
            .read(&mut credential_public_key_bytes)
            .map_err(|_| Error::AuthenticatorDataDeserialize("COSE PUBLICKEY".to_string()))?;
        let credential_public_key =
            CoseKey::decode_bytes(&credential_public_key_bytes).map_err(Error::CoseKeyError)?;

        log::info!("CredentialData::try_from succeeded");
        Ok(Self {
            aaguid,
            credential_id,
            credential_public_key,
            extensions: None,
        })
    }
}

/// Byte data:
/// - RP ID hash: 32
/// - FLAGS: 1
/// - COUNTER: 4 (big endian)
/// - attestedCredentialData
#[derive(Debug, Clone)]
pub struct AuthenticatorData {
    pub rp_id_hash: [u8; 32],
    pub flags: u8,
    pub counter: u32,
    pub credential_data: Option<CredentialData>,
}

pub const USER_PRESENT: u8 = 1;
pub const USER_VERIFIED: u8 = 4;
pub const ATTESTED_CREDENTIAL_DATA_INCLUDED: u8 = 64;
pub const EXTENSION_DATA_INCLUDED: u8 = 128;

impl TryFrom<&Value> for AuthenticatorData {
    type Error = Error;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(data) => Self::try_from(data.as_slice()),
            _ => Err(Error::AuthenticatorDataDeserialize(
                "RP ID Hash".to_string(),
            )),
        }
    }
}

impl TryFrom<&[u8]> for AuthenticatorData {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let front_matter_len = 37;
        let data_len: usize = data.len();
        let mut file = Cursor::new(data);

        let mut rp_id_hash: [u8; 32] = [0; 32];
        let _ = file
            .read(&mut rp_id_hash)
            .map_err(|_| Error::AuthenticatorDataDeserialize("RP ID Hash".to_string()))?;

        let mut flags: [u8; 1] = [0; 1];
        let _ = file
            .read(&mut flags)
            .map_err(|_| Error::AuthenticatorDataDeserialize("FLAGS".to_string()))?;
        let flags = flags[0];

        let mut counter: [u8; 4] = [0; 4];
        let _ = file
            .read(&mut counter)
            .map_err(|_| Error::AuthenticatorDataDeserialize("COUNTER".to_string()))?;
        let counter = u32::from_be_bytes(counter);

        // If attested credential data was included, unpack it
        let credential_data = match (flags & ATTESTED_CREDENTIAL_DATA_INCLUDED) != 0 {
            true => {
                let remainder = data_len - front_matter_len;
                let mut bytes: Vec<u8> = vec![0; remainder];
                let bytes_read = file.read(&mut bytes).map_err(|_| {
                    Error::AuthenticatorDataDeserialize("Credentialdata".to_string())
                })?;
                if remainder != bytes_read {
                    log::info!("Oops!! Too few bytes read!");
                }
                Some(CredentialData::try_from(bytes.as_ref())?)
            }
            false => None,
        };

        log::info!("AuthenticatorData::try_from succeeded");
        Ok(Self {
            rp_id_hash,
            flags,
            counter,
            credential_data,
        })
    }
}

impl AuthenticatorData {
    pub fn get_public_key(&self, alg: COSEAlgorithm) -> Result<Vec<u8>, Error> {
        match &self.credential_data {
            Some(credential_data) => credential_data.get_public_key(alg),
            None => Err(Error::AuthenticatorDataPublicKeyError),
        }
    }

    pub fn credential_data(&self) -> Result<CredentialData, Error> {
        match &self.credential_data {
            Some(credential_data) => Ok(credential_data.clone()),
            None => Err(Error::AuthenticatorDataPublicKeyError),
        }
    }

    fn test_flag(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }

    pub fn is_user_present(&self) -> bool {
        self.test_flag(USER_PRESENT)
    }

    pub fn is_user_verified(&self) -> bool {
        self.test_flag(USER_VERIFIED)
    }

    pub fn is_attested_credential_data_included(&self) -> bool {
        self.test_flag(ATTESTED_CREDENTIAL_DATA_INCLUDED)
    }
    pub fn is_extension_data_included(&self) -> bool {
        self.test_flag(EXTENSION_DATA_INCLUDED)
    }

    pub fn as_credential(&self) -> Credential {
        let credential_data = self.credential_data().expect("damnit");
        Credential {
            id: Base64UrlSafeData(credential_data.credential_id.clone()),
            type_: PublicKeyCredentialType::PublicKey,
            count: self.counter,
            aaguid: credential_data.aaguid,
            credential_public_key: credential_data.credential_public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let data: Vec<u8> = vec![
            73, 150, 13, 229, 136, 14, 140, 104, 116, 52, 23, 15, 100, 118, 96, 91, 143, 228, 174,
            185, 162, 134, 50, 199, 153, 92, 243, 186, 131, 29, 151, 99, 69, 0, 0, 0, 0, 173, 206,
            0, 2, 53, 188, 198, 10, 100, 139, 11, 37, 241, 240, 85, 3, 0, 73, 95, 191, 221, 204,
            49, 132, 11, 135, 215, 52, 83, 99, 249, 106, 57, 239, 170, 41, 22, 128, 232, 129, 36,
            26, 37, 64, 217, 109, 81, 160, 84, 124, 103, 116, 230, 98, 67, 171, 79, 91, 58, 47,
            116, 82, 86, 198, 236, 187, 130, 204, 190, 54, 169, 117, 28, 139, 8, 78, 246, 197, 6,
            62, 242, 175, 3, 43, 138, 241, 179, 25, 96, 252, 93, 165, 1, 2, 3, 38, 32, 1, 33, 88,
            32, 12, 25, 245, 38, 7, 19, 192, 244, 246, 95, 19, 86, 171, 15, 70, 178, 5, 56, 155,
            189, 151, 83, 87, 71, 72, 39, 164, 2, 6, 246, 128, 204, 34, 88, 32, 158, 185, 45, 166,
            111, 16, 243, 155, 197, 31, 170, 192, 215, 0, 124, 161, 228, 58, 96, 182, 166, 82, 58,
            193, 120, 185, 239, 186, 189, 37, 23, 106,
        ];

        let auth_data = AuthenticatorData::try_from(data.as_slice()).expect("oops");
        let pub_key = auth_data
            .credential_data()
            .expect("oops")
            .credential_public_key
            .get_pub_key(-7)
            .expect("failed");
        dbg!(auth_data.is_attested_credential_data_included());
        dbg!(auth_data.is_user_verified());
        dbg!(auth_data.is_user_present());
        dbg!(auth_data.is_extension_data_included());
        dbg!(
            &auth_data
                .credential_data()
                .expect("oops")
                .credential_public_key
        );
        dbg!(&pub_key);
    }
}
