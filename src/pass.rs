use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};

use self::barcode::Barcode;
use self::beacon::Beacon;
use self::location::Location;
use self::nfc::NFC;
use self::pass_fields::{PassFields, PassType};
use self::semantic_tags::SemanticTags;
use self::visual_appearance::VisualAppearance;
use self::web_service::WebService;

pub mod barcode;
pub mod beacon;
pub mod location;
pub mod nfc;
pub mod pass_fields;
pub mod semantic_tags;
pub mod visual_appearance;
pub mod web_service;

/// Required fields for [Pass]
/// Used for [Pass] construction
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassConfig {
    /// The name of the organization.
    pub organization_name: String,

    /// A short description that iOS accessibility technologies use for a pass.
    pub description: String,

    /// The pass type identifier that’s registered with Apple.
    /// The value must be the same as the distribution certificate used to sign the pass.
    pub pass_type_identifier: String,

    /// The Team ID for the Apple Developer Program account that registered the pass type identifier.
    pub team_identifier: String,

    /// An alphanumeric serial number.
    /// The combination of the serial number and pass type identifier must be unique for each pass.
    pub serial_number: String,
}

/// Represents a pass (pass.json file)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pass {
    /// The version of the file format. The value must be 1.
    format_version: u32,

    /// Primary required fields, specified for pass.json
    #[serde(flatten)]
    pub config: PassConfig,

    /// An identifier the system uses to group related boarding passes or event tickets.
    /// Wallet displays passes with the same [grouping_identifier](Pass::grouping_identifier), [pass_type_identifier](Pass::pass_type_identifier), and type as a group.
    /// Use this identifier to group passes that are tightly related, such as boarding passes for different connections on the same trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_identifier: Option<String>,

    /// Colors and other visual parts of the pass
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<VisualAppearance>,

    /// The text to display next to the logo on the pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_text: Option<String>,

    /// The date and time when the pass becomes relevant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_date: Option<String>,

    /// The date and time the pass expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,

    /// A URL to be passed to the associated app when launching it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "appLaunchURL")]
    pub app_launch_url: Option<String>,

    /// An array of App Store identifiers for apps associated with the pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_store_identifiers: Vec<i32>,

    /// Implement a web server to register, update, and unregister a pass on a device.
    /// See [Apple documentation](https://developer.apple.com/documentation/walletpasses/adding_a_web_service_to_update_passes)
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_service: Option<WebService>,

    /// Controls whether to show the Share button on the back of a pass.
    /// A value of true removes the button. The default value is false.
    /// This flag has no effect in earlier versions of iOS, nor does it prevent sharing the pass in some other way.
    #[serde(skip_serializing_if = "is_false")]
    pub sharing_prohibited: bool,

    /// Controls whether to display the strip image without a shine effect.
    /// The default value is true.
    #[serde(skip_serializing_if = "is_true")]
    pub suppress_strip_shine: bool,

    /// Indicates that the pass is void, such as a redeemed, one-time-use coupon.
    /// The default value is false.
    #[serde(skip_serializing_if = "is_false")]
    pub voided: bool,

    /// Barcode on a pass
    /// The system uses the first displayable barcode for the device.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub barcodes: Vec<Barcode>,

    // Array of Bluetooth Low Energy beacons the system uses to show a relevant pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub beacons: Vec<Beacon>,

    // An array of up to 10 geographic locations the system uses to show a relevant pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<Location>,

    /// The maximum distance, in meters, from a location in the [locations](Pass::locations) array at which the pass is relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<u32>,

    /// Near-field communication (NFC) payload the device passes to an Apple Pay terminal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc: Option<NFC>,

    /// Semantic tags
    // Metadata the system uses to offer a pass and suggest related actions.
    // For example, setting Don’t Disturb mode for the duration of a movie.
    #[serde(skip_serializing_if = "SemanticTags::is_empty")]
    pub semantics: SemanticTags,

    // TODO: PassTypes
    #[serde(flatten)]
    pub fields: PassType,
    // boarding pass
    // coupon
    // event ticket
    // generic

    // TODO: UserInfo
    // custom JSOM
}

