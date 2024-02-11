use serde::{Deserialize, Serialize};

/// Represents the near-field communication (NFC) payload the device passes to an Apple Pay terminal.
/// Adding NFC to a Pass requires a special entitlement issued by Apple. For more information, see Near Field Communication in [Getting Started](https://developer.apple.com/wallet/get-started/) with Apple Wallet.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NFC {
    /// (Required) The public encryption key the Value Added Services protocol uses.
    /// Use a Base64-encoded X.509 SubjectPublicKeyInfo structure that contains an ECDH public key for group P256.
    pub encryption_public_key: String,

    /// (Required) The payload the device transmits to the Apple Pay terminal.
    /// The size must be no more than 64 bytes. The system truncates messages longer than 64 bytes.
    pub message: String,

    /// Indicates whether the NFC pass requires authentication.
    /// The default value is false. A value of true requires the user to authenticate for each use of the NFC pass.
    pub requires_authentication: bool,
}

impl Default for NFC {
    /// Creates an empty `NFC`.
    fn default() -> Self {
        Self {
            encryption_public_key: String::new(),
            message: String::new(),
            requires_authentication: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_nfc() {
        let nfc = NFC {
            encryption_public_key: String::from("ABCDEFG_0011223344556677889900"),
            message: String::from("test message"),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&nfc).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "encryptionPublicKey": "ABCDEFG_0011223344556677889900",
  "message": "test message",
  "requiresAuthentication": false
}"#;

        assert_eq!(json_expected, json);
    }
}
