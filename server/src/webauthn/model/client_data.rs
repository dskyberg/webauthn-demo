use base64urlsafedata::Base64UrlSafeData;
use serde::Deserialize;
use url::Url;

/*
{
    "type":"webauthn.create",
    "challenge":"6P8pAZg6ARXv5SsttAet8XN0IJ1wyF7lZegY-tPq3BgA",
    "origin":"http://localhost:3000",
    "crossOrigin":false
}
*/

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum ClientDataType {
    #[serde(rename = "webauthn.get")]
    Get,
    #[serde(rename = "webauthn.create")]
    Create,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TokenBindingStatus {
    Present,
    Supported,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct TokenBinding {
    pub status: TokenBindingStatus,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
    #[serde(rename = "type")]
    pub client_data_type: ClientDataType,
    pub challenge: Base64UrlSafeData,
    pub origin: Url,
    pub cross_origin: Option<bool>,
    pub token_binding: Option<TokenBinding>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = r#"{
            "type":"webauthn.create",
            "challenge":"6P8pAZg6ARXv5SsttAet8XN0IJ1wyF7lZegY-tPq3BgA",
            "origin":"http://localhost:3000",
            "crossOrigin":false
        }
        "#;

        let result: ClientData = serde_json::from_str(json).expect("not yet");
        dbg!(&result);
    }
}
