use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};

use super::semantic_tags::SemanticTags;

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

/// Represents the information to display in a field on a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassFieldContent {
    /// (Required) A unique key that identifies a field in the pass; for example, “departure-gate”.
    pub key: String,

    /// (Required) The value to use for the field; for example, 42. A date or time value must include a time zone.
    pub value: String,

    /// All optionals
    #[serde(flatten)]
    pub options: PassFieldContentOptions,
}

impl PassFieldContent {
    /// Creates `PassFieldContent`.
    pub fn new(key: &str, value: &str, options: PassFieldContentOptions) -> Self {
        Self {
            key: String::from(key),
            value: String::from(value),
            options: options,
        }
    }
}

/// Represents options for `PassFieldContent`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassFieldContentOptions {
    /// The value of the field, including HTML markup for links.
    /// The only supported tag is the <a> tag and its href attribute.
    /// The value of this key overrides that of the value key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_value: Option<String>,

    /// A format string for the alert text to display when the pass is updated.
    /// The format string must contain the escape %@, which is replaced with the field’s new value.
    /// For example, “Gate changed to %@”.
    ///
    /// You must provide a value for the system to show a change notification.
    ///
    /// This field isn’t used for watchOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,

    /// The currency code to use for the value of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    /// The data detectors to apply to the value of a field on the back of the pass.
    /// The default is to apply all data detectors. To use no data detectors, specify an empty array.
    ///
    /// You don’t use data detectors for fields on the front of the pass.
    ///
    /// This field isn’t used for watchOS.
    ///
    /// Possible Values: PKDataDetectorTypePhoneNumber, PKDataDetectorTypeLink, PKDataDetectorTypeAddress, PKDataDetectorTypeCalendarEvent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_detector_types: Option<DetectorTypes>,

    /// The style of the date to display in the field.
    /// Possible Values: PKDateStyleNone, PKDateStyleShort, PKDateStyleMedium, PKDateStyleLong, PKDateStyleFull
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_style: Option<String>,

    /// A Boolean value that controls the time zone for the time and date to display in the field.
    /// The default value is false, which displays the time and date using the current device’s time zone.
    /// Otherwise, the time and date appear in the time zone associated with the date and time of value.
    ///
    /// This key doesn’t affect the pass relevance calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_time_zone: Option<bool>,

    /// A Boolean value that controls whether the date appears as a relative date.
    /// The default value is false, which displays the date as an absolute date.
    ///
    /// This key doesn’t affect the pass relevance calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relative: Option<bool>,

    /// The text for a field label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The style of the number to display in the field.
    /// Formatter styles have the same meaning as the formats with corresponding names in NumberFormatter.Style.
    /// Possible Values: PKNumberStyleDecimal, PKNumberStylePercent, PKNumberStyleScientific, PKNumberStyleSpellOut
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_style: Option<String>,

    /// The alignment for the content of a field. The default is natural alignment, which aligns the text based on its script direction.
    /// This key is invalid for primary and back fields.
    /// Possible Values: PKTextAlignmentLeft, PKTextAlignmentCenter, PKTextAlignmentRight, PKTextAlignmentNatural
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<String>,

    /// The style of the time displayed in the field.
    /// Possible Values: PKDateStyleNone, PKDateStyleShort, PKDateStyleMedium, PKDateStyleLong, PKDateStyleFull
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_style: Option<String>,

    /// Semantic tags
    // Metadata the system uses to offer a pass and suggest related actions.
    #[serde(skip_serializing_if = "SemanticTags::is_empty")]
    pub semantics: SemanticTags,
}

impl Default for PassFieldContentOptions {
    /// Creates an empty `PassFieldContentOptions`.
    fn default() -> Self {
        Self {
            attributed_value: None,
            change_message: None,
            currency_code: None,
            data_detector_types: None,
            date_style: None,
            ignores_time_zone: None,
            is_relative: None,
            label: None,
            number_style: None,
            text_alignment: None,
            time_style: None,
            semantics: Default::default(),
        }
    }
}

/// The data detectors to apply to the value of a field on the back of the pass.
#[derive(Serialize, Deserialize, Debug)]
pub enum DetectorType {
    #[serde(rename = "PKDataDetectorTypePhoneNumber")]
    PhoneNumber,
    #[serde(rename = "PKDataDetectorTypeLink")]
    Link,
    #[serde(rename = "PKDataDetectorTypeAddress")]
    Address,
    #[serde(rename = "PKDataDetectorTypeCalendarEvent")]
    CalendarEvent,
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

impl PassType {
    /// Add field that display additional information on the front of a pass.
    pub fn add_auxiliary_field(mut self, field: PassFieldContent) -> Self {
        match self {
            Self::BoardingPass {
                ref mut pass_fields,
                transit_type: _,
            } => pass_fields.auxiliary_fields.push(field),
            Self::Coupon {
                ref mut pass_fields,
            }
            | Self::EventTicket {
                ref mut pass_fields,
            }
            | Self::Generic {
                ref mut pass_fields,
            } => pass_fields.auxiliary_fields.push(field),
        }
        self
    }

    /// Add field that display information on the back of a pass.
    pub fn add_back_field(mut self, field: PassFieldContent) -> Self {
        match self {
            Self::BoardingPass {
                ref mut pass_fields,
                transit_type: _,
            } => pass_fields.back_fields.push(field),
            Self::Coupon {
                ref mut pass_fields,
            }
            | Self::EventTicket {
                ref mut pass_fields,
            }
            | Self::Generic {
                ref mut pass_fields,
            } => pass_fields.back_fields.push(field),
        }
        self
    }

