use std::error;

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// Should be implemented for clients to send requests to Slack.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send(&self, method: &str, params: &[(&str, &str)]) -> Result<String, Self::Error>;
}

pub fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

#[cfg(feature = "reqwest")]
mod reqwest_support {
    extern crate reqwest;

    use std::io::Read;

    use super::{SlackWebRequestSender, get_slack_url_for_method};

    impl SlackWebRequestSender for reqwest::Client {
        type Error = reqwest::Error;

        fn send(&self, method: &str, params: &[(&str, &str)]) -> Result<String, Self::Error> {
            let url_string = get_slack_url_for_method(method);
            let mut url = reqwest::Url::parse(&url_string).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params);

            let mut response = self.get(url).send()?;
            let mut res_str = String::new();
            response.read_to_string(&mut res_str).map_err(reqwest::HyperError::from)?;

            Ok(res_str)
        }
    }
}

#[cfg(feature = "reqwest")]
pub use self::reqwest_support::*;
