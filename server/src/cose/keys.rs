//! Module to encode/decode cose-keys/cose-keySet.
use cbor::{decoder::DecodeError, types::Type, Config, Decoder, Encoder};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::str::from_utf8;

use super::algs;
use super::common;
use super::errors::{CoseError, CoseResult, CoseResultWithRet};
//COMMON PARAMETERS
pub const D: i32 = -4;
pub const Y: i32 = -3;
pub const X: i32 = -2;
pub const CRV_K: i32 = -1;
pub const KTY: i32 = 1;
pub const KID: i32 = 2;
pub const ALG: i32 = 3;
pub const KEY_OPS: i32 = 4;
pub const BASE_IV: i32 = 5;

//KEY TYPES
pub const OKP: i32 = 1;
pub const EC2: i32 = 2;
pub const SYMMETRIC: i32 = 4;
pub const RESERVED: i32 = 0;
pub const KTY_ALL: [i32; 4] = [RESERVED, OKP, EC2, SYMMETRIC];
pub const KTY_NAMES: [&str; 4] = ["Reserved", "OKP", "EC2", "Symmetric"];

//KEY OPERATIONS
pub const KEY_OPS_SIGN: i32 = 1;
pub const KEY_OPS_VERIFY: i32 = 2;
pub const KEY_OPS_ENCRYPT: i32 = 3;
pub const KEY_OPS_DECRYPT: i32 = 4;
pub const KEY_OPS_WRAP: i32 = 5;
pub const KEY_OPS_UNWRAP: i32 = 6;
pub const KEY_OPS_DERIVE: i32 = 7;
pub const KEY_OPS_DERIVE_BITS: i32 = 8;
pub const KEY_OPS_MAC: i32 = 9;
pub const KEY_OPS_MAC_VERIFY: i32 = 10;
pub const KEY_OPS_ALL: [i32; 10] = [
    KEY_OPS_SIGN,
    KEY_OPS_VERIFY,
    KEY_OPS_ENCRYPT,
    KEY_OPS_DECRYPT,
    KEY_OPS_WRAP,
    KEY_OPS_UNWRAP,
    KEY_OPS_DERIVE,
    KEY_OPS_DERIVE_BITS,
    KEY_OPS_MAC,
    KEY_OPS_MAC_VERIFY,
];
pub const KEY_OPS_NAMES: [&str; 10] = [
    "sign",
    "verify",
    "encrypt",
    "decrypt",
    "wrap key",
    "unwrap key",
    "derive key",
    "derive bits",
    "MAC create",
    "MAC verify",
];

//CURVES
pub const P_256: i32 = 1;
pub const P_384: i32 = 2;
pub const P_521: i32 = 3;
pub const X25519: i32 = 4;
pub const X448: i32 = 5;
pub const ED25519: i32 = 6;
pub const ED448: i32 = 7;
pub const CURVES_ALL: [i32; 7] = [P_256, P_384, P_521, X25519, X448, ED25519, ED448];
pub const CURVES_NAMES: [&str; 7] = [
    "P-256", "P-384", "P-521", "X25519", "X448", "Ed25519", "Ed448",
];

/// cose-key structure.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CoseKey {
    /// cose-key encoded bytes.
    pub bytes: Vec<u8>,
    labels_found: Vec<i32>,
    used: Vec<i32>,
    /// Key Type.
    pub kty: Option<i32>,
    /// Base Initialization Vector.
    pub base_iv: Option<Vec<u8>>,
    /// List of Key Operations.
    pub key_ops: Vec<i32>,
    /// COSE Algorithm.
    pub alg: Option<i32>,
    /// Public Key X parameter for OKP/EC2 Keys.
    pub x: Option<Vec<u8>>,
    /// Public Key Y parameter for EC2 Keys.
    pub y: Option<Vec<u8>>,
    /// Private Key D parameter for OKP/EC2 Keys.
    pub d: Option<Vec<u8>>,
    /// Key value for Symmetric Keys.
    pub k: Option<Vec<u8>>,
    /// Key ID.
    pub kid: Option<Vec<u8>>,
    /// COSE curve for OKP/EC2 keys.
    pub crv: Option<i32>,
}

impl Default for CoseKey {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseKey {
    /// Creates an empty CoseKey structure
    pub fn new() -> CoseKey {
        CoseKey {
            bytes: Vec::new(),
            labels_found: Vec::new(),
            used: Vec::new(),
            key_ops: Vec::new(),
            base_iv: None,
            kty: None,
            alg: None,
            x: None,
            y: None,
            d: None,
            k: None,
            kid: None,
            crv: None,
        }
    }