/// Builder for pass (represents pass.json file)
pub struct PassBuilder {
    pass: Pass,
}

impl PassBuilder {
    /// Creates builder for `Pass`.
    pub fn new(config: PassConfig) -> Self {
        let pass = Pass {
            // setup required vars
            format_version: 1,
            config,
            // Setup default optional vars
            grouping_identifier: None,
            appearance: None,
            logo_text: None,
            relevant_date: None,
            expiration_date: None,
            app_launch_url: None,
            associated_store_identifiers: Vec::new(),
            web_service: None,
            sharing_prohibited: false,
            suppress_strip_shine: true,
            voided: false,
            barcodes: Vec::new(),
            beacons: Vec::new(),
            locations: Vec::new(),
            max_distance: None,
            nfc: None,
            semantics: Default::default(),
            fields: PassType::Generic {
                pass_fields: PassFields {
                    ..Default::default()
                },
            },
        };
        Self { pass }
    }

    /// Adding [grouping_identifier](Pass::grouping_identifier)
    pub fn grouping_identifier(mut self, field: String) -> PassBuilder {
        self.pass.grouping_identifier = Some(field);
        self
    }

    /// Adding [appearance](Pass::appearance).
    pub fn appearance(mut self, field: VisualAppearance) -> PassBuilder {
        self.pass.appearance = Some(field);
        self
    }

    /// Adding [logo_text](Pass::logo_text)
    pub fn logo_text(mut self, field: String) -> PassBuilder {
        self.pass.logo_text = Some(field);
        self
    }

    /// Adding [relevant_date](Pass::relevant_date)
    pub fn relevant_date(mut self, field: String) -> PassBuilder {
        self.pass.relevant_date = Some(field);
        self
    }

    /// Adding [expiration_date](Pass::expiration_date)
    pub fn expiration_date(mut self, field: String) -> PassBuilder {
        self.pass.expiration_date = Some(field);
        self
    }

    /// Adding [app_launch_url](Pass::app_launch_url)
    pub fn app_launch_url(mut self, field: String) -> PassBuilder {
        self.pass.app_launch_url = Some(field);
        self
    }

    /// Adding [associated_store_identifiers](Pass::associated_store_identifiers)
    pub fn add_associated_store_identifier(mut self, id: i32) -> PassBuilder {
        self.pass.associated_store_identifiers.push(id);
        self
    }

    /// Adding [web_service_url](Pass::web_service_url)
    pub fn web_service(mut self, web_service: WebService) -> PassBuilder {
        self.pass.web_service = Some(web_service);
        self
    }

    /// Adding [sharing_prohibited](Pass::sharing_prohibited)
    pub fn set_sharing_prohibited(mut self, field: bool) -> PassBuilder {
        self.pass.sharing_prohibited = field;
        self
    }

    /// Adding [suppress_strip_shine](Pass::suppress_strip_shine)
    pub fn set_suppress_strip_shine(mut self, field: bool) -> PassBuilder {
        self.pass.suppress_strip_shine = field;
        self
    }

    /// Adding [voided](Pass::voided)
    pub fn voided(mut self, field: bool) -> PassBuilder {
        self.pass.voided = field;
        self
    }

    /// Adding [Barcode] to [barcodes](Pass::barcodes)
    pub fn add_barcode(mut self, barcode: Barcode) -> PassBuilder {
        self.pass.barcodes.push(barcode);
        self
    }

    /// Adding [Beacon] to [beacons](Pass::beacons)
    pub fn add_beacon(mut self, beacon: Beacon) -> PassBuilder {
        self.pass.beacons.push(beacon);
        self
    }

    /// Adding [Location] to [locations](Pass::locations)
    pub fn add_location(mut self, location: Location) -> PassBuilder {
        assert!(
            self.pass.locations.len() < 10,
            "Reached limit for geographic locations (maximum - 10)"
        );
        self.pass.locations.push(location);
        self
    }

    /// Adding [max_distance](Pass::max_distance)
    pub fn max_distance(mut self, field: u32) -> PassBuilder {
        self.pass.max_distance = Some(field);
        self
    }

