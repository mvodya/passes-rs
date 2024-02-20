use chrono::{DateTime, Utc};
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};

/// Machine-readable metadata the system uses to offer a pass and suggest related actions.
/// https://developer.apple.com/documentation/walletpasses/semantictags
#[derive(Serialize, Deserialize, Debug, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTags {
    /// The IATA airline code, such as “EX” for flightCode “EX123”. Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airline_code: Option<String>,

    /// An array of the Apple Music persistent ID for each artist performing at the event, in decreasing order of significance.
    /// Use this key for any type of event ticket.
    #[serde(rename = "artistIDs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[is_empty(if = "Vec::is_empty")]
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
    pub balance: Option<SemanticTagCurrencyAmount>,

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
    #[serde(with = "super::date_format")]
    pub current_arrival_date: Option<DateTime<Utc>>,

    /// The updated date and time of boarding, if different from the originally scheduled date and time.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub current_boarding_date: Option<DateTime<Utc>>,

    /// The updated departure date and time, if different from the originally scheduled date and time.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub current_departure_date: Option<DateTime<Utc>>,

    /// The IATA airport code for the departure airport, such as “MPM” or “LHR”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,

    /// The full name of the departure airport, such as “Maputo International Airport”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_name: Option<String>,

    /// The gate number or letters of the departure gate, such as “1A”.
    /// Do not include the word “Gate.”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_gate: Option<String>,

    /// An object that represents the geographic coordinates of the transit departure location, suitable for display on a map.
    /// If possible, use precise locations, which are more useful to travelers; for example, the specific location of an airport gate.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_location: Option<SemanticTagLocation>,

    /// A brief description of the departure location.
    /// For example, for a flight departing from an airport whose code is “LHR,” an appropriate description might be “London, Heathrow“.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_location_description: Option<String>,

    /// The name of the departure platform, such as “A”.
    /// Don’t include the word “Platform.” Use this key only for a train or other rail boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_platform: Option<String>,

    /// The name of the departure station, such as “1st Street Station”.
    /// Use this key only for a train or other rail boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_station_name: Option<String>,

    /// The name or letter of the departure terminal, such as “A”.
    /// Don’t include the word “Terminal.” Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_terminal: Option<String>,

    /// The IATA airport code for the destination airport, such as “MPM” or “LHR”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_airport_code: Option<String>,

    /// The full name of the destination airport, such as “London Heathrow”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_airport_name: Option<String>,

    /// The gate number or letter of the destination gate, such as “1A”.
    /// Don’t include the word “Gate”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_gate: Option<String>,

    /// An object that represents the geographic coordinates of the transit departure location, suitable for display on a map.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location: Option<SemanticTagLocation>,

    /// A brief description of the destination location.
    /// For example, for a flight arriving at an airport whose code is “MPM,” “Maputo“ might be an appropriate description.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location_description: Option<String>,

    /// The name of the destination platform, such as “A”.
    /// Don’t include the word “Platform”.
    /// Use this key only for a train or other rail boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_platform: Option<String>,

    /// The name of the destination station, such as “1st Street Station”.
    /// Use this key only for a train or other rail boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_station_name: Option<String>,

    /// The terminal name or letter of the destination terminal, such as “A”.
    /// Don’t include the word “Terminal”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_terminal: Option<String>,

    /// The duration of the event or transit journey, in seconds.
    /// Use this key for any type of boarding pass and any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// The date and time the event ends. Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub event_end_date: Option<DateTime<Utc>>,

    /// The full name of the event, such as the title of a movie.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    /// The date and time the event starts.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub event_start_date: Option<DateTime<Utc>>,

    /// The type of event. Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<SemanticEventType>,

    /// The IATA flight code, such as “EX123”. Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_code: Option<String>,

    /// The numeric portion of the IATA flight code, such as 123 for flightCode “EX123”.
    /// Use this key only for airline boarding passes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<u32>,

    /// The genre of the performance, such as “Classical”. Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// The unique abbreviation of the home team’s name. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_abbreviation: Option<String>,

    /// The home location of the home team. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_location: Option<String>,

    /// The name of the home team. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_name: Option<String>,

    /// The abbreviated league name for a sports event. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league_abbreviation: Option<String>,

    /// The unabbreviated league name for a sports event. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league_name: Option<String>,

    /// The name of a frequent flyer or loyalty program. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_program_name: Option<String>,

    /// The ticketed passenger’s frequent flyer or loyalty number. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_program_number: Option<String>,

    /// The originally scheduled date and time of arrival. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub original_arrival_date: Option<DateTime<Utc>>,

    /// The originally scheduled date and time of boarding. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub original_boarding_date: Option<DateTime<Utc>>,

    /// The originally scheduled date and time of departure. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "super::date_format")]
    pub original_departure_date: Option<DateTime<Utc>>,

    /// An object that represents the name of the passenger. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<SemanticTagPersonNameComponents>,

    /// An array of the full names of the performers and opening acts at the event, in decreasing order of significance.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[is_empty(if = "Vec::is_empty")]
    pub performer_names: Vec<String>,

    /// The priority status the ticketed passenger holds, such as “Gold” or “Silver”.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_status: Option<String>,

    /// An array of objects that represent the details for each seat at an event or on a transit journey.
    /// Use this key for any type of boarding pass or event ticket.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[is_empty(if = "Vec::is_empty")]
    pub seats: Vec<SemanticTagSeat>,

    /// The type of security screening for the ticketed passenger, such as “Priority”.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_screening: Option<String>,

    /// Determines whether the user’s device remains silent during an event or transit journey.
    /// The system may override the key and determine the length of the period of silence.
    /// Use this key for any type of boarding pass or event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_requested: Option<bool>,

    /// The commonly used name of the sport. Use this key only for a sports event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport_name: Option<String>,

    /// The total price for the pass. Use this key for any pass type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<SemanticTagCurrencyAmount>,

    /// The name of the transit company. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_provider: Option<String>,

    /// A brief description of the current boarding status for the vessel, such as “On Time” or “Delayed”.
    /// For delayed status, provide [current_boarding_date](SemanticTags::current_boarding_date), [current_departure_date](SemanticTags::current_arrival_date), and [current_arrival_date](SemanticTags::current_arrival_date) where available.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_status: Option<String>,

    /// A brief description that explains the reason for the current transitStatus, such as “Thunderstorms”.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_status_reason: Option<String>,

    /// The name of the vehicle to board, such as the name of a boat. Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_name: Option<String>,

    /// The identifier of the vehicle to board, such as the aircraft registration number or train number.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,

    /// A brief description of the type of vehicle to board, such as the model and manufacturer of a plane or the class of a boat.
    /// Use this key for any type of boarding pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,

    /// The full name of the entrance, such as “Gate A”, to use to gain access to the ticketed event.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_entrance: Option<String>,

    /// An object that represents the geographic coordinates of the venue.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_location: Option<SemanticTagLocation>,

    /// The full name of the venue. Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_name: Option<String>,

    /// The phone number for enquiries about the venue’s ticketed event.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_phone_number: Option<String>,

    /// The full name of the room where the ticketed event is to take place.
    /// Use this key for any type of event ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_room: Option<String>,

    /// An array of objects that represent the WiFi networks associated with the event; for example, the network name and password associated with a developer conference.
    /// Use this key for any type of pass.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[is_empty(if = "Vec::is_empty")]
    pub wifi_access: Vec<SemanticTagWifiNetwork>,
}

