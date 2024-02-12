use serde::{Deserialize, Serialize};

/// Machine-readable metadata the system uses to offer a pass and suggest related actions.
/// https://developer.apple.com/documentation/walletpasses/semantictags
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTags {
    /// The IATA airline code, such as “EX” for flightCode “EX123”. Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airline_code: Option<String>,

    /// An array of the Apple Music persistent ID for each artist performing at the event, in decreasing order of significance.
    /// Use this key for any type of event ticket.
    #[serde(rename = "artistIDs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artist_ids: Vec<String>,

    /// The unique abbreviation of the away team’s name. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_abbreviation: Option<String>,

    /// The home location of the away team. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_location: Option<String>,

    /// The name of the away team. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_name: Option<String>,

    /// The current balance redeemable with the pass. Use this key only for a store card pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<SemanticTagBalance>,

    /// A group number for boarding. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_group: Option<String>,

    /// A sequence number for boarding. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_sequence_number: Option<String>,

    /// The number of the passenger car.
    /// A train car is also called a carriage, wagon, coach, or bogie in some countries.
    /// Use this key only for a train or other rail boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_number: Option<String>,

    /// A booking or reservation confirmation number. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,

    /// The updated date and time of arrival, if different from the originally scheduled date and time.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_arrival_date: Option<String>,

    /// The updated date and time of boarding, if different from the originally scheduled date and time.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_boarding_date: Option<String>,

    /// The updated departure date and time, if different from the originally scheduled date and time.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_departure_date: Option<String>,

    /// The IATA airport code for the departure airport, such as “MPM” or “LHR”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,
}

/// Represents an amount of money and type of currency.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagBalance {
    /// The amount of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// The currency code for amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl Default for SemanticTags {
    /// Creates an empty `SemanticTags`.
    fn default() -> Self {
        Self {
            airline_code: None,
            artist_ids: Vec::new(),
            away_team_abbreviation: None,
            away_team_location: None,
            away_team_name: None,
            balance: None,
            boarding_group: None,
            boarding_sequence_number: None,
            car_number: None,
            confirmation_number: None,
            current_arrival_date: None,
            current_boarding_date: None,
            current_departure_date: None,
            departure_airport_code: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn make_semantic_tags() {
//         let mut tags = SemanticTags {
//             airline_code: Some(String::from("EX123")),
//             artist_ids: vec![
//                 String::from("100"),
//                 String::from("200"),
//                 String::from("300"),
//             ],
//             away_team_abbreviation: Some(String::from("ABC")),
//             away_team_location: String::from("Michigan").into(),
//             away_team_name: Some(String::from("Bebras")),
//             ..Default::default()
//         };

//         let json = serde_json::to_string_pretty(&tags).unwrap();

//         println!("{}", json);

//         let json_expected = r#"{
// }"#;

//         assert_eq!(json_expected, json);
//     }

    // #[test]
    // fn semantic_tag_airplane_code() {
    //     let tag = SemanticTag::AirlineCode(String::from("EX123"));
    // }
}