    /// Adding [nfc](Pass::nfc)
    pub fn nfc(mut self, field: NFC) -> PassBuilder {
        self.pass.nfc = Some(field);
        self
    }

    /// Adding [semantics](Pass::semantics)
    pub fn semantics(mut self, field: SemanticTags) -> PassBuilder {
        self.pass.semantics = field;
        self
    }

    /// Adding [fields](Pass::fields)
    pub fn fields(mut self, field: PassType) -> PassBuilder {
        self.pass.fields = field;
        self
    }

    /// Makes `Pass`.
    pub fn build(self) -> Pass {
        self.pass
    }
}

#[cfg(test)]
mod tests {
    use tests::{
        pass_fields::{PassFieldContent, PassFieldContentOptions},
        semantic_tags::SemanticTagLocation,
        visual_appearance::Color,
    };

    use super::*;

    #[test]
    fn make_minimal_pass() {
        let pass = PassBuilder::new(PassConfig {
            organization_name: String::from("Apple inc."),
            description: String::from("Example pass"),
            pass_type_identifier: String::from("com.example.pass"),
            team_identifier: String::from("AA00AA0A0A"),
            serial_number: String::from("ABCDEFG1234567890"),
        })
        .build();

        let json = serde_json::to_string_pretty(&pass).unwrap();

        println!("{}", serde_json::to_string_pretty(&pass).unwrap());

        let json_expected = r#"{
  "formatVersion": 1,
  "organizationName": "Apple inc.",
  "description": "Example pass",
  "passTypeIdentifier": "com.example.pass",
  "teamIdentifier": "AA00AA0A0A",
  "serialNumber": "ABCDEFG1234567890",
  "generic": {
    "auxiliaryFields": [],
    "backFields": [],
    "headerFields": [],
    "primaryFields": [],
    "secondaryFields": []
  }
}"#;

        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_pass() {
        let pass = PassBuilder::new(PassConfig {
            organization_name: String::from("Apple inc."),
            description: String::from("Example pass"),
            pass_type_identifier: String::from("com.example.pass"),
            team_identifier: String::from("AA00AA0A0A"),
            serial_number: String::from("ABCDEFG1234567890"),
        })
        .grouping_identifier(String::from("com.example.pass.app"))
        .appearance(VisualAppearance {
            label_color: None,
            foreground_color: Color::new(250, 10, 10),
            background_color: Color::white(),
        })
        .logo_text(String::from("Test pass"))
        .relevant_date(String::from("2024-02-07T00:00"))
        .expiration_date(String::from("2024-02-08T00:00"))
        .app_launch_url(String::from("testapp:param?index=1"))
        .add_associated_store_identifier(100)
        .web_service(WebService {
            authentication_token: String::from("abcdefg01234567890abcdefg"),
            web_service_url: String::from("https://example.com/passes/"),
        })
        .set_sharing_prohibited(false)
        .set_suppress_strip_shine(false)
        .voided(false)
        .add_barcode(Barcode {
            message: String::from("Hello world!"),
            format: barcode::BarcodeFormat::QR,
            alt_text: Some(String::from("test by test")),
            ..Default::default()
        })
        .add_beacon(Beacon {
            proximity_uuid: String::from("e286373b-15b5-4f4e-bf91-e9e64787724a"),
            major: Some(2),
            minor: Some(150),
            relevant_text: Some(String::from("The simple beacon")),
        })
        .add_location(Location {
            latitude: 37.334606,
            longitude: -122.009102,
            relevant_text: Some(String::from("Apple Park, Cupertino, CA, USA")),
            ..Default::default()
        })
        .max_distance(1000)
        .nfc(NFC {
            encryption_public_key: String::from("ABCDEFG_0011223344556677889900"),
            message: String::from("test message"),
            ..Default::default()
        })
        .semantics(SemanticTags {
            airline_code: String::from("EX123").into(),
            departure_location: SemanticTagLocation {
                latitude: 43.3948533,
                longitude: 132.1451673,
            }
            .into(),
            ..Default::default()
        })
        .fields(
            PassType::BoardingPass {
                pass_fields: PassFields {
                    ..Default::default()
                },
                transit_type: String::from("PKTransitTypeAir"),
            }
            .add_header_field(PassFieldContent::new(
                "serial",
                "1122",
                PassFieldContentOptions {
                    label: String::from("SERIAL").into(),
                    ..Default::default()
                },
            ))
            .add_header_field(PassFieldContent::new(
                "number",
                "0011223344",
                PassFieldContentOptions {
                    label: String::from("NUMBER").into(),
                    text_alignment: String::from("PKTextAlignmentRight").into(),
                    ..Default::default()
                },
            ))
            .add_primary_field(PassFieldContent::new(
                "from",
                "UHWW",
                PassFieldContentOptions {
                    label: String::from("FROM").into(),
                    text_alignment: String::from("PKTextAlignmentLeft").into(),
                    ..Default::default()
                },
            ))
            .add_primary_field(PassFieldContent::new(
                "to",
                "RKSI",
                PassFieldContentOptions {
                    label: String::from("TO").into(),
                    text_alignment: String::from("PKTextAlignmentRight").into(),
                    ..Default::default()
                },
            ))
            .add_auxiliary_field(PassFieldContent::new(
                "date_departure",
                "20.02.2024",
                PassFieldContentOptions {
                    label: String::from("Departure date").into(),
                    ..Default::default()
                },
            )),
        )
        .build();

        let json = serde_json::to_string_pretty(&pass).unwrap();

        println!("{}", serde_json::to_string_pretty(&pass).unwrap());

        let json_expected = r#"{
  "formatVersion": 1,
  "organizationName": "Apple inc.",
  "description": "Example pass",
  "passTypeIdentifier": "com.example.pass",
  "teamIdentifier": "AA00AA0A0A",
  "serialNumber": "ABCDEFG1234567890",
  "groupingIdentifier": "com.example.pass.app",
  "foregroundColor": "rgb(250, 10, 10)",
  "backgroundColor": "rgb(255, 255, 255)",
  "logoText": "Test pass",
  "relevantDate": "2024-02-07T00:00",
  "expirationDate": "2024-02-08T00:00",
  "appLaunchURL": "testapp:param?index=1",
  "associatedStoreIdentifiers": [
    100
  ],
  "authenticationToken": "abcdefg01234567890abcdefg",
  "webServiceURL": "https://example.com/passes/",
  "suppressStripShine": false,
  "barcodes": [
    {
      "message": "Hello world!",
      "format": "PKBarcodeFormatQR",
      "altText": "test by test",
      "messageEncoding": "iso-8859-1"
    }
  ],
  "beacons": [
    {
      "proximityUUID": "e286373b-15b5-4f4e-bf91-e9e64787724a",
      "major": 2,
      "minor": 150,
      "relevantText": "The simple beacon"
    }
  ],
  "locations": [
    {
      "latitude": 37.334606,
      "longitude": -122.009102,
      "relevantText": "Apple Park, Cupertino, CA, USA"
    }
  ],
  "maxDistance": 1000,
  "nfc": {
    "encryptionPublicKey": "ABCDEFG_0011223344556677889900",
    "message": "test message",
    "requiresAuthentication": false
  },
  "semantics": {
    "airlineCode": "EX123",
    "departureLocation": {
      "latitude": 43.3948533,
      "longitude": 132.1451673
    }
  },
  "boardingPass": {
    "auxiliaryFields": [
      {
        "key": "date_departure",
        "value": "20.02.2024",
        "label": "Departure date"
      }
    ],
    "backFields": [],
    "headerFields": [
      {
        "key": "serial",
        "value": "1122",
        "label": "SERIAL"
      },
      {
        "key": "number",
        "value": "0011223344",
        "label": "NUMBER",
        "textAlignment": "PKTextAlignmentRight"
      }
    ],
    "primaryFields": [
      {
        "key": "from",
        "value": "UHWW",
        "label": "FROM",
        "textAlignment": "PKTextAlignmentLeft"
      },
      {
        "key": "to",
        "value": "RKSI",
        "label": "TO",
        "textAlignment": "PKTextAlignmentRight"
      }
    ],
    "secondaryFields": [],
    "transit_type": "PKTransitTypeAir"
  }
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