/// Represents an amount of money and type of currency.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagCurrencyAmount {
    /// The amount of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// The currency code for amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl Default for SemanticTagCurrencyAmount {
    /// Creates an empty `SemanticTagCurrencyAmount`.
    fn default() -> Self {
        Self {
            amount: None,
            currency_code: None,
        }
    }
}

/// Represents the coordinates of a location.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagLocation {
    /// (Required) The latitude, in degrees.
    pub latitude: f64,

    /// (Required) The longitude, in degrees.
    pub longitude: f64,
}

/// Represents the parts of a person’s name.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagPersonNameComponents {
    /// The person’s family name or last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    /// The person’s given name; also called the forename or first name in some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// The person’s middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// The prefix for the person’s name, such as “Dr”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// The suffix for the person’s name, such as “Junior”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// The person’s nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The phonetic representation of the person’s name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_representation: Option<String>,
}

impl Default for SemanticTagPersonNameComponents {
    /// Creates an empty `SemanticTagPersonNameComponents`.
    fn default() -> Self {
        Self {
            family_name: None,
            given_name: None,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            nickname: None,
            phonetic_representation: None,
        }
    }
}

/// Represents the identification of a seat for a transit journey or an event.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagSeat {
    /// A description of the seat, such as “A flat bed seat”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_description: Option<String>,

    /// The identifier code for the seat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_identifier: Option<String>,

    /// The number of the seat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_number: Option<String>,

    /// The row that contains the seat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_row: Option<String>,

    /// The section that contains the seat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_section: Option<String>,

    /// The type of seat, such as “Reserved seating”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<String>,
}

