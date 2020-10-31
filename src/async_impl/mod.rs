mod mods;
pub use self::mods::*;

pub mod requests;

#[cfg(feature = "reqwest")]
pub use self::requests::default_client;

use async_trait::async_trait;

use std::{borrow::Borrow, error};

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
#[async_trait]
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an get API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    async fn get<I, K, V, S>(&self, method_url: S, params: I) -> Result<String, Self::Error>
    where
        I: IntoIterator + Send,
        K: AsRef<str>,
        V: AsRef<str>,
        I::Item: Borrow<(K, V)>,
        S: AsRef<str> + Send;

    /// Make an post API call to Slack. Takes a map of parameters that get appended to the request as body
    /// and a map of headers to set.
    async fn post<I, K, V, S>(
        &self,
        method_url: S,
        form: &[(&str, &str)],
        headers: I,
    ) -> Result<String, Self::Error>
    where
        I: IntoIterator + Send,
        K: AsRef<str>,
        V: AsRef<str>,
        I::Item: Borrow<(K, V)>,
        S: AsRef<str> + Send;
}
