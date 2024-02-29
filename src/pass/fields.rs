use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};

use super::semantic_tags::SemanticTags;

/// Represents the groups of fields that display information on the front and back of a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    /// Represents the fields that display additional information on the front of a pass.
    pub auxiliary_fields: Vec<Content>,

    /// Represents the fields that display information on the back of a pass.
    pub back_fields: Vec<Content>,

    /// Represents the fields that display information at the top of a pass.
    pub header_fields: Vec<Content>,

    /// Represents the fields that display the most important information on a pass.
    pub primary_fields: Vec<Content>,

    /// Represents the fields that display supporting information on the front of a pass.
    pub secondary_fields: Vec<Content>,
}

impl Default for Fields {
    /// Creates an empty `Fields`.
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
pub struct Content {
    /// (Required) A unique key that identifies a field in the pass; for example, “departure-gate”.
    pub key: String,

    /// (Required) The value to use for the field; for example, 42. A date or time value must include a time zone.
    pub value: String,

    /// All optionals
    #[serde(flatten)]
    pub options: ContentOptions,
}

impl Content {
    /// Creates `FieldContent`.
    pub fn new(key: &str, value: &str, options: ContentOptions) -> Self {
        Self {
            key: String::from(key),
            value: String::from(value),
            options: options,
        }
    }
}

/// Represents options for `FieldContent`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentOptions {
    /// The value of the field, including HTML markup for links.
    /// The only supported tag is the `<a>` tag and its href attribute.
    /// The value of this key overrides that of the value key.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_value: Option<String>,

    /// A format string for the alert text to display when the pass is updated.
    /// The format string must contain the escape %@, which is replaced with the field’s new value.
    /// For example, “Gate changed to %@”.
    ///
    /// You must provide a value for the system to show a change notification.
    ///
    /// This field isn’t used for watchOS.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,

    /// The currency code to use for the value of the field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    /// The data detectors to apply to the value of a field on the back of the pass.
    /// The default is to apply all data detectors. To use no data detectors, specify an empty array.
    ///
    /// You don’t use data detectors for fields on the front of the pass.
    ///
    /// This field isn’t used for watchOS.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_detector_types: Option<DetectorType>,

    /// The style of the date to display in the field.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_style: Option<DateStyle>,

    /// A Boolean value that controls the time zone for the time and date to display in the field.
    /// The default value is false, which displays the time and date using the current device’s time zone.
    /// Otherwise, the time and date appear in the time zone associated with the date and time of value.
    ///
    /// This key doesn’t affect the pass relevance calculation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_time_zone: Option<bool>,

    /// A Boolean value that controls whether the date appears as a relative date.
    /// The default value is false, which displays the date as an absolute date.
    ///
    /// This key doesn’t affect the pass relevance calculation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relative: Option<bool>,

    /// The text for a field label.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The style of the number to display in the field.
    /// Formatter styles have the same meaning as the formats with corresponding names in NumberFormatter.Style.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_style: Option<NumberStyle>,

    /// The alignment for the content of a field. The default is natural alignment, which aligns the text based on its script direction.
    /// This key is invalid for primary and back fields.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<TextAlignment>,

    /// The style of the time displayed in the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_style: Option<DateStyle>,

    /// Semantic tags
    // Metadata the system uses to offer a pass and suggest related actions.
    #[serde(default)]
    #[serde(skip_serializing_if = "SemanticTags::is_empty")]
    pub semantics: SemanticTags,
}

