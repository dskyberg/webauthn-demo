use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthenticatorTransport {
    #[serde(rename = "usb")]
    USB,
    #[serde(rename = "NFC")]
    NFC,
    #[serde(rename = "ble")]
    BLE,
    #[serde(rename = "internal")]
    Internal,
}