impl Default for SemanticTagSeat {
    /// Creates an empty `SemanticTagSeat`.
    fn default() -> Self {
        Self {
            seat_description: None,
            seat_identifier: None,
            seat_number: None,
            seat_row: None,
            seat_section: None,
            seat_type: None,
        }
    }
}

/// Contains information required to connect to a WiFi network.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTagWifiNetwork {
    /// (Required) The password for the WiFi network.
    pub password: f64,

    /// (Required) The name for the WiFi network.
    pub ssid: f64,
}

/// The type of event.
#[derive(Serialize, Deserialize, Debug)]
pub enum SemanticEventType {
    #[serde(rename = "PKEventTypeGeneric")]
    Generic,
    #[serde(rename = "PKEventTypeLivePerformance")]
    LivePerformance,
    #[serde(rename = "PKEventTypeMovie")]
    Movie,
    #[serde(rename = "PKEventTypeSports")]
    Sports,
    #[serde(rename = "PKEventTypeConference")]
    Conference,
    #[serde(rename = "PKEventTypeConvention")]
    Convention,
    #[serde(rename = "PKEventTypeWorkshop")]
    Workshop,
    #[serde(rename = "PKEventTypeSocialGathering")]
    SocialGathering,
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
            departure_airport_name: None,
            departure_gate: None,
            departure_location: None,
            departure_location_description: None,
            departure_platform: None,
            departure_station_name: None,
            departure_terminal: None,
            destination_airport_code: None,
            destination_airport_name: None,
            destination_gate: None,
            destination_location: None,
            destination_location_description: None,
            destination_platform: None,
            destination_station_name: None,
            destination_terminal: None,
            duration: None,
            event_end_date: None,
            event_name: None,
            event_start_date: None,
            event_type: None,
            flight_code: None,
            flight_number: None,
            genre: None,
            home_team_abbreviation: None,
            home_team_location: None,
            home_team_name: None,
            league_abbreviation: None,
            league_name: None,
            membership_program_name: None,
            membership_program_number: None,
            original_arrival_date: None,
            original_boarding_date: None,
            original_departure_date: None,
            passenger_name: None,
            performer_names: Vec::new(),
            priority_status: None,
            seats: Vec::new(),
            security_screening: None,
            silence_requested: None,
            sport_name: None,
            total_price: None,
            transit_provider: None,
            transit_status: None,
            transit_status_reason: None,
            vehicle_name: None,
            vehicle_number: None,
            vehicle_type: None,
            venue_entrance: None,
            venue_location: None,
            venue_name: None,
            venue_phone_number: None,
            venue_room: None,
            wifi_access: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::*;

