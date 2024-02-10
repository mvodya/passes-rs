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

        let json_pass = serde_json::to_string_pretty(&appearance).unwrap();

        println!("{}", json_pass);

        let json_expected = r#"{}"#;

        assert_eq!(json_expected, json_pass);
    }
}