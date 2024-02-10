use regex::Regex;
use serde::{Deserialize, Serialize};

/// Visual appearance of a pass
#[derive(Serialize, Deserialize, Debug)]
pub struct VisualAppearance {
    /// A color for the label text of the pass.
    /// If you don’t provide a value, the system determines the label color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

    /// A foreground color for the pass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<Color>,

    /// A background color for the pass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,

    /// The text to display next to the logo on the pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_text: Option<String>,
}

impl Default for VisualAppearance {
    /// Creates an empty `VisualAppearance` for `Pass`.
    fn default() -> Self {
        Self {
            label_color: None,
            foreground_color: None,
            background_color: None,
            logo_text: None,
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
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a white `Color`.
    fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
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

        let re = Regex::new(r"rgb\((?P<r>\d+),\s*(?P<g>\d+),\s*(?P<b>\d+)\)").unwrap();
        let captures = re.captures(&str).unwrap();

        let r = captures["r"].parse::<u8>().unwrap();
        let g = captures["g"].parse::<u8>().unwrap();
        let b = captures["b"].parse::<u8>().unwrap();

        Ok(Self { r, g, b })
    }
}

impl Default for Color {
    /// Creates a black `Color`.
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_appearance() {
        let appearance = VisualAppearance {
            label_color: Some(Color::new(255, 100, 100)),
            foreground_color: Some(Color::new(255, 100, 100)),
            background_color: Some(Color::new(255, 100, 100)),
            logo_text: Some(String::from("Test Logo")),
        };

        let json = serde_json::to_string_pretty(&appearance).unwrap();

        println!("{}", json);

        let json_expected = r#"{
  "label_color": "rgb(255, 100, 100)",
  "foreground_color": "rgb(255, 100, 100)",
  "background_color": "rgb(255, 100, 100)",
  "logo_text": "Test Logo"
}"#;

        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_new_color() {
        let color = Color::new(100, 200, 240);

        println!("{:?}", color);

        assert_eq!(100, color.r);
        assert_eq!(200, color.g);
        assert_eq!(240, color.b);
    }

    #[test]
    fn make_black_color() {
        let color = Color::default();

        println!("{:?}", color);

        assert_eq!(0, color.r);
        assert_eq!(0, color.g);
        assert_eq!(0, color.b);
    }

    #[test]
    fn make_white_color() {
        let color = Color::white();

        println!("{:?}", color);

        assert_eq!(255, color.r);
        assert_eq!(255, color.g);
        assert_eq!(255, color.b);
    }
}
