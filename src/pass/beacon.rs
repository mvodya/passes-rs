use serde::{Deserialize, Serialize};

/// Represents the identify of a Bluetooth Low Energy beacon the system uses to show a relevant pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Beacon {
    /// (Required) The unique identifier of a Bluetooth Low Energy location beacon.
    #[serde(rename = "proximityUUID")]
    pub proximity_uuid: String,

    /// The major identifier of a Bluetooth Low Energy location beacon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<u16>,

    /// The minor identifier of a Bluetooth Low Energy location beacon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<u16>,

    /// The text to display on the lock screen when the pass is relevant.
    /// For example, a description of a nearby location, such as “Store nearby on 1st and Main”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

impl Default for Beacon {
    /// Creates an empty `Beacon`.
    fn default() -> Self {
        Self {
            proximity_uuid: String::from("00000000-0000-0000-0000-000000000000"),
            major: None,
            minor: None,
            relevant_text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_beacon() {
        let beacon = Beacon {
            proximity_uuid: String::from("e286373b-15b5-4f4e-bf91-e9e64787724a"),
            major: Some(2),
            minor: Some(150),
            relevant_text: Some(String::from("The simple beacon")),
        };

        let json = serde_json::to_string_pretty(&beacon).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "proximityUUID": "e286373b-15b5-4f4e-bf91-e9e64787724a",
  "major": 2,
  "minor": 150,
  "relevantText": "The simple beacon"
}"#;

        assert_eq!(json_expected, json);
    }
}
