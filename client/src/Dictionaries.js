/**
 * Dictionaries
 * @ignore
 */
const Dictionaries = {
  /**
   * UserVerificationRequirement
   * @link https://www.w3.org/TR/webauthn/#enumdef-userverificationrequirement
   */
  UserVerificationRequirement: {
    Required: 'required',
    Preferred: 'preferred',
    Discouraged: 'discouraged',
  },

  /**
   * AuthenticatorAttachment
   * @link https://www.w3.org/TR/webauthn/#enumdef-authenticatorattachment
   */
  AuthenticatorAttachment: {
    Platform: 'platform',
    'Cross Plafform': 'cross-platform',
    'Multi Platform': 'multi-latform',
  },
  /**
   * AttestationConveyancePreference
   * @link https://www.w3.org/TR/webauthn/#attestation-convey
   */
  AttestationConveyancePreference: {
    None: 'none',
    Direct: 'direct',
    Indirect: 'indirect',
  },

  /**
   * PublicKeyCredentialType
   * @link https://www.w3.org/TR/webauthn/#credentialType
   */
  PublicKeyCredentialType: {
    'Public Key': 'public-key',
  },

  /**
   * AuthenticatorTransport
   * @link https://www.w3.org/TR/webauthn/#enumdef-authenticatortransport
   */
  AuthenticatorTransport: {
    USB: 'usb',
    NFC: 'nfc',
    BLE: 'ble',
    INTERNAL: 'internal',
  },
}

/**
 * Exports
 * @ignore
 */
Object.freeze(Dictionaries)
module.exports = Dictionaries
