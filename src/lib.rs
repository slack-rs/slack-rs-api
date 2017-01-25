// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level, direct interface for the [Slack Web
//! API](https://api.slack.com/methods).

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::Deserialize;

mod mods;
pub use mods::*;

mod types;
pub use types::*;

#[derive(Clone, Debug)]
pub struct ClientError;

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientError")
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        "Client error"
    }
}

/// Trait to turn an object into a Result<T, E>.
pub trait ToResult<T, E> {
    /// Turn self into a Result<T, E>
    fn to_result(&self) -> Result<T, E>;
}

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// Should be implemented for clients to send requests to Slack.
pub trait SlackWebRequestSender {
    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send<R>(&self, method: &str, params: HashMap<&str, String>) -> Result<R, ClientError>
        where R: Deserialize;
}
