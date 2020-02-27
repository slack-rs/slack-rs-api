//! Functionality for sending requests to Slack.

use std::{borrow::Borrow, error};

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send<I, K, V, S>(&self, method: S, params: I) -> Result<String, Self::Error>
    where
        I: IntoIterator,
        K: AsRef<str>,
        V: AsRef<str>,
        I::Item: Borrow<(K, V)>,
        S: AsRef<str>;
}

#[cfg(feature = "reqwest")]
mod reqwest_support {
    pub use self::reqwest::Error;
    use reqwest_ as reqwest;
    use std::borrow::Borrow;

    use super::SlackWebRequestSender;

    impl SlackWebRequestSender for reqwest::blocking::Client {
        type Error = reqwest::Error;

        fn send<I, K, V, S>(&self, method_url: S, params: I) -> Result<String, Self::Error>
        where
            I: IntoIterator,
            K: AsRef<str>,
            V: AsRef<str>,
            I::Item: Borrow<(K, V)>,
            S: AsRef<str>,
        {
            let mut url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params);

            self.get(url).send()?.text()
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
    pub fn default_client() -> Result<reqwest::blocking::Client, reqwest::Error> {
        Ok(reqwest::blocking::Client::new())
    }
}

#[cfg(feature = "reqwest")]
pub use self::reqwest_support::*;