    fn reg_label(&mut self, label: i32) {
        self.used.retain(|&x| x != label);
        self.used.push(label);
    }

    /// Adds Key Type to the cose-key.
    pub fn kty(&mut self, kty: i32) {
        self.reg_label(KTY);
        self.kty = Some(kty);
    }

    /// Adds Key ID to the cose-key.
    pub fn kid(&mut self, kid: Vec<u8>) {
        self.reg_label(KID);
        self.kid = Some(kid);
    }

    /// Adds Algorithm to cose-key.
    pub fn alg(&mut self, alg: i32) {
        self.reg_label(ALG);
        self.alg = Some(alg);
    }

    /// Adds Key Operations to the cose-key.
    pub fn key_ops(&mut self, key_ops: Vec<i32>) {
        self.reg_label(KEY_OPS);
        self.key_ops = key_ops;
    }

    /// Adds Base Initialization Vector to the cose-key.
    pub fn base_iv(&mut self, base_iv: Vec<u8>) {
        self.reg_label(BASE_IV);
        self.base_iv = Some(base_iv);
    }

    /// Adds Curve to the cose-key.
    pub fn crv(&mut self, crv: i32) {
        self.reg_label(CRV_K);
        self.crv = Some(crv);
    }

    /// Adds X parameter to the cose-key.
    pub fn x(&mut self, x: Vec<u8>) {
        self.reg_label(X);
        self.x = Some(x);
    }

    /// Adds Y parameter to the cose-key.
    pub fn y(&mut self, y: Vec<u8>) {
        self.reg_label(Y);
        self.y = Some(y);
    }

    /// Adds D parameter to the cose-key.
    pub fn d(&mut self, d: Vec<u8>) {
        self.reg_label(D);
        self.d = Some(d);
    }

    /// Adds Symmetric Key value to the cose-key.
    pub fn k(&mut self, k: Vec<u8>) {
        self.reg_label(CRV_K);
        self.k = Some(k);
    }

    /// Method to encode the cose-Key.
    pub fn encode(&mut self) -> CoseResult {
        let mut e = Encoder::new(Vec::new());
        self.encode_key(&mut e)?;
        self.bytes = e.into_writer().to_vec();
        Ok(())
    }

    pub(crate) fn encode_key(&self, e: &mut Encoder<Vec<u8>>) -> CoseResult {
        let kty = *self
            .kty
            .as_ref()
            .ok_or_else(|| CoseError::MissingParameter("KTY".to_string()))?;
        let key_ops = self.key_ops.clone();
        let key_ops_len = key_ops.len();
        if key_ops_len > 0 {
            if kty == EC2 || kty == OKP {
                if key_ops.contains(&KEY_OPS_VERIFY)
                    || key_ops.contains(&KEY_OPS_DERIVE)
                    || key_ops.contains(&KEY_OPS_DERIVE_BITS)
                {
                    if self.x == None {
                        return Err(CoseError::MissingParameter("x".to_string()));
                    } else if self.crv == None {
                        return Err(CoseError::MissingParameter("curve".to_string()));
                    }
                }
                if key_ops.contains(&KEY_OPS_SIGN) {
                    if self.d == None {
                        return Err(CoseError::MissingParameter("d".to_string()));
                    } else if self.crv == None {
                        return Err(CoseError::MissingParameter("curve".to_string()));
                    }
                }
            } else if kty == SYMMETRIC
                && (key_ops.contains(&KEY_OPS_ENCRYPT)
                    || key_ops.contains(&KEY_OPS_MAC_VERIFY)
                    || key_ops.contains(&KEY_OPS_MAC)
                    || key_ops.contains(&KEY_OPS_DECRYPT)
                    || key_ops.contains(&KEY_OPS_UNWRAP)
                    || key_ops.contains(&KEY_OPS_WRAP))
            {
                if self.x != None {
                    return Err(CoseError::InvalidParameter("x".to_string()));
                } else if self.y != None {
                    return Err(CoseError::InvalidParameter("y".to_string()));
                } else if self.d != None {
                    return Err(CoseError::InvalidParameter("d".to_string()));
                }
                if self.k == None {
                    return Err(CoseError::MissingParameter("k".to_string()));
                }
            }
        }
        e.object(self.used.len())?;
        for i in self.used.clone() {
            e.i32(i)?;

            if i == KTY {
                e.i32(kty)?;
            } else if i == KEY_OPS {
                e.array(self.key_ops.len())?;
                for x in &key_ops {
                    e.i32(*x)?;
                }
            } else if i == CRV_K {
                if self.crv != None {
                    e.i32(
                        self.crv
                            .ok_or_else(|| CoseError::MissingParameter("curve".to_string()))?,
                    )?;
                } else {
                    e.bytes(
                        self.k
                            .as_ref()
                            .ok_or_else(|| CoseError::MissingParameter("k".to_string()))?,
                    )?;
                }
            } else if i == KID {
                e.bytes(
                    self.kid
                        .as_ref()
                        .ok_or_else(|| CoseError::MissingParameter("KID".to_string()))?,
                )?;
            } else if i == ALG {
                e.i32(self.alg.ok_or(CoseError::MissingAlgorithm())?)?
            } else if i == BASE_IV {
                e.bytes(
                    self.base_iv
                        .as_ref()
                        .ok_or_else(|| CoseError::MissingParameter("base iv".to_string()))?,
                )?
            } else if i == X {
                e.bytes(
                    self.x
                        .as_ref()
                        .ok_or_else(|| CoseError::MissingParameter("x".to_string()))?,
                )?
            } else if i == Y {
                e.bytes(
                    self.y
                        .as_ref()
                        .ok_or_else(|| CoseError::MissingParameter("y".to_string()))?,
                )?
            } else if i == D {
                e.bytes(
                    self.d
                        .as_ref()
                        .ok_or_else(|| CoseError::MissingParameter("d".to_string()))?,
                )?
            } else {
                return Err(CoseError::InvalidLabel(i));
            }
        }
        Ok(())
    }

