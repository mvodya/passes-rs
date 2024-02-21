use serde::{Deserialize, Serialize};

/// Represents a barcode on a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    /// (Required) The message or payload to display as a barcode.
    pub message: String,

    /// (Required) The format of the barcode.
    /// The barcode format PKBarcodeFormatCode128 isn’t supported for watchOS.
    pub format: BarcodeFormat,

    /// The text to display near the barcode.
    /// For example, a human-readable version of the barcode data in case the barcode doesn’t scan.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,

    /// (Required) The IANA character set name of the text encoding to use to convert message from a string representation to a data representation that the system renders as a barcode.
    pub message_encoding: String,
}

impl Default for Barcode {
    /// Creates an empty `Barcode` with QR.
    fn default() -> Self {
        Self {
            message: String::new(),
            format: BarcodeFormat::QR,
            alt_text: None,
            message_encoding: String::from("iso-8859-1"),
        }
    }
}

/// Barcode format
#[derive(Serialize, Deserialize, Debug)]
pub enum BarcodeFormat {
    /// QR
    #[serde(rename = "PKBarcodeFormatQR")]
    QR,

    /// PDF417
    #[serde(rename = "PKBarcodeFormatPDF417")]
    PDF417,

    /// Aztec
    #[serde(rename = "PKBarcodeFormatAztec")]
    Aztec,

    /// Code128
    #[serde(rename = "PKBarcodeFormatCode128")]
    Code128,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_barcode() {
        // Serialization test
        let barcode = Barcode {
            message: String::from("Hello world!"),
            format: BarcodeFormat::PDF417,
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&barcode).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "message": "Hello world!",
  "format": "PKBarcodeFormatPDF417",
  "messageEncoding": "iso-8859-1"
}"#;

        assert_eq!(json_expected, json);

        // Deserialization test
        let barcode: Barcode = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&barcode).unwrap();
        assert_eq!(json_expected, json);
    }
}
