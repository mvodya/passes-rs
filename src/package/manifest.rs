use openssl::sha::Sha1;
use serde::{ser::SerializeMap, Serialize};

/// Represents manifest.json file, contains SHA-256 of all .pkpass files.
/// Only serialization supported! (TODO?)
///
/// <https://developer.apple.com/documentation/walletorders/building_a_distributable_order_package>
pub struct Manifest {
    /// All manifest files with SHA-256
    items: Vec<Item>,
}

impl Manifest {
    /// Create empty manifest
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Add items & calculate SHA-256
    pub fn add_item(&mut self, path: &str, data: &[u8]) {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let checksum = hasher.finish();
        let item = Item {
            path: path.to_string(),
            checksum: hex::encode(&checksum),
        };
        self.items.push(item);
    }

    /// Add multiple items & calculate SHA-256
    pub fn add_items(&mut self, items: Vec<(&str, &[u8])>) {
        for (path, data) in items.iter() {
            self.add_item(path, data);
        }
    }

    /// Build JSON output for manifest (manifest.json)
    pub fn make_json(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    /// Remove all items from Manifest
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Manifest item
struct Item {
    /// Path of zip file
    path: String,

    /// SHA-256 hash
    checksum: String,
}

impl Serialize for Manifest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.items.len()))?;
        for item in self.items.iter() {
            map.serialize_entry(&item.path, &item.checksum)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_manifest() {
        let example_data = "hello world".as_bytes();
        let path = "pass.json";

        let mut manifest = Manifest::new();
        manifest.add_item(path, example_data);

        let json = manifest.make_json().unwrap();
        let json_expected = r#"{"pass.json":"2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"}"#;

        assert_eq!(json_expected, json);
    }

    #[test]
    fn make_manifest_multiple_items() {
        let mut items = Vec::new();
        items.push(("pass.json", "hello world".as_bytes()));
        items.push(("logo.png", "PNG DATA 1".as_bytes()));
        items.push(("background.png", "PNG DATA 2".as_bytes()));

        let mut manifest = Manifest::new();
        manifest.add_items(items);

        let json = manifest.make_json().unwrap();
        let json_expected = r#"{"pass.json":"2aae6c35c94fcfb415dbe95f408b9ce91ee846ed","logo.png":"e2507820ce1bd6d09669504e6a5536f7a3ccc94b","background.png":"05cc11980f5826d11c5c1292a4cd04ad11ddbf45"}"#;

        assert_eq!(json_expected, json);
    }
}