impl Default for ContentOptions {
    /// Creates an empty `FieldContentOptions`.
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

/// The style of the date to display in the field.
#[derive(Serialize, Deserialize, Debug)]
pub enum DateStyle {
    #[serde(rename = "PKDateStyleNone")]
    None,
    #[serde(rename = "PKDateStyleShort")]
    Short,
    #[serde(rename = "PKDateStyleMedium")]
    Medium,
    #[serde(rename = "PKDateStyleLong")]
    Long,
    #[serde(rename = "PKDateStyleFull")]
    Full,
}

/// The style of the number to display in the field.
#[derive(Serialize, Deserialize, Debug)]
pub enum NumberStyle {
    #[serde(rename = "PKNumberStyleDecimal")]
    Decimal,
    #[serde(rename = "PKNumberStylePercent")]
    Percent,
    #[serde(rename = "PKNumberStyleScientific")]
    Scientific,
    #[serde(rename = "PKNumberStyleSpellOut")]
    SpellOut,
}

/// The alignment for the content of a field.
#[derive(Serialize, Deserialize, Debug)]
pub enum TextAlignment {
    #[serde(rename = "PKTextAlignmentLeft")]
    Left,
    #[serde(rename = "PKTextAlignmentCenter")]
    Center,
    #[serde(rename = "PKTextAlignmentRight")]
    Right,
    #[serde(rename = "PKTextAlignmentNatural")]
    Natural,
}

/// Groups of fields that display information on the front and back of a pass.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    /// Represents the groups of fields that display the information for a boarding pass.
    BoardingPass {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: Fields,

        /// (Required) The type of transit for a boarding pass. This key is invalid for other types of passes.
        #[serde(rename = "transitType")]
        transit_type: TransitType,
    },
    /// Represents the groups of fields that display the information for a coupon.
    Coupon {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: Fields,
    },
    /// Represents the groups of fields that display the information for an event ticket.
    EventTicket {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: Fields,
    },
    /// Represents the groups of fields that display the information for a generic pass.
    Generic {
        /// Groups of fields that display information on the front and back of a pass.
        #[serde(flatten)]
        pass_fields: Fields,
    },
}

/// The type of transit for a boarding pass.
#[derive(Serialize, Deserialize, Debug)]
pub enum TransitType {
    #[serde(rename = "PKTransitTypeAir")]
    Air,
    #[serde(rename = "PKTransitTypeBoat")]
    Boat,
    #[serde(rename = "PKTransitTypeBus")]
    Bus,
    #[serde(rename = "PKTransitTypeGeneric")]
    Generic,
    #[serde(rename = "PKTransitTypeTrain")]
    Train,
}

impl Type {
    /// Add field that display additional information on the front of a pass.
    pub fn add_auxiliary_field(mut self, field: Content) -> Self {
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
    pub fn add_back_field(mut self, field: Content) -> Self {
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
    pub fn add_header_field(mut self, field: Content) -> Self {
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
    pub fn add_primary_field(mut self, field: Content) -> Self {
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
    pub fn add_secondary_field(mut self, field: Content) -> Self {
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
        // Serialization test
        let pass = Type::Generic {
            pass_fields: Fields {
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

        // Deserialization test
        let pass: Type = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&pass).unwrap();
        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_boarding_pass() {
        // Serialization test
        let boarding_pass = Type::BoardingPass {
            pass_fields: Fields {
                ..Default::default()
            },
            transit_type: TransitType::Air,
        }
        .add_primary_field(Content::new("title", "Airplane Ticket", Default::default()))
        .add_primary_field(Content::new(
            "seat",
            "12",
            ContentOptions {
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
        .add_header_field(Content::new("company", "DAL", Default::default()))
        .add_header_field(Content::new(
            "company_sub",
            "Dodo Air Lines",
            Default::default(),
        ))
        .add_secondary_field(Content::new(
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
    "transitType": "PKTransitTypeAir"
  }
}"#;
        assert_eq!(json_expected, json);

        // Deserialization test
        let boarding_pass: Type = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&boarding_pass).unwrap();
        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_event_ticket() {
        // Serialization test
        let event_ticket = Type::EventTicket {
            pass_fields: Fields {
                ..Default::default()
            },
        }
        .add_primary_field(Content::new(
            "title",
            "Super Ticket",
            ContentOptions {
                label: String::from("NAME").into(),
                ..Default::default()
            },
        ))
        .add_primary_field(Content::new("seat", "12", Default::default()))
        .add_header_field(Content::new("event_title", "KKK", Default::default()))
        .add_header_field(Content::new("some", "123", Default::default()))
        .add_secondary_field(Content::new(
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

        // Deserialization test
        let event_ticket: Type = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&event_ticket).unwrap();
        assert_eq!(json_expected, json);
    }
}
