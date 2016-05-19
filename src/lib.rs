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

extern crate rustc_serialize;

#[cfg(feature = "hyper")]
extern crate hyper;

use std::collections::HashMap;
use rustc_serialize::{json, Decodable};

mod types;
mod error;
mod message_events;

pub use self::types::*;
pub use self::message_events::Message;
pub use self::error::{Error, HttpRequestError};

pub mod api;
pub mod auth;
pub mod channels;
pub mod chat;
pub mod emoji;
pub mod files;
pub mod groups;
pub mod im;
pub mod oauth;
pub mod pins;
pub mod reactions;
pub mod rtm;
pub mod search;
pub mod stars;
pub mod team;
pub mod users;

pub type ApiResult<T> = Result<T, Error>;

fn parse_slack_response<T: Decodable>(response: String, check_ok: bool) -> ApiResult<T> {
    let raw_json = try!(json::Json::from_str(&response));
    let jobj = try!(raw_json.as_object()
                            .ok_or(Error::Api(format!("bad slack json response (not an \
                                                       object) {:?}",
                                                      raw_json))));
    if check_ok {
        let ok = try!(jobj.get("ok")
                          .ok_or(Error::Api(format!("slack json reponse does not contain \
                                                     \"ok\" field {:?}",
                                                    raw_json))));
        let is_ok = try!(ok.as_boolean()
                           .ok_or(Error::Api(format!("slack json reponse \"ok\" is not a \
                                                      boolean: {:?}",
                                                     raw_json))));
        if !is_ok {
            return Err(Error::Api(format!("slack json reponse \"ok\" is not true: {:?}",
                                          raw_json)));
        }
    }

    Ok(try!(json::decode(&response)))
}

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// Should be implemented for clients to send requests to Slack.
pub trait SlackWebRequestSender {
    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params. Returns the response body string after checking it has "ok": true, or an Error
    fn send<'a>(&self, method: &str, params: HashMap<&str, &'a str>) -> Result<String, HttpRequestError>;

    /// Make an API call to Slack that includes the configured token. Takes a map of parameters that
    /// get appended to the request as query params. Returns the response body string after checking it
    /// has `"ok": true`, or an Error
    fn send_authed<'a>(&self,
                       method: &str,
                       token: &'a str,
                       mut params: HashMap<&str, &'a str>)
                       -> Result<String, HttpRequestError> {
        params.insert("token", token);
        self.send(method, params)
    }
}

#[cfg(feature = "hyper")]
mod hyper_support {
    use hyper;
    use std::collections::HashMap;
    use std::io::Read;
    
    use super::{SlackWebRequestSender, HttpRequestError};
    
    impl SlackWebRequestSender for hyper::Client {
        fn send<'a>(&self, method: &str, params: HashMap<&str, &'a str>) -> Result<String, HttpRequestError> {
            let url_string = format!("https://slack.com/api/{}", method);
            let mut url = hyper::Url::parse(&url_string).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params.into_iter());

            let mut response = try!(self.get(url).send());
            let mut res_str = String::new();
            try!(response.read_to_string(&mut res_str));

            Ok(res_str)
        }
    }
    
    impl From<hyper::Error> for HttpRequestError {
        fn from(err: hyper::Error) -> HttpRequestError {
            match err {
                hyper::Error::Io(e) => HttpRequestError::Io(e),
                e => panic!("Unexpected Hyper request error: {}", e)
            }
        }
    }
}

#[cfg(feature = "hyper")]
pub use hyper_support::*;

#[cfg(test)]
mod test_helpers {
    use std::collections::HashMap;
    use super::{SlackWebRequestSender, HttpRequestError};
    
    pub struct MockSlackWebRequestSender {
        response: String
    }

    impl MockSlackWebRequestSender {
        pub fn respond_with<S: Into<String>>(response: S) -> Self {
            MockSlackWebRequestSender {
                response: response.into()
            }
        }
    }

    impl SlackWebRequestSender for MockSlackWebRequestSender {
        fn send<'b>(&self, _: &str, _: HashMap<&str, &'b str>) -> Result<String, HttpRequestError> {
            Ok(self.response.clone())
        }
    }
}