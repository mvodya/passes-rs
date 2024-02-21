use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

/// Serialization to custom date format
pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.unwrap().to_rfc3339());
    serializer.serialize_str(&s)
}

/// Deserialization from custom date format
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        // RFC3339
        Ok(Some(dt.with_timezone(&Utc)))
    } else if let Ok(dt) = DateTime::parse_from_rfc2822(&s) {
        // RFC2822
        Ok(Some(dt.with_timezone(&Utc)))
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
        // Custom naive format
        Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
    } else {
        Err(serde::de::Error::custom("Invalid DateTime format"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{prelude::*, DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct DateTest {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "super")]
        pub date: Option<DateTime<Utc>>,
    }

    #[test]
    fn serialize_check() {
        let date_struct = DateTest {
            date: Some(Utc.with_ymd_and_hms(2024, 02, 07, 10, 15, 0).unwrap()),
        };
        let json = serde_json::to_string_pretty(&date_struct).unwrap();
        println!("{}", json);
        let json_expected = r#"{
  "date": "2024-02-07T10:15:00+00:00"
}"#;
        assert_eq!(json_expected, json);
    }

    #[test]
    fn rfc3339_deserialize_check() {
        let json = r#"{
  "date": "2024-02-07T10:15:00+00:00"
}"#;
        let date_struct: DateTest = serde_json::from_str(json).unwrap();
        let date_expected = Utc.with_ymd_and_hms(2024, 02, 07, 10, 15, 0).unwrap();
        assert_eq!(date_expected, date_struct.date.unwrap());
    }

    #[test]
    fn rfc2822_deserialize_check() {
        let json = r#"{
  "date": "Wed, 07 Feb 2024 10:15:00 GMT"
}"#;
        let date_struct: DateTest = serde_json::from_str(json).unwrap();
        let date_expected = Utc.with_ymd_and_hms(2024, 02, 07, 10, 15, 0).unwrap();
        assert_eq!(date_expected, date_struct.date.unwrap());
    }

    #[test]
    fn no_tz_deserialize_check() {
        let json = r#"{
      "date": "2024-02-07T10:15:00"
    }"#;
        let date_struct: DateTest = serde_json::from_str(json).unwrap();
        let date_expected = Utc.with_ymd_and_hms(2024, 02, 07, 10, 15, 0).unwrap();
        assert_eq!(date_expected, date_struct.date.unwrap());
    }
}