    /// Add field that display information at the top of a pass.
    pub fn add_header_field(mut self, field: PassFieldContent) -> Self {
        match self {
            Self::BoardingPass {
                ref mut pass_fields,
                transit_type: _,
            } => pass_fields.header_fields.push(field),
            Self::Coupon {
                ref mut pass_fields,
            }
            | Self::EventTicket {
                ref mut pass_fields,
            }
            | Self::Generic {
                ref mut pass_fields,
            } => pass_fields.header_fields.push(field),
        }
        self
    }

    /// Add field that display the most important information on a pass.
    pub fn add_primary_field(mut self, field: PassFieldContent) -> Self {
        match self {
            Self::BoardingPass {
                ref mut pass_fields,
                transit_type: _,
            } => pass_fields.primary_fields.push(field),
            Self::Coupon {
                ref mut pass_fields,
            }
            | Self::EventTicket {
                ref mut pass_fields,
            }
            | Self::Generic {
                ref mut pass_fields,
            } => pass_fields.primary_fields.push(field),
        }
        self
    }

    /// Add field that display supporting information on the front of a pass.
    pub fn add_secondary_field(mut self, field: PassFieldContent) -> Self {
        match self {
            Self::BoardingPass {
                ref mut pass_fields,
                transit_type: _,
            } => pass_fields.secondary_fields.push(field),
            Self::Coupon {
                ref mut pass_fields,
            }
            | Self::EventTicket {
                ref mut pass_fields,
            }
            | Self::Generic {
                ref mut pass_fields,
            } => pass_fields.secondary_fields.push(field),
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::pass::semantic_tags::SemanticTagSeat;

    use super::*;

    #[test]
    fn make_pass() {
        let pass = PassType::Generic {
            pass_fields: PassFields {
                ..Default::default()
            },
        };

        let json = serde_json::to_string_pretty(&pass).unwrap();

        println!("{}", json);

        let json_expected = r#"{
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
    fn make_boarding_pass() {
        let boarding_pass = PassType::BoardingPass {
            pass_fields: PassFields {
                ..Default::default()
            },
            transit_type: String::from("PKTransitTypeAir"),
        }
        .add_primary_field(PassFieldContent::new(
            "title",
            "Airplane Ticket",
            Default::default(),
        ))
        .add_primary_field(PassFieldContent::new(
            "seat",
            "12",
            PassFieldContentOptions {
                semantics: SemanticTags {
                    seats: vec![SemanticTagSeat {
                        seat_number: String::from("12").into(),
                        seat_row: String::from("5A").into(),
                        seat_section: String::from("A").into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                label: String::from("Seat").into(),
                ..Default::default()
            },
        ))
        .add_header_field(PassFieldContent::new("company", "DAL", Default::default()))
        .add_header_field(PassFieldContent::new(
            "company_sub",
            "Dodo Air Lines",
            Default::default(),
        ))
        .add_secondary_field(PassFieldContent::new(
            "description",
            "Some information here",
            Default::default(),
        ));

        let json = serde_json::to_string_pretty(&boarding_pass).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "boardingPass": {
    "auxiliaryFields": [],
    "backFields": [],
    "headerFields": [
      {
        "key": "company",
        "value": "DAL"
      },
      {
        "key": "company_sub",
        "value": "Dodo Air Lines"
      }
    ],
    "primaryFields": [
      {
        "key": "title",
        "value": "Airplane Ticket"
      },
      {
        "key": "seat",
        "value": "12",
        "label": "Seat",
        "semantics": {
          "seats": [
            {
              "seatNumber": "12",
              "seatRow": "5A",
              "seatSection": "A"
            }
          ]
        }
      }
    ],
    "secondaryFields": [
      {
        "key": "description",
        "value": "Some information here"
      }
    ],
    "transit_type": "PKTransitTypeAir"
  }
}"#;
        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_event_ticket() {
        let event_ticket = PassType::EventTicket {
            pass_fields: PassFields {
                ..Default::default()
            },
        }
        .add_primary_field(PassFieldContent::new(
            "title",
            "Super Ticket",
            PassFieldContentOptions {
                label: String::from("NAME").into(),
                ..Default::default()
            },
        ))
        .add_primary_field(PassFieldContent::new("seat", "12", Default::default()))
        .add_header_field(PassFieldContent::new(
            "event_title",
            "KKK",
            Default::default(),
        ))
        .add_header_field(PassFieldContent::new("some", "123", Default::default()))
        .add_secondary_field(PassFieldContent::new(
            "description",
            "Some information here",
            Default::default(),
        ));

        let json = serde_json::to_string_pretty(&event_ticket).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "eventTicket": {
    "auxiliaryFields": [],
    "backFields": [],
    "headerFields": [
      {
        "key": "event_title",
        "value": "KKK"
      },
      {
        "key": "some",
        "value": "123"
      }
    ],
    "primaryFields": [
      {
        "key": "title",
        "value": "Super Ticket",
        "label": "NAME"
      },
      {
        "key": "seat",
        "value": "12"
      }
    ],
    "secondaryFields": [
      {
        "key": "description",
        "value": "Some information here"
      }
    ]
  }
}"#;
        assert_eq!(json_expected, json);
    }
}
