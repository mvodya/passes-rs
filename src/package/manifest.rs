/// Represents manifest.json file, contains SHA-256 of all .pkpass files.
/// Only serialization supported! (TODO?)
///
/// https://developer.apple.com/documentation/walletorders/building_a_distributable_order_package
pub struct Manifest {
    /// All manifest files with SHA-256
    items: Vec<Item>,
}

impl Manifest {
    fn add_item(path: &str, data: &[u8]) {
        todo!("Manifest not implemented yet!");
    }
}

pub struct Item {
    path: String,
    checksum: String,
}
