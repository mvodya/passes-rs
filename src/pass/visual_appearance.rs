use serde::{Deserialize, Serialize};

/// Visual appearance of a pass
#[derive(Serialize, Deserialize, Debug)]
pub struct VisualAppearance {
    /// A color for the label text of the pass.
    /// If you don’t provide a value, the system determines the label color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,

    /// A foreground color for the pass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,

    /// A background color for the pass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

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
        Self { r: r, g: g, b: b }
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
            label_color: None,
            foreground_color: None,
            background_color: None,
            logo_text: None,
        };

        let json = serde_json::to_string_pretty(&appearance).unwrap();

        println!("{}", json);

        let json_expected = r#"{}"#;

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
