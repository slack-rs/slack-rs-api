//! Functionality for sending requests to Slack.

use std::error;

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send(&self, method: &str, params: &[(&str, &str)]) -> Result<String, Self::Error>;
}

#[cfg(feature = "reqwest")]
mod reqwest_support {
    extern crate reqwest;
    pub use self::reqwest::Client;
    pub use self::reqwest::Error;

    use std::io::Read;

    use super::SlackWebRequestSender;

    impl SlackWebRequestSender for reqwest::Client {
        type Error = reqwest::Error;

        fn send(&self, method_url: &str, params: &[(&str, &str)]) -> Result<String, Self::Error> {
            let mut url = reqwest::Url::parse(&method_url).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params);

            let mut response = self.get(url).send()?;
            let mut res_str = String::new();
            response.read_to_string(&mut res_str).map_err(reqwest::HyperError::from)?;

            Ok(res_str)
        }
    }

    /// Provides a default `reqwest` client to give to the API functions to send requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # let token = "some_token";
    /// let client = slack_api::requests::default_client().unwrap();
    /// let response = slack_api::channels::list(&client, &token, &Default::default());
    /// ```
    pub fn default_client() -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::new()
    }
}

#[cfg(feature = "reqwest")]
pub use self::reqwest_support::*;
