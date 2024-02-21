use regex::Regex;
use serde::{de, Deserialize, Serialize};

/// Visual appearance of a pass
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VisualAppearance {
    /// A color for the label text of the pass.
    /// If you don’t provide a value, the system determines the label color.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

    /// A foreground color for the pass
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<Color>,

    /// A background color for the pass
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
}

impl Default for VisualAppearance {
    /// Creates an empty `VisualAppearance` for `Pass`.
    fn default() -> Self {
        Self {
            label_color: None,
            foreground_color: None,
            background_color: None,
        }
    }
}

/// Represents color - specified as a CSS-style RGB triple
#[derive(Debug)]
pub struct Color {
    /// Red
    r: u8,

    /// Green
    g: u8,

    /// Blue
    b: u8,
}

impl Color {
    /// Creates a new `Color`.
    pub fn new(r: u8, g: u8, b: u8) -> Option<Self> {
        Some(Self { r, g, b })
    }

    /// Creates a white `Color`.
    pub fn black() -> Option<Self> {
        Some(Self { r: 0, g: 0, b: 0 })
    }

    /// Creates a white `Color`.
    pub fn white() -> Option<Self> {
        Some(Self {
            r: 255,
            g: 255,
            b: 255,
        })
    }
}

impl Serialize for Color {
    /// Serialize `Color` to format `rgb(red, blue, green)`
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = format!("rgb({}, {}, {})", self.r, self.g, self.b);
        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for Color {
    /// Deserialize `Color` from format `rgb(red, blue, green)`
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        // Parse triple `rgb(red, blue, green)`
        let re = Regex::new(r"rgb\((?P<r>\d+),\s*(?P<g>\d+),\s*(?P<b>\d+)\)").unwrap();
        let captures = re.captures(&str);

        if let Some(captures) = captures {
            if let (Ok(r), Ok(g), Ok(b)) = (
                captures["r"].parse::<u8>(),
                captures["g"].parse::<u8>(),
                captures["b"].parse::<u8>(),
            ) {
                Ok(Self { r, g, b })
            } else {
                Err(de::Error::custom("Invalid color arguments"))
            }
        } else {
            Err(de::Error::custom("Invalid color format"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_appearance() {
        // Serialization test
        let appearance = VisualAppearance {
            label_color: Color::new(255, 100, 100),
            foreground_color: Color::new(255, 100, 100),
            background_color: Color::new(255, 100, 100),
        };

        let json = serde_json::to_string_pretty(&appearance).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "labelColor": "rgb(255, 100, 100)",
  "foregroundColor": "rgb(255, 100, 100)",
  "backgroundColor": "rgb(255, 100, 100)"
}"#;

        assert_eq!(json_expected, json);

        // Deserialization test
        let appearance: VisualAppearance = serde_json::from_str(json_expected).unwrap();
        let json = serde_json::to_string_pretty(&appearance).unwrap();
        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_custom_color() {
        let color = Color::new(100, 200, 240).unwrap();

        assert_eq!(100, color.r);
        assert_eq!(200, color.g);
        assert_eq!(240, color.b);
    }

    #[test]
    fn make_black_color() {
        let color = Color::black().unwrap();

        println!("{:?}", color);

        assert_eq!(0, color.r);
        assert_eq!(0, color.g);
        assert_eq!(0, color.b);
    }

    #[test]
    fn make_white_color() {
        let color = Color::white().unwrap();

        println!("{:?}", color);

        assert_eq!(255, color.r);
        assert_eq!(255, color.g);
        assert_eq!(255, color.b);
    }

    #[test]
    fn color_serialization() {
        let color = Color::new(12, 34, 56).unwrap();

        let json = serde_json::to_string_pretty(&color).unwrap();

        let json_expected = r#""rgb(12, 34, 56)""#;

        assert_eq!(json_expected, json);
    }

    #[test]
    fn color_deserialization() {
        let json = r#""rgb(12, 34, 56)""#;

        let color: Color = serde_json::from_str(json).unwrap();

        let expected_color = Color::new(12, 34, 56).unwrap();

        assert_eq!(expected_color.r, color.r);
        assert_eq!(expected_color.g, color.g);
        assert_eq!(expected_color.b, color.b);
    }

    #[test]
    #[should_panic(expected = "Invalid color format")]
    fn color_deserialization_error() {
        let json = r#""rgb(12, 34, abc)""#;

        let _: Color = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid color arguments")]
    fn color_incorrect_args_0() {
        let json = r#""rgb(12, 500, 0)""#;

        let _: Color = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid color arguments")]
    fn color_incorrect_args_1() {
        let json = r#""rgb(0, 256, 0)""#;

        let _: Color = serde_json::from_str(json).unwrap();
    }
}
