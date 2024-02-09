/// Represents a pass (pass.json file)
pub struct Pass {
    /// (Required) The version of the file format. The value must be 1.
    pub format_version: u32,

    /// (Required) The name of the organization.
    pub organization_name: String,

    /// (Required) A short description that iOS accessibility technologies use for a pass.
    pub description: String,

    /// (Required) The pass type identifier that’s registered with Apple.
    /// The value must be the same as the distribution certificate used to sign the pass.
    pub pass_type_identifier: String,

    /// (Required) The Team ID for the Apple Developer Program account that registered the pass type identifier.
    pub team_identifier: String,

    /// (Required) An alphanumeric serial number.
    /// The combination of the serial number and pass type identifier must be unique for each pass.
    pub serial_number: Option<String>,

    /// An identifier the system uses to group related boarding passes or event tickets.
    /// Wallet displays passes with the same [groupingIdentifier](Pass::grouping_identifier), [passTypeIdentifier](Pass::pass_type_identifier), and type as a group.
    ///
    /// Use this identifier to group passes that are tightly related, such as boarding passes for different connections on the same trip.
    pub grouping_identifier: Option<String>,

    /// A color for the label text of the pass.
    /// If you don’t provide a value, the system determines the label color.
    pub label_color: Option<String>,

    /// A foreground color for the pass
    pub foreground_color: Option<String>,

    /// A background color for the pass
    pub background_color: Option<String>,

    /// The text to display next to the logo on the pass.
    pub logo_text: Option<String>,

    /// The date and time when the pass becomes relevant
    pub relevant_date: Option<String>,

    /// The date and time the pass expires.
    pub expiration_date: Option<String>,

    /// A URL to be passed to the associated app when launching it.
    pub app_launch_url: Option<String>,

    /// An array of App Store identifiers for apps associated with the pass.
    pub associated_store_identifiers: Vec<i32>,

    /// The authentication token to use with the web service in the [webServiceURL](Pass::web_service_url) key.
    pub authentication_token: Option<String>,

    /// The URL for a web service that you use to update or personalize the pass. The URL can include an optional port number.
    pub web_service_url: Option<String>,

    /// Controls whether to show the Share button on the back of a pass.
    /// A value of true removes the button. The default value is false.
    /// This flag has no effect in earlier versions of iOS, nor does it prevent sharing the pass in some other way.
    pub sharing_prohibited: bool, // TODO: default false

    /// Controls whether to display the strip image without a shine effect.
    /// The default value is true.
    pub suppress_strip_shine: bool, // TODO: default true

    /// Indicates that the pass is void, such as a redeemed, one-time-use coupon.
    /// The default value is false.
    pub voided: bool, // TODO: default false

    /// TODO: Barcode on a pass
    /// The system uses the first displayable barcode for the device.
    // pub barcodes: Vec<Barcode>,

    /// TODO: Array of Bluetooth Low Energy beacons the system uses to show a relevant pass.
    // pub beacons: Vec<Beacon>,

    // TODO: An array of up to 10 geographic locations the system uses to show a relevant pass.
    // pub locations: Vec<Location>,

    // The maximum distance, in meters, from a location in the [locations](Pass::locations) array at which the pass is relevant.
    pub max_distance: Option<u32>,

    // TODO: NFC
    // pub nfc: Option<NFC>,

    /// TODO: Semantic tags
    /// Metadata the system uses to offer a pass and suggest related actions.
    /// For example, setting Don’t Disturb mode for the duration of a movie.
    // pub semantics: Vec<SemanticTag>,

    // TODO: PassTypes
    // boarding pass
    // coupon
    // event ticket
    // generic

    // TODO: UserInfo
    // custom JSOM
}

#[cfg(test)]
mod tests {
    use super::*;
}