    /// Method to decode a cose-Key.
    pub fn decode(&mut self) -> CoseResult {
        let input = Cursor::new(self.bytes.clone());
        let mut d = Decoder::new(Config::default(), input);

        self.decode_key(&mut d)?;
        Ok(())
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, CoseError> {
        let mut key = Self::new();
        key.bytes = bytes.to_vec();
        match key.decode() {
            Ok(_) => Ok(key),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn decode_key(&mut self, d: &mut Decoder<Cursor<Vec<u8>>>) -> CoseResult {
        let mut label: i32;
        self.labels_found = Vec::new();
        self.used = Vec::new();
        for _ in 0..d.object()? {
            label = d.i32()?;
            if !self.labels_found.contains(&label) {
                self.labels_found.push(label);
            } else {
                return Err(CoseError::DuplicateLabel(label));
            }
            if label == KTY {
                let type_info = d.kernel().typeinfo()?;
                if type_info.0 == Type::Text {
                    self.kty = Some(common::get_kty_id(
                        from_utf8(&d.kernel().raw_data(type_info.1, common::MAX_BYTES)?)
                            .unwrap()
                            .to_string(),
                    )?);
                } else if common::CBOR_NUMBER_TYPES.contains(&type_info.0) {
                    self.kty = Some(d.kernel().i32(&type_info)?);
                } else {
                    return Err(CoseError::InvalidCoseStructure());
                }
                self.used.push(label);
            } else if label == ALG {
                let type_info = d.kernel().typeinfo()?;
                if type_info.0 == Type::Text {
                    self.alg = Some(common::get_alg_id(
                        from_utf8(&d.kernel().raw_data(type_info.1, common::MAX_BYTES)?)
                            .unwrap()
                            .to_string(),
                    )?);
                } else if common::CBOR_NUMBER_TYPES.contains(&type_info.0) {
                    self.alg = Some(d.kernel().i32(&type_info)?);
                } else {
                    return Err(CoseError::InvalidCoseStructure());
                }
                self.used.push(label);
            } else if label == KID {
                self.kid = Some(d.bytes()?);
                self.used.push(label);
            } else if label == KEY_OPS {
                let mut key_ops = Vec::new();
                for _i in 0..d.array()? {
                    let type_info = d.kernel().typeinfo()?;
                    if type_info.0 == Type::Text {
                        key_ops.push(common::get_key_op_id(
                            from_utf8(&d.kernel().raw_data(type_info.1, common::MAX_BYTES)?)
                                .unwrap()
                                .to_string(),
                        )?);
                    } else if common::CBOR_NUMBER_TYPES.contains(&type_info.0) {
                        key_ops.push(d.kernel().i32(&type_info)?);
                    } else {
                        return Err(CoseError::InvalidCoseStructure());
                    }
                }
                self.key_ops = key_ops;
                self.used.push(label);
            } else if label == BASE_IV {
                self.base_iv = Some(d.bytes()?);
                self.used.push(label);
            } else if label == CRV_K {
                if self
                    .kty
                    .ok_or_else(|| CoseError::MissingParameter("KTY".to_string()))?
                    == SYMMETRIC
                {
                    self.k = Some(d.bytes()?);
                } else {
                    let type_info = d.kernel().typeinfo()?;
                    if type_info.0 == Type::Text {
                        self.crv = Some(common::get_crv_id(
                            from_utf8(&d.kernel().raw_data(type_info.1, common::MAX_BYTES)?)
                                .unwrap()
                                .to_string(),
                        )?);
                    } else if common::CBOR_NUMBER_TYPES.contains(&type_info.0) {
                        self.crv = Some(d.kernel().i32(&type_info)?);
                    } else {
                        return Err(CoseError::InvalidCoseStructure());
                    }
                }
                self.used.push(label);
            } else if label == X {
                self.x = Some(d.bytes()?);
                self.used.push(label);
            } else if label == Y {
                self.y = match d.bytes() {
                    Ok(value) => {
                        self.used.push(label);
                        Some(value)
                    }
                    Err(ref err) => match err {
                        DecodeError::UnexpectedType { datatype, info: _ } => {
                            if *datatype == Type::Bool {
                                None
                            } else {
                                return Err(CoseError::InvalidCoseStructure());
                            }
                        }
                        _ => {
                            return Err(CoseError::InvalidCoseStructure());
                        }
                    },
                };
            } else if label == D {
                self.d = Some(d.bytes()?);
                self.used.push(label);
            } else {
                return Err(CoseError::InvalidLabel(label));
            }
        }
        Ok(())
    }

    pub fn get_pub_key(&self, alg: i32) -> CoseResultWithRet<Vec<u8>> {
        let mut pub_key: Vec<u8>;
        if algs::SIGNING_ALGS.contains(&alg) || algs::ECDH_ALGS.contains(&alg) {
            let mut x = self
                .x
                .as_ref()
                .ok_or_else(|| CoseError::MissingParameter("x".to_string()))?
                .to_vec();
            if x.is_empty() {
                return Err(CoseError::MissingParameter("x".to_string()));
            }
            if algs::EDDSA == alg {
                //DER prefixes
                //302e020100300506032b657004220420 -> priv
                //302a300506032b6570032100 -> pub
                pub_key = vec![48, 42, 48, 5, 6, 3, 43, 101, 112, 3, 33, 0];
                pub_key.append(&mut x);
            } else if self.y == None {
                pub_key = vec![3];
                pub_key.append(&mut x);
            } else {
                let mut y = self
                    .y
                    .as_ref()
                    .ok_or_else(|| CoseError::MissingParameter("y".to_string()))?
                    .to_vec();
                pub_key = vec![4];
                pub_key.append(&mut x);
                pub_key.append(&mut y);
            }
        } else {
            return Err(CoseError::InvalidAlgorithm());
        }
        Ok(pub_key)
    }
}

/// cose-keySet structure.
pub struct CoseKeySet {
    /// List of the cose-keys.
    pub cose_keys: Vec<CoseKey>,
    /// COSE encoded key set.
    pub bytes: Vec<u8>,
}

impl Default for CoseKeySet {
    fn default() -> Self {
        Self::new()
    }
}

impl CoseKeySet {
    /// Creates a new empty structure.
    pub fn new() -> CoseKeySet {
        CoseKeySet {
            cose_keys: Vec::new(),
            bytes: Vec::new(),
        }
    }

    /// Adds a cose-key to the cose-keySet.
    pub fn add_key(&mut self, key: CoseKey) {
        self.cose_keys.push(key);
    }

    /// Encodes the cose-keySet.
    pub fn encode(&mut self) -> CoseResult {
        let mut e = Encoder::new(Vec::new());
        let len = self.cose_keys.len();
        e.array(len)?;
        for i in 0..len {
            self.cose_keys[i].encode_key(&mut e)?;
        }
        self.bytes = e.into_writer().to_vec();
        Ok(())
    }

    /// Decodes an encoded cose-keySet.
    ///
    /// The COSE encoded bytes of the cose-keySet must be set with the structure attribute bytes beforehand.
    pub fn decode(&mut self) -> CoseResult {
        let input = Cursor::new(self.bytes.clone());
        let mut d = Decoder::new(Config::default(), input);
        let len = d.array()?;
        for _ in 0..len {
            let mut cose_key = CoseKey::new();
            cose_key.decode_key(&mut d)?;
            self.cose_keys.push(cose_key);
        }
        Ok(())
    }

    /// Function that returns a cose-key from the cose-keySet with a given Key ID.
    pub fn get_key(&self, kid: &[u8]) -> CoseResultWithRet<CoseKey> {
        for i in 0..self.cose_keys.len() {
            if self.cose_keys[i]
                .kid
                .as_ref()
                .ok_or_else(|| CoseError::MissingParameter("KID".to_string()))?
                == kid
            {
                return Ok(self.cose_keys[i].clone());
            }
        }
        Err(CoseError::MissingKey())
    }
}
