use serde::{Deserialize, Serialize};

/// Represents Web Service
///
/// See [Apple documentation](https://developer.apple.com/documentation/walletpasses/adding_a_web_service_to_update_passes)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebService {
    /// The authentication token to use with the web service in the [web_service_url](WebService::web_service_url) key.
    pub authentication_token: String,

    /// The URL for a web service that you use to update or personalize the pass. The URL can include an optional port number.
    #[serde(rename = "webServiceURL")]
    pub web_service_url: String,
}
