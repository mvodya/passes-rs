use serde::{Deserialize, Serialize};

/// Represents a location that the system uses to show a relevant pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// (Required) The latitude, in degrees, of the location.
    pub latitude: f64,

    /// (Required) The longitude, in degrees, of the location.
    pub longitude: f64,

    /// The altitude, in meters, of the location.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,

    /// The text to display on the lock screen when the pass is relevant.
    /// For example, a description of a nearby location, such as “Store nearby on 1st and Main”.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

impl Default for Location {
    /// Creates an empty `Location`.
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            altitude: None,
            relevant_text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_location() {
        // Serialization test
        let location = Location {
            latitude: 37.334606,
            longitude: -122.009102,
            relevant_text: Some(String::from("Apple Park, Cupertino, CA, USA")),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&location).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "latitude": 37.334606,
  "longitude": -122.009102,
  "relevantText": "Apple Park, Cupertino, CA, USA"
}"#;

        assert_eq!(json_expected, json);

        // Deserialization test
        let location: Location = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&location).unwrap();
        assert_eq!(json_expected, json);
    }
}