    #[test]
    fn make_semantic_tags() {
        let tags = SemanticTags {
            airline_code: Some(String::from("EX123")),
            artist_ids: vec![
                String::from("100"),
                String::from("200"),
                String::from("300"),
            ],
            away_team_abbreviation: String::from("ABC").into(),
            away_team_location: String::from("Michigan").into(),
            away_team_name: String::from("Bebras").into(),
            balance: SemanticTagCurrencyAmount {
                amount: String::from("100").into(),
                currency_code: String::from("USD").into(),
            }
            .into(),
            boarding_group: String::from("ABC").into(),
            boarding_sequence_number: String::from("123").into(),
            car_number: String::from("01").into(),
            confirmation_number: String::from("1234").into(),
            current_arrival_date: Utc.with_ymd_and_hms(2024, 02, 10, 0, 0, 0).unwrap().into(),
            current_boarding_date: Utc.with_ymd_and_hms(2024, 02, 08, 0, 0, 0).unwrap().into(),
            current_departure_date: Utc.with_ymd_and_hms(2024, 02, 09, 0, 0, 0).unwrap().into(),
            departure_airport_code: String::from("VVO").into(),
            departure_airport_name: String::from("Vladivostok International Airport").into(),
            departure_gate: String::from("8").into(),
            departure_location: SemanticTagLocation {
                latitude: 43.3948533,
                longitude: 132.1451673,
            }
            .into(),
            departure_location_description: String::from(
                "An airport in the eastern part of Russia with flights to Asian countries.",
            )
            .into(),
            departure_platform: String::from("A").into(),
            departure_station_name: String::from("1st Street Station").into(),
            departure_terminal: String::from("A").into(),
            destination_airport_code: String::from("ICN").into(),
            destination_airport_name: String::from("Incheon International Airport").into(),
            destination_gate: String::from("1B").into(),
            destination_location: SemanticTagLocation {
                latitude: 37.4493342,
                longitude: 126.4487646,
            }
            .into(),
            destination_location_description: String::from("Seoul airport with various recreational areas: spa, golf course and ice skating rink.").into(),
            destination_platform: String::from("C").into(),
            destination_station_name: String::from("2st Street Station").into(),
            destination_terminal: String::from("B").into(),
            duration: Some(12345),
            event_end_date: Utc.with_ymd_and_hms(2024, 02, 10, 0, 0, 0).unwrap().into(),
            event_name: String::from("Super cool movie").into(),
            event_start_date: Utc.with_ymd_and_hms(2024, 02, 10, 8, 0, 0).unwrap().into(),
            event_type: SemanticEventType::Generic.into(),
            flight_code: String::from("EX123").into(),
            passenger_name: SemanticTagPersonNameComponents {
              nickname: String::from("gigachad").into(),
              family_name: String::from("Gigus").into(),
              given_name: String::from("Chadus").into(),
              ..Default::default()
            }.into(),
            seats: vec![SemanticTagSeat {
              seat_number: String::from("10").into(),
              seat_row: String::from("3").into(),
              ..Default::default()
            }, SemanticTagSeat {
              seat_number: String::from("11").into(),
              seat_row: String::from("3").into(),
              ..Default::default()
            }],
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&tags).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "airlineCode": "EX123",
  "artistIDs": [
    "100",
    "200",
    "300"
  ],
  "awayTeamAbbreviation": "ABC",
  "awayTeamLocation": "Michigan",
  "awayTeamName": "Bebras",
  "balance": {
    "amount": "100",
    "currencyCode": "USD"
  },
  "boardingGroup": "ABC",
  "boardingSequenceNumber": "123",
  "carNumber": "01",
  "confirmationNumber": "1234",
  "currentArrivalDate": "2024-02-10T00:00:00+00:00",
  "currentBoardingDate": "2024-02-08T00:00:00+00:00",
  "currentDepartureDate": "2024-02-09T00:00:00+00:00",
  "departureAirportCode": "VVO",
  "departureAirportName": "Vladivostok International Airport",
  "departureGate": "8",
  "departureLocation": {
    "latitude": 43.3948533,
    "longitude": 132.1451673
  },
  "departureLocationDescription": "An airport in the eastern part of Russia with flights to Asian countries.",
  "departurePlatform": "A",
  "departureStationName": "1st Street Station",
  "departureTerminal": "A",
  "destinationAirportCode": "ICN",
  "destinationAirportName": "Incheon International Airport",
  "destinationGate": "1B",
  "destinationLocation": {
    "latitude": 37.4493342,
    "longitude": 126.4487646
  },
  "destinationLocationDescription": "Seoul airport with various recreational areas: spa, golf course and ice skating rink.",
  "destinationPlatform": "C",
  "destinationStationName": "2st Street Station",
  "destinationTerminal": "B",
  "duration": 12345,
  "eventEndDate": "2024-02-10T00:00:00+00:00",
  "eventName": "Super cool movie",
  "eventStartDate": "2024-02-10T08:00:00+00:00",
  "eventType": "PKEventTypeGeneric",
  "flightCode": "EX123",
  "passengerName": {
    "familyName": "Gigus",
    "givenName": "Chadus",
    "nickname": "gigachad"
  },
  "seats": [
    {
      "seatNumber": "10",
      "seatRow": "3"
    },
    {
      "seatNumber": "11",
      "seatRow": "3"
    }
  ]
}"#;

        assert_eq!(json_expected, json);
    }
}
