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

use std::fmt;
use std::io;
use std::error;

use rustc_serialize;

/// Represents errors that can happen while using the Slack API
#[derive(Debug)]
pub enum Error {
    /// Error sending HTTP request to Slack
    HttpRequest(HttpRequestError),
    /// Error decoding Json
    JsonDecode(rustc_serialize::json::DecoderError),
    /// Error parsing Json
    JsonParse(rustc_serialize::json::ParserError),
    /// Error encoding Json
    JsonEncode(rustc_serialize::json::EncoderError),
    /// Slack Api Error
    Api(String),
    
    #[doc(hidden)]
    __Nonexhaustive(Void)
}

impl From<rustc_serialize::json::DecoderError> for Error {
    fn from(err: rustc_serialize::json::DecoderError) -> Error {
        Error::JsonDecode(err)
    }
}

impl From<rustc_serialize::json::ParserError> for Error {
    fn from(err: rustc_serialize::json::ParserError) -> Error {
        Error::JsonParse(err)
    }
}

impl From<rustc_serialize::json::EncoderError> for Error {
    fn from(err: rustc_serialize::json::EncoderError) -> Error {
        Error::JsonEncode(err)
    }
}

impl From<HttpRequestError> for Error {
    fn from(err: HttpRequestError) -> Error {
        Error::HttpRequest(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        f.write_str(self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpRequest(ref e) => e.description(),
            Error::JsonDecode(ref e) => e.description(),
            Error::JsonParse(ref e) => e.description(),
            Error::JsonEncode(ref e) => e.description(),
            Error::Api(ref st) => st,
            Error::__Nonexhaustive(ref void) => match *void {}
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::HttpRequest(ref e) => Some(e),
            Error::JsonDecode(ref e) => Some(e),
            Error::JsonParse(ref e) => Some(e),
            Error::JsonEncode(ref e) => Some(e),
            Error::Api(_) => None,
            Error::__Nonexhaustive(ref void) => match *void {}
        }
    }
}

/// Represents errors that can happen that occur when sending an HTTP request to Slack
///
/// Used in the `Request` variant of the `Error` type.
#[derive(Debug)]
pub enum HttpRequestError {
    /// An error occurring during network stream reading or writing
    Io(io::Error),
    
    #[doc(hidden)]
    __Nonexhaustive(Void)
}

impl error::Error for HttpRequestError {
    fn description(&self) -> &str {
        match *self {
            HttpRequestError::Io(ref e) => e.description(),
            HttpRequestError::__Nonexhaustive(ref void) => match *void {}
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HttpRequestError::Io(ref e) => Some(e),
            HttpRequestError::__Nonexhaustive(ref void) => match *void {}
        }
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        f.write_str(self.description())
    }
}

impl From<io::Error> for HttpRequestError {
    fn from(err: io::Error) -> HttpRequestError {
        HttpRequestError::Io(err)
    }
}

#[doc(hidden)]
pub enum Void {}

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}