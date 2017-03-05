

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Gets information about a bot user.
///
/// Wraps https://api.slack.com/methods/bots.info

pub fn info<R>(client: &R, token: &str, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), request.bot.map(|bot| ("bot", bot))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("bots.info", &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| serde_json::from_str::<InfoResponse>(&result).map_err(|_| InfoError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Bot user to get info on
    pub bot: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub bot: Option<InfoResponseBot>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponseBot {
    pub app_id: Option<String>,
    pub deleted: Option<bool>,
    pub icons: Option<InfoResponseBotIcons>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponseBotIcons {
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
}


impl<E: Error> Into<Result<InfoResponse, InfoError<E>>> for InfoResponse {
    fn into(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum InfoError<E: Error> {
    /// Value passed for bot was invalid.
    BotNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bot_not_found" => InfoError::BotNotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "user_is_bot" => InfoError::UserIsBot,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "request_timeout" => InfoError::RequestTimeout,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for InfoError<E> {
    fn description(&self) -> &str {
        match self {
            &InfoError::BotNotFound => "bot_not_found: Value passed for bot was invalid.",
            &InfoError::NotAuthed => "not_authed: No authentication token provided.",
            &InfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &InfoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &InfoError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &InfoError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common \
                 decency. This includes very long names and names with non-alphanumeric characters other than _. If \
                 you get this error, it is typically an indication that you have made a very malformed API call."
            }
            &InfoError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). \
                 These are never valid with the Slack API."
            }
            &InfoError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the \
                 Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            &InfoError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type \
                 application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or \
                 syntactically invalid."
            }
            &InfoError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was \
                 invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data \
                 text/plain."
            }
            &InfoError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the \
                 request did not include a Content-Type header."
            }
            &InfoError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or \
                 truncated."
            }
            &InfoError::MalformedResponse => "Malformed response data from Slack.",
            &InfoError::Unknown(ref s) => s,
            &InfoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
