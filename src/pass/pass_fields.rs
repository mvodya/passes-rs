use serde::{Deserialize, Serialize};

type PassFieldContent = String;

/// Represents the groups of fields that display information on the front and back of a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassFields {
    /// Represents the fields that display additional information on the front of a pass.
    pub auxiliary_fields: Vec<PassFieldContent>,

    /// Represents the fields that display information on the back of a pass.
    pub back_fields: Vec<PassFieldContent>,

    /// Represents the fields that display information at the top of a pass.
    pub header_fields: Vec<PassFieldContent>,

    /// Represents the fields that display the most important information on a pass.
    pub primary_fields: Vec<PassFieldContent>,

    /// Represents the fields that display supporting information on the front of a pass.
    pub secondary_fields: Vec<PassFieldContent>,
}

impl Default for PassFields {
    /// Creates an empty `PassFields`.
    fn default() -> Self {
        Self {
            auxiliary_fields: Vec::new(),
            back_fields: Vec::new(),
            header_fields: Vec::new(),
            primary_fields: Vec::new(),
            secondary_fields: Vec::new(),
        }
    }
}

/// Groups of fields that display information on the front and back of a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PassType {
    /// Represents the groups of fields that display the information for a boarding pass.
    BoardingPass {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: PassFields,

        /// (Required) The type of transit for a boarding pass. This key is invalid for other types of passes.
        /// Possible Values: PKTransitTypeAir, PKTransitTypeBoat, PKTransitTypeBus, PKTransitTypeGeneric, PKTransitTypeTrain
        transit_type: String,
    },
    /// Represents the groups of fields that display the information for a coupon.
    Coupon {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: PassFields,
    },
    /// Represents the groups of fields that display the information for an event ticket.
    EventTicket {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: PassFields,
    },
    /// Represents the groups of fields that display the information for a generic pass.
    Generic {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: PassFields,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_boarding_pass() {
        let boarding_pass = PassType::BoardingPass {
            pass_fields: PassFields {
                ..Default::default()
            },
            transit_type: String::from("test"),
        };

        let json = serde_json::to_string_pretty(&boarding_pass).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "boardingPass": {
    "auxiliaryFields": [],
    "backFields": [],
    "headerFields": [],
    "primaryFields": [],
    "secondaryFields": [],
    "transit_type": "test"
  }
}"#;
        assert_eq!(json_expected, json);
    }
}
