use serde::{Deserialize, Serialize};

use self::visual_appearance::VisualAppearance;

pub mod visual_appearance;

/// Represents a pass (pass.json file)
#[derive(Serialize, Deserialize, Debug)]
pub struct Pass {
    /// (Required) The version of the file format. The value must be 1.
    pub format_version: u32,

    /// (Required) The name of the organization.
    pub organization_name: String,

    /// (Required) A short description that iOS accessibility technologies use for a pass.
    pub description: String,

    /// (Required) The pass type identifier that’s registered with Apple.
    /// The value must be the same as the distribution certificate used to sign the pass.
    pub pass_type_identifier: String,

    /// (Required) The Team ID for the Apple Developer Program account that registered the pass type identifier.
    pub team_identifier: String,

    /// (Required) An alphanumeric serial number.
    /// The combination of the serial number and pass type identifier must be unique for each pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// An identifier the system uses to group related boarding passes or event tickets.
    /// Wallet displays passes with the same [grouping_identifier](Pass::grouping_identifier), [pass_type_identifier](Pass::pass_type_identifier), and type as a group.
    ///
    /// Use this identifier to group passes that are tightly related, such as boarding passes for different connections on the same trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_identifier: Option<String>,

    // Colors and other visual parts of the pass
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<VisualAppearance>,

    /// The date and time when the pass becomes relevant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_date: Option<String>,

    /// The date and time the pass expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,

    /// A URL to be passed to the associated app when launching it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_launch_url: Option<String>,

    /// An array of App Store identifiers for apps associated with the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_store_identifiers: Vec<i32>,

    /// The authentication token to use with the web service in the [web_service_url](Pass::web_service_url) key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,

    /// The URL for a web service that you use to update or personalize the pass. The URL can include an optional port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_service_url: Option<String>,

    /// Controls whether to show the Share button on the back of a pass.
    /// A value of true removes the button. The default value is false.
    /// This flag has no effect in earlier versions of iOS, nor does it prevent sharing the pass in some other way.
    #[serde(skip_serializing_if = "is_false")]
    pub sharing_prohibited: bool, // TODO: default false

    /// Controls whether to display the strip image without a shine effect.
    /// The default value is true.
    #[serde(skip_serializing_if = "is_true")]
    pub suppress_strip_shine: bool, // TODO: default true

    /// Indicates that the pass is void, such as a redeemed, one-time-use coupon.
    /// The default value is false.
    #[serde(skip_serializing_if = "is_false")]
    pub voided: bool, // TODO: default false

    // TODO: Barcode on a pass
    // The system uses the first displayable barcode for the device.
    // pub barcodes: Vec<Barcode>,

    // TODO: Array of Bluetooth Low Energy beacons the system uses to show a relevant pass.
    // pub beacons: Vec<Beacon>,

    // TODO: An array of up to 10 geographic locations the system uses to show a relevant pass.
    // pub locations: Vec<Location>,
    /// The maximum distance, in meters, from a location in the [locations](Pass::locations) array at which the pass is relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<u32>,
    // TODO: NFC
    // pub nfc: Option<NFC>,

    // TODO: Semantic tags
    // Metadata the system uses to offer a pass and suggest related actions.
    // For example, setting Don’t Disturb mode for the duration of a movie.
    // pub semantics: Vec<SemanticTag>,

    // TODO: PassTypes
    // boarding pass
    // coupon
    // event ticket
    // generic

    // TODO: UserInfo
    // custom JSOM
}

impl Default for Pass {
    /// Creates an empty `Pass`.
    fn default() -> Self {
        Self {
            format_version: 1,
            organization_name: Default::default(),
            description: Default::default(),
            pass_type_identifier: Default::default(),
            team_identifier: Default::default(),
            serial_number: None,
            grouping_identifier: None,
            appearance: None,
            relevant_date: None,
            expiration_date: None,
            app_launch_url: None,
            associated_store_identifiers: Vec::new(),
            authentication_token: None,
            web_service_url: None,
            sharing_prohibited: false,
            suppress_strip_shine: true,
            voided: false,
            max_distance: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_default_pass() {
        let pass = Pass {
            organization_name: String::from("Apple inc."),
            description: String::from("Example pass"),
            pass_type_identifier: String::from("com.example.pass"),
            team_identifier: String::from("AA00AA0A0A"),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&pass).unwrap();

        println!("{}", serde_json::to_string_pretty(&pass).unwrap());

        let json_expected = r#"{
  "format_version": 1,
  "organization_name": "Apple inc.",
  "description": "Example pass",
  "pass_type_identifier": "com.example.pass",
  "team_identifier": "AA00AA0A0A"
}"#;

        assert_eq!(json_expected, json);
    }
}

// For serde skipping - if boolean false
fn is_false(b: &bool) -> bool {
    !b
}
// For serde skipping - if boolean true
fn is_true(b: &bool) -> bool {
    *b
}
