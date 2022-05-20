use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(i32)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[allow(non_camel_case_types)]
pub enum COSEAlgorithm {
    RSS512 = -259,
    RSS384 = -258,
    RSS256 = -257,
    ES256 = -47,
    HSS_LMS = -46,
    SHA512 = 44,
    SHA384 = -43,
    PS512 = -39,
    PS384 = -38,
    PS256 = -37,
    ES512 = -36,
    ES384 = -35,
    ECDH_SS__A256KW = -34,
    ECDH_SS__A192KW = -33,
    ECDH_SS__A128KW = -32,
    ECDH_ES__A256KW = -31,
    ECDH_ES__A192KW = -30,
    ECDH_ES__A128KW = -29,
    ECDH_SS__HKDF_512 = -28,
    ECDH_SS__HKDF_256 = -27,
    ECDH_ES__HKDF_512 = -26,
    ECDH_ES__HKDF_256 = -25,
}

impl Default for COSEAlgorithm {
    fn default() -> Self {
        Self::ECDH_ES__HKDF_256
    }
}
