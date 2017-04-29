//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.


#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive

pub fn archive<R>(client: &R,
                  token: &str,
                  request: &ArchiveRequest)
                  -> Result<ArchiveResponse, ArchiveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.archive");
    client
        .send(&url, &params[..])
        .map_err(|err| ArchiveError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<ArchiveResponse>(&result)
                            .map_err(|e| ArchiveError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ArchiveRequest<'a> {
    /// Channel to archive
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArchiveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<ArchiveResponse, ArchiveError<E>>> for ArchiveResponse {
    fn into(self) -> Result<ArchiveResponse, ArchiveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum ArchiveError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel has already been archived.
    AlreadyArchived,
    /// You cannot archive the general channel
    CantArchiveGeneral,
    /// You cannot archive the last channel for a multi-channel guest
    LastRaChannel,
    /// A team preference prevents the authenticated user from archiving.
    RestrictedAction,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ArchiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => ArchiveError::ChannelNotFound,
            "already_archived" => ArchiveError::AlreadyArchived,
            "cant_archive_general" => ArchiveError::CantArchiveGeneral,
            "last_ra_channel" => ArchiveError::LastRaChannel,
            "restricted_action" => ArchiveError::RestrictedAction,
            "not_authed" => ArchiveError::NotAuthed,
            "invalid_auth" => ArchiveError::InvalidAuth,
            "account_inactive" => ArchiveError::AccountInactive,
            "user_is_bot" => ArchiveError::UserIsBot,
            "user_is_restricted" => ArchiveError::UserIsRestricted,
            "invalid_arg_name" => ArchiveError::InvalidArgName,
            "invalid_array_arg" => ArchiveError::InvalidArrayArg,
            "invalid_charset" => ArchiveError::InvalidCharset,
            "invalid_form_data" => ArchiveError::InvalidFormData,
            "invalid_post_type" => ArchiveError::InvalidPostType,
            "missing_post_type" => ArchiveError::MissingPostType,
            "request_timeout" => ArchiveError::RequestTimeout,
            _ => ArchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ArchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for ArchiveError<E> {
    fn description(&self) -> &str {
        match self {
            &ArchiveError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &ArchiveError::AlreadyArchived => "already_archived: Channel has already been archived.",
            &ArchiveError::CantArchiveGeneral => "cant_archive_general: You cannot archive the general channel",
            &ArchiveError::LastRaChannel => "last_ra_channel: You cannot archive the last channel for a multi-channel guest",
            &ArchiveError::RestrictedAction => "restricted_action: A team preference prevents the authenticated user from archiving.",
            &ArchiveError::NotAuthed => "not_authed: No authentication token provided.",
            &ArchiveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &ArchiveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &ArchiveError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &ArchiveError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &ArchiveError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &ArchiveError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &ArchiveError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &ArchiveError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &ArchiveError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &ArchiveError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &ArchiveError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &ArchiveError::MalformedResponse(ref e) => e.description(),
            &ArchiveError::Unknown(ref s) => s,
            &ArchiveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ArchiveError::MalformedResponse(ref e) => Some(e),
            &ArchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create

pub fn create<R>(client: &R,
                 token: &str,
                 request: &CreateRequest)
                 -> Result<CreateResponse, CreateError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.create");
    client
        .send(&url, &params[..])
        .map_err(|err| CreateError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<CreateResponse>(&result)
                            .map_err(|e| CreateError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct CreateRequest<'a> {
    /// Name of channel to create
    pub name: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    pub channel: Option<::Channel>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<CreateResponse, CreateError<E>>> for CreateResponse {
    fn into(self) -> Result<CreateResponse, CreateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum CreateError<E: Error> {
    /// A channel cannot be created with the given name.
    NameTaken,
    /// A team preference prevents the authenticated user from creating channels.
    RestrictedAction,
    /// Value passed for name was empty.
    NoChannel,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "name_taken" => CreateError::NameTaken,
            "restricted_action" => CreateError::RestrictedAction,
            "no_channel" => CreateError::NoChannel,
            "not_authed" => CreateError::NotAuthed,
            "invalid_auth" => CreateError::InvalidAuth,
            "account_inactive" => CreateError::AccountInactive,
            "user_is_bot" => CreateError::UserIsBot,
            "user_is_restricted" => CreateError::UserIsRestricted,
            "invalid_arg_name" => CreateError::InvalidArgName,
            "invalid_array_arg" => CreateError::InvalidArrayArg,
            "invalid_charset" => CreateError::InvalidCharset,
            "invalid_form_data" => CreateError::InvalidFormData,
            "invalid_post_type" => CreateError::InvalidPostType,
            "missing_post_type" => CreateError::MissingPostType,
            "request_timeout" => CreateError::RequestTimeout,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for CreateError<E> {
    fn description(&self) -> &str {
        match self {
            &CreateError::NameTaken => "name_taken: A channel cannot be created with the given name.",
            &CreateError::RestrictedAction => "restricted_action: A team preference prevents the authenticated user from creating channels.",
            &CreateError::NoChannel => "no_channel: Value passed for name was empty.",
            &CreateError::NotAuthed => "not_authed: No authentication token provided.",
            &CreateError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &CreateError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &CreateError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &CreateError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &CreateError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &CreateError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &CreateError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &CreateError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &CreateError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &CreateError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &CreateError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &CreateError::MalformedResponse(ref e) => e.description(),
            &CreateError::Unknown(ref s) => s,
            &CreateError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &CreateError::MalformedResponse(ref e) => Some(e),
            &CreateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history

pub fn history<R>(client: &R,
                  token: &str,
                  request: &HistoryRequest)
                  -> Result<HistoryResponse, HistoryError<R::Error>>
    where R: SlackWebRequestSender
{
    let count = request.count.map(|count| count.to_string());
    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      request.latest.map(|latest| ("latest", latest)),
                      request.oldest.map(|oldest| ("oldest", oldest)),
                      request
                          .inclusive
                          .map(|inclusive| ("inclusive", if inclusive { "1" } else { "0" })),
                      count.as_ref().map(|count| ("count", &count[..])),
                      request
                          .unreads
                          .map(|unreads| ("unreads", if unreads { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.history");
    client
        .send(&url, &params[..])
        .map_err(|err| HistoryError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<HistoryResponse>(&result)
                            .map_err(|e| HistoryError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct HistoryRequest<'a> {
    /// Channel to fetch history for.
    pub channel: &'a str,
    /// End of time range of messages to include in results.
    pub latest: Option<&'a str>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<&'a str>,
    /// Include messages with latest or oldest timestamp in results.
    pub inclusive: Option<bool>,
    /// Number of messages to return, between 1 and 1000.
    pub count: Option<u32>,
    /// Include unread_count_display in the output?
    pub unreads: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryResponse {
    error: Option<String>,
    pub has_more: Option<bool>,
    pub latest: Option<String>,
    pub messages: Option<Vec<::Message>>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<HistoryResponse, HistoryError<E>>> for HistoryResponse {
    fn into(self) -> Result<HistoryResponse, HistoryError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum HistoryError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for latest was invalid
    InvalidTsLatest,
    /// Value passed for oldest was invalid
    InvalidTsOldest,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for HistoryError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => HistoryError::ChannelNotFound,
            "invalid_ts_latest" => HistoryError::InvalidTsLatest,
            "invalid_ts_oldest" => HistoryError::InvalidTsOldest,
            "not_authed" => HistoryError::NotAuthed,
            "invalid_auth" => HistoryError::InvalidAuth,
            "account_inactive" => HistoryError::AccountInactive,
            "invalid_arg_name" => HistoryError::InvalidArgName,
            "invalid_array_arg" => HistoryError::InvalidArrayArg,
            "invalid_charset" => HistoryError::InvalidCharset,
            "invalid_form_data" => HistoryError::InvalidFormData,
            "invalid_post_type" => HistoryError::InvalidPostType,
            "missing_post_type" => HistoryError::MissingPostType,
            "request_timeout" => HistoryError::RequestTimeout,
            _ => HistoryError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for HistoryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for HistoryError<E> {
    fn description(&self) -> &str {
        match self {
            &HistoryError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &HistoryError::InvalidTsLatest => "invalid_ts_latest: Value passed for latest was invalid",
            &HistoryError::InvalidTsOldest => "invalid_ts_oldest: Value passed for oldest was invalid",
            &HistoryError::NotAuthed => "not_authed: No authentication token provided.",
            &HistoryError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &HistoryError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &HistoryError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &HistoryError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &HistoryError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &HistoryError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &HistoryError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &HistoryError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &HistoryError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &HistoryError::MalformedResponse(ref e) => e.description(),
            &HistoryError::Unknown(ref s) => s,
            &HistoryError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &HistoryError::MalformedResponse(ref e) => Some(e),
            &HistoryError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info

pub fn info<R>(client: &R,
               token: &str,
               request: &InfoRequest)
               -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.info");
    client
        .send(&url, &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<InfoResponse>(&result)
                            .map_err(|e| InfoError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Channel to get info on
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub channel: Option<::Channel>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<InfoResponse, InfoError<E>>> for InfoResponse {
    fn into(self) -> Result<InfoResponse, InfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum InfoError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => InfoError::ChannelNotFound,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
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
            &InfoError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &InfoError::NotAuthed => "not_authed: No authentication token provided.",
            &InfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &InfoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &InfoError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &InfoError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &InfoError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &InfoError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &InfoError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &InfoError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &InfoError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &InfoError::MalformedResponse(ref e) => e.description(),
            &InfoError::Unknown(ref s) => s,
            &InfoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &InfoError::MalformedResponse(ref e) => Some(e),
            &InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite

pub fn invite<R>(client: &R,
                 token: &str,
                 request: &InviteRequest)
                 -> Result<InviteResponse, InviteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.invite");
    client
        .send(&url, &params[..])
        .map_err(|err| InviteError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<InviteResponse>(&result)
                            .map_err(|e| InviteError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InviteRequest<'a> {
    /// Channel to invite user to.
    pub channel: &'a str,
    /// User to invite to channel.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteResponse {
    pub channel: Option<::Channel>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<InviteResponse, InviteError<E>>> for InviteResponse {
    fn into(self) -> Result<InviteResponse, InviteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum InviteError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for user was invalid.
    UserNotFound,
    /// Authenticated user cannot invite themselves to a channel.
    CantInviteSelf,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Invited user is already in the channel.
    AlreadyInChannel,
    /// Channel has been archived.
    IsArchived,
    /// User cannot be invited to this channel.
    CantInvite,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a single channel guest.
    UserIsUltraRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InviteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => InviteError::ChannelNotFound,
            "user_not_found" => InviteError::UserNotFound,
            "cant_invite_self" => InviteError::CantInviteSelf,
            "not_in_channel" => InviteError::NotInChannel,
            "already_in_channel" => InviteError::AlreadyInChannel,
            "is_archived" => InviteError::IsArchived,
            "cant_invite" => InviteError::CantInvite,
            "not_authed" => InviteError::NotAuthed,
            "invalid_auth" => InviteError::InvalidAuth,
            "account_inactive" => InviteError::AccountInactive,
            "user_is_bot" => InviteError::UserIsBot,
            "user_is_ultra_restricted" => InviteError::UserIsUltraRestricted,
            "invalid_arg_name" => InviteError::InvalidArgName,
            "invalid_array_arg" => InviteError::InvalidArrayArg,
            "invalid_charset" => InviteError::InvalidCharset,
            "invalid_form_data" => InviteError::InvalidFormData,
            "invalid_post_type" => InviteError::InvalidPostType,
            "missing_post_type" => InviteError::MissingPostType,
            "request_timeout" => InviteError::RequestTimeout,
            _ => InviteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InviteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for InviteError<E> {
    fn description(&self) -> &str {
        match self {
            &InviteError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &InviteError::UserNotFound => "user_not_found: Value passed for user was invalid.",
            &InviteError::CantInviteSelf => "cant_invite_self: Authenticated user cannot invite themselves to a channel.",
            &InviteError::NotInChannel => "not_in_channel: Authenticated user is not in the channel.",
            &InviteError::AlreadyInChannel => "already_in_channel: Invited user is already in the channel.",
            &InviteError::IsArchived => "is_archived: Channel has been archived.",
            &InviteError::CantInvite => "cant_invite: User cannot be invited to this channel.",
            &InviteError::NotAuthed => "not_authed: No authentication token provided.",
            &InviteError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &InviteError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &InviteError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &InviteError::UserIsUltraRestricted => "user_is_ultra_restricted: This method cannot be called by a single channel guest.",
            &InviteError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &InviteError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &InviteError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &InviteError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &InviteError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &InviteError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &InviteError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &InviteError::MalformedResponse(ref e) => e.description(),
            &InviteError::Unknown(ref s) => s,
            &InviteError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &InviteError::MalformedResponse(ref e) => Some(e),
            &InviteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join

pub fn join<R>(client: &R,
               token: &str,
               request: &JoinRequest)
               -> Result<JoinResponse, JoinError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.join");
    client
        .send(&url, &params[..])
        .map_err(|err| JoinError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<JoinResponse>(&result)
                            .map_err(|e| JoinError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct JoinRequest<'a> {
    /// Name of channel to join
    pub name: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinResponse {
    pub channel: Option<::Channel>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<JoinResponse, JoinError<E>>> for JoinResponse {
    fn into(self) -> Result<JoinResponse, JoinError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum JoinError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// A channel cannot be created with the given name.
    NameTaken,
    /// A team preference prevents the authenticated user from creating channels.
    RestrictedAction,
    /// Value passed for name was empty.
    NoChannel,
    /// Channel has been archived.
    IsArchived,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for JoinError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => JoinError::ChannelNotFound,
            "name_taken" => JoinError::NameTaken,
            "restricted_action" => JoinError::RestrictedAction,
            "no_channel" => JoinError::NoChannel,
            "is_archived" => JoinError::IsArchived,
            "not_authed" => JoinError::NotAuthed,
            "invalid_auth" => JoinError::InvalidAuth,
            "account_inactive" => JoinError::AccountInactive,
            "user_is_bot" => JoinError::UserIsBot,
            "user_is_restricted" => JoinError::UserIsRestricted,
            "invalid_arg_name" => JoinError::InvalidArgName,
            "invalid_array_arg" => JoinError::InvalidArrayArg,
            "invalid_charset" => JoinError::InvalidCharset,
            "invalid_form_data" => JoinError::InvalidFormData,
            "invalid_post_type" => JoinError::InvalidPostType,
            "missing_post_type" => JoinError::MissingPostType,
            "request_timeout" => JoinError::RequestTimeout,
            _ => JoinError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for JoinError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for JoinError<E> {
    fn description(&self) -> &str {
        match self {
            &JoinError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &JoinError::NameTaken => "name_taken: A channel cannot be created with the given name.",
            &JoinError::RestrictedAction => "restricted_action: A team preference prevents the authenticated user from creating channels.",
            &JoinError::NoChannel => "no_channel: Value passed for name was empty.",
            &JoinError::IsArchived => "is_archived: Channel has been archived.",
            &JoinError::NotAuthed => "not_authed: No authentication token provided.",
            &JoinError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &JoinError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &JoinError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &JoinError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &JoinError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &JoinError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &JoinError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &JoinError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &JoinError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &JoinError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &JoinError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &JoinError::MalformedResponse(ref e) => e.description(),
            &JoinError::Unknown(ref s) => s,
            &JoinError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &JoinError::MalformedResponse(ref e) => Some(e),
            &JoinError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick

pub fn kick<R>(client: &R,
               token: &str,
               request: &KickRequest)
               -> Result<KickResponse, KickError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.kick");
    client
        .send(&url, &params[..])
        .map_err(|err| KickError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<KickResponse>(&result)
                            .map_err(|e| KickError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct KickRequest<'a> {
    /// Channel to remove user from.
    pub channel: &'a str,
    /// User to remove from channel.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KickResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<KickResponse, KickError<E>>> for KickResponse {
    fn into(self) -> Result<KickResponse, KickError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum KickError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for user was invalid.
    UserNotFound,
    /// Authenticated user can't kick themselves from a channel.
    CantKickSelf,
    /// User was not in the channel.
    NotInChannel,
    /// User cannot be removed from #general.
    CantKickFromGeneral,
    /// User cannot be removed from the last channel they're in.
    CantKickFromLastChannel,
    /// A team preference prevents the authenticated user from kicking.
    RestrictedAction,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for KickError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => KickError::ChannelNotFound,
            "user_not_found" => KickError::UserNotFound,
            "cant_kick_self" => KickError::CantKickSelf,
            "not_in_channel" => KickError::NotInChannel,
            "cant_kick_from_general" => KickError::CantKickFromGeneral,
            "cant_kick_from_last_channel" => KickError::CantKickFromLastChannel,
            "restricted_action" => KickError::RestrictedAction,
            "not_authed" => KickError::NotAuthed,
            "invalid_auth" => KickError::InvalidAuth,
            "account_inactive" => KickError::AccountInactive,
            "user_is_bot" => KickError::UserIsBot,
            "user_is_restricted" => KickError::UserIsRestricted,
            "invalid_arg_name" => KickError::InvalidArgName,
            "invalid_array_arg" => KickError::InvalidArrayArg,
            "invalid_charset" => KickError::InvalidCharset,
            "invalid_form_data" => KickError::InvalidFormData,
            "invalid_post_type" => KickError::InvalidPostType,
            "missing_post_type" => KickError::MissingPostType,
            "request_timeout" => KickError::RequestTimeout,
            _ => KickError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for KickError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for KickError<E> {
    fn description(&self) -> &str {
        match self {
            &KickError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &KickError::UserNotFound => "user_not_found: Value passed for user was invalid.",
            &KickError::CantKickSelf => "cant_kick_self: Authenticated user can't kick themselves from a channel.",
            &KickError::NotInChannel => "not_in_channel: User was not in the channel.",
            &KickError::CantKickFromGeneral => "cant_kick_from_general: User cannot be removed from #general.",
            &KickError::CantKickFromLastChannel => "cant_kick_from_last_channel: User cannot be removed from the last channel they're in.",
            &KickError::RestrictedAction => "restricted_action: A team preference prevents the authenticated user from kicking.",
            &KickError::NotAuthed => "not_authed: No authentication token provided.",
            &KickError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &KickError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &KickError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &KickError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &KickError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &KickError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &KickError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &KickError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &KickError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &KickError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &KickError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &KickError::MalformedResponse(ref e) => e.description(),
            &KickError::Unknown(ref s) => s,
            &KickError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &KickError::MalformedResponse(ref e) => Some(e),
            &KickError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave

pub fn leave<R>(client: &R,
                token: &str,
                request: &LeaveRequest)
                -> Result<LeaveResponse, LeaveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.leave");
    client
        .send(&url, &params[..])
        .map_err(|err| LeaveError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<LeaveResponse>(&result)
                            .map_err(|e| LeaveError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct LeaveRequest<'a> {
    /// Channel to leave
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LeaveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<LeaveResponse, LeaveError<E>>> for LeaveResponse {
    fn into(self) -> Result<LeaveResponse, LeaveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum LeaveError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel has been archived.
    IsArchived,
    /// Authenticated user cannot leave the general channel
    CantLeaveGeneral,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for LeaveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => LeaveError::ChannelNotFound,
            "is_archived" => LeaveError::IsArchived,
            "cant_leave_general" => LeaveError::CantLeaveGeneral,
            "not_authed" => LeaveError::NotAuthed,
            "invalid_auth" => LeaveError::InvalidAuth,
            "account_inactive" => LeaveError::AccountInactive,
            "user_is_bot" => LeaveError::UserIsBot,
            "user_is_restricted" => LeaveError::UserIsRestricted,
            "invalid_arg_name" => LeaveError::InvalidArgName,
            "invalid_array_arg" => LeaveError::InvalidArrayArg,
            "invalid_charset" => LeaveError::InvalidCharset,
            "invalid_form_data" => LeaveError::InvalidFormData,
            "invalid_post_type" => LeaveError::InvalidPostType,
            "missing_post_type" => LeaveError::MissingPostType,
            "request_timeout" => LeaveError::RequestTimeout,
            _ => LeaveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for LeaveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for LeaveError<E> {
    fn description(&self) -> &str {
        match self {
            &LeaveError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &LeaveError::IsArchived => "is_archived: Channel has been archived.",
            &LeaveError::CantLeaveGeneral => "cant_leave_general: Authenticated user cannot leave the general channel",
            &LeaveError::NotAuthed => "not_authed: No authentication token provided.",
            &LeaveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &LeaveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &LeaveError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &LeaveError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &LeaveError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &LeaveError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &LeaveError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &LeaveError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &LeaveError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &LeaveError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &LeaveError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &LeaveError::MalformedResponse(ref e) => e.description(),
            &LeaveError::Unknown(ref s) => s,
            &LeaveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LeaveError::MalformedResponse(ref e) => Some(e),
            &LeaveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list

pub fn list<R>(client: &R,
               token: &str,
               request: &ListRequest)
               -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      request
                          .exclude_archived
                          .map(|exclude_archived| {
                                   ("exclude_archived", if exclude_archived { "1" } else { "0" })
                               })];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.list");
    client
        .send(&url, &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<ListResponse>(&result)
                            .map_err(|e| ListError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest {
    /// Don't return archived channels.
    pub exclude_archived: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub channels: Option<Vec<::Channel>>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum ListError<E: Error> {
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
            "request_timeout" => ListError::RequestTimeout,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for ListError<E> {
    fn description(&self) -> &str {
        match self {
            &ListError::NotAuthed => "not_authed: No authentication token provided.",
            &ListError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &ListError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &ListError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &ListError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &ListError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &ListError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &ListError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &ListError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &ListError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &ListError::MalformedResponse(ref e) => e.description(),
            &ListError::Unknown(ref s) => s,
            &ListError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ListError::MalformedResponse(ref e) => Some(e),
            &ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/channels.mark

pub fn mark<R>(client: &R,
               token: &str,
               request: &MarkRequest)
               -> Result<MarkResponse, MarkError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("ts", request.ts))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.mark");
    client
        .send(&url, &params[..])
        .map_err(|err| MarkError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<MarkResponse>(&result)
                            .map_err(|e| MarkError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct MarkRequest<'a> {
    /// Channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarkResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<MarkResponse, MarkError<E>>> for MarkResponse {
    fn into(self) -> Result<MarkResponse, MarkError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum MarkError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for timestamp was invalid.
    InvalidTimestamp,
    /// Caller is not a member of the channel.
    NotInChannel,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for MarkError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => MarkError::ChannelNotFound,
            "invalid_timestamp" => MarkError::InvalidTimestamp,
            "not_in_channel" => MarkError::NotInChannel,
            "not_authed" => MarkError::NotAuthed,
            "invalid_auth" => MarkError::InvalidAuth,
            "account_inactive" => MarkError::AccountInactive,
            "invalid_arg_name" => MarkError::InvalidArgName,
            "invalid_array_arg" => MarkError::InvalidArrayArg,
            "invalid_charset" => MarkError::InvalidCharset,
            "invalid_form_data" => MarkError::InvalidFormData,
            "invalid_post_type" => MarkError::InvalidPostType,
            "missing_post_type" => MarkError::MissingPostType,
            "request_timeout" => MarkError::RequestTimeout,
            _ => MarkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MarkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for MarkError<E> {
    fn description(&self) -> &str {
        match self {
            &MarkError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &MarkError::InvalidTimestamp => "invalid_timestamp: Value passed for timestamp was invalid.",
            &MarkError::NotInChannel => "not_in_channel: Caller is not a member of the channel.",
            &MarkError::NotAuthed => "not_authed: No authentication token provided.",
            &MarkError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &MarkError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &MarkError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &MarkError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &MarkError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &MarkError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &MarkError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &MarkError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &MarkError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &MarkError::MalformedResponse(ref e) => e.description(),
            &MarkError::Unknown(ref s) => s,
            &MarkError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &MarkError::MalformedResponse(ref e) => Some(e),
            &MarkError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename

pub fn rename<R>(client: &R,
                 token: &str,
                 request: &RenameRequest)
                 -> Result<RenameResponse, RenameError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.rename");
    client
        .send(&url, &params[..])
        .map_err(|err| RenameError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<RenameResponse>(&result)
                            .map_err(|e| RenameError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RenameRequest<'a> {
    /// Channel to rename
    pub channel: &'a str,
    /// New name for channel.
    pub name: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameResponse {
    pub channel: Option<RenameResponseChannel>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameResponseChannel {
    pub created: Option<f32>,
    pub id: Option<String>,
    pub is_channel: Option<bool>,
    pub name: Option<String>,
}


impl<E: Error> Into<Result<RenameResponse, RenameError<E>>> for RenameResponse {
    fn into(self) -> Result<RenameResponse, RenameError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum RenameError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Caller is not a member of the channel.
    NotInChannel,
    /// Caller cannot rename this channel
    NotAuthorized,
    /// New name is invalid
    InvalidName,
    /// New channel name is taken
    NameTaken,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RenameError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => RenameError::ChannelNotFound,
            "not_in_channel" => RenameError::NotInChannel,
            "not_authorized" => RenameError::NotAuthorized,
            "invalid_name" => RenameError::InvalidName,
            "name_taken" => RenameError::NameTaken,
            "not_authed" => RenameError::NotAuthed,
            "invalid_auth" => RenameError::InvalidAuth,
            "account_inactive" => RenameError::AccountInactive,
            "user_is_bot" => RenameError::UserIsBot,
            "user_is_restricted" => RenameError::UserIsRestricted,
            "invalid_arg_name" => RenameError::InvalidArgName,
            "invalid_array_arg" => RenameError::InvalidArrayArg,
            "invalid_charset" => RenameError::InvalidCharset,
            "invalid_form_data" => RenameError::InvalidFormData,
            "invalid_post_type" => RenameError::InvalidPostType,
            "missing_post_type" => RenameError::MissingPostType,
            "request_timeout" => RenameError::RequestTimeout,
            _ => RenameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RenameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RenameError<E> {
    fn description(&self) -> &str {
        match self {
            &RenameError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &RenameError::NotInChannel => "not_in_channel: Caller is not a member of the channel.",
            &RenameError::NotAuthorized => "not_authorized: Caller cannot rename this channel",
            &RenameError::InvalidName => "invalid_name: New name is invalid",
            &RenameError::NameTaken => "name_taken: New channel name is taken",
            &RenameError::NotAuthed => "not_authed: No authentication token provided.",
            &RenameError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &RenameError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &RenameError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &RenameError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &RenameError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &RenameError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &RenameError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &RenameError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &RenameError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &RenameError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &RenameError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &RenameError::MalformedResponse(ref e) => e.description(),
            &RenameError::Unknown(ref s) => s,
            &RenameError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RenameError::MalformedResponse(ref e) => Some(e),
            &RenameError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Retrieve a thread of messages posted to a channel
///
/// Wraps https://api.slack.com/methods/channels.replies

pub fn replies<R>(client: &R,
                  token: &str,
                  request: &RepliesRequest)
                  -> Result<RepliesResponse, RepliesError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("thread_ts", request.thread_ts))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.replies");
    client
        .send(&url, &params[..])
        .map_err(|err| RepliesError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<RepliesResponse>(&result)
                            .map_err(|e| RepliesError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RepliesRequest<'a> {
    /// Channel to fetch thread from
    pub channel: &'a str,
    /// Unique identifier of a thread's parent message
    pub thread_ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepliesResponse {
    error: Option<String>,
    pub messages: Option<Vec<::Message>>,
    #[serde(default)]
    ok: bool,
    pub thread_info: Option<::ThreadInfo>,
}


impl<E: Error> Into<Result<RepliesResponse, RepliesError<E>>> for RepliesResponse {
    fn into(self) -> Result<RepliesResponse, RepliesError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum RepliesError<E: Error> {
    /// Value for channel was missing or invalid.
    ChannelNotFound,
    /// Value for thread_ts was missing or invalid.
    ThreadNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RepliesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => RepliesError::ChannelNotFound,
            "thread_not_found" => RepliesError::ThreadNotFound,
            "not_authed" => RepliesError::NotAuthed,
            "invalid_auth" => RepliesError::InvalidAuth,
            "account_inactive" => RepliesError::AccountInactive,
            "invalid_arg_name" => RepliesError::InvalidArgName,
            "invalid_array_arg" => RepliesError::InvalidArrayArg,
            "invalid_charset" => RepliesError::InvalidCharset,
            "invalid_form_data" => RepliesError::InvalidFormData,
            "invalid_post_type" => RepliesError::InvalidPostType,
            "missing_post_type" => RepliesError::MissingPostType,
            "request_timeout" => RepliesError::RequestTimeout,
            _ => RepliesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RepliesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RepliesError<E> {
    fn description(&self) -> &str {
        match self {
            &RepliesError::ChannelNotFound => "channel_not_found: Value for channel was missing or invalid.",
            &RepliesError::ThreadNotFound => "thread_not_found: Value for thread_ts was missing or invalid.",
            &RepliesError::NotAuthed => "not_authed: No authentication token provided.",
            &RepliesError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &RepliesError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &RepliesError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &RepliesError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &RepliesError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &RepliesError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &RepliesError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &RepliesError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &RepliesError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &RepliesError::MalformedResponse(ref e) => e.description(),
            &RepliesError::Unknown(ref s) => s,
            &RepliesError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RepliesError::MalformedResponse(ref e) => Some(e),
            &RepliesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose

pub fn set_purpose<R>(client: &R,
                      token: &str,
                      request: &SetPurposeRequest)
                      -> Result<SetPurposeResponse, SetPurposeError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("purpose", request.purpose))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.setPurpose");
    client
        .send(&url, &params[..])
        .map_err(|err| SetPurposeError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<SetPurposeResponse>(&result)
                            .map_err(|e| SetPurposeError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetPurposeRequest<'a> {
    /// Channel to set the purpose of
    pub channel: &'a str,
    /// The new purpose
    pub purpose: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub purpose: Option<String>,
}


impl<E: Error> Into<Result<SetPurposeResponse, SetPurposeError<E>>> for SetPurposeResponse {
    fn into(self) -> Result<SetPurposeResponse, SetPurposeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum SetPurposeError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Purpose was longer than 250 characters.
    TooLong,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetPurposeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => SetPurposeError::ChannelNotFound,
            "not_in_channel" => SetPurposeError::NotInChannel,
            "is_archived" => SetPurposeError::IsArchived,
            "too_long" => SetPurposeError::TooLong,
            "user_is_restricted" => SetPurposeError::UserIsRestricted,
            "not_authed" => SetPurposeError::NotAuthed,
            "invalid_auth" => SetPurposeError::InvalidAuth,
            "account_inactive" => SetPurposeError::AccountInactive,
            "invalid_arg_name" => SetPurposeError::InvalidArgName,
            "invalid_array_arg" => SetPurposeError::InvalidArrayArg,
            "invalid_charset" => SetPurposeError::InvalidCharset,
            "invalid_form_data" => SetPurposeError::InvalidFormData,
            "invalid_post_type" => SetPurposeError::InvalidPostType,
            "missing_post_type" => SetPurposeError::MissingPostType,
            "request_timeout" => SetPurposeError::RequestTimeout,
            _ => SetPurposeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPurposeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetPurposeError<E> {
    fn description(&self) -> &str {
        match self {
            &SetPurposeError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &SetPurposeError::NotInChannel => "not_in_channel: Authenticated user is not in the channel.",
            &SetPurposeError::IsArchived => "is_archived: Channel has been archived.",
            &SetPurposeError::TooLong => "too_long: Purpose was longer than 250 characters.",
            &SetPurposeError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &SetPurposeError::NotAuthed => "not_authed: No authentication token provided.",
            &SetPurposeError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &SetPurposeError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &SetPurposeError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &SetPurposeError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &SetPurposeError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &SetPurposeError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &SetPurposeError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &SetPurposeError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &SetPurposeError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &SetPurposeError::MalformedResponse(ref e) => e.description(),
            &SetPurposeError::Unknown(ref s) => s,
            &SetPurposeError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetPurposeError::MalformedResponse(ref e) => Some(e),
            &SetPurposeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic

pub fn set_topic<R>(client: &R,
                    token: &str,
                    request: &SetTopicRequest)
                    -> Result<SetTopicResponse, SetTopicError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)),
                      Some(("channel", request.channel)),
                      Some(("topic", request.topic))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.setTopic");
    client
        .send(&url, &params[..])
        .map_err(|err| SetTopicError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<SetTopicResponse>(&result)
                            .map_err(|e| SetTopicError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetTopicRequest<'a> {
    /// Channel to set the topic of
    pub channel: &'a str,
    /// The new topic
    pub topic: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub topic: Option<String>,
}


impl<E: Error> Into<Result<SetTopicResponse, SetTopicError<E>>> for SetTopicResponse {
    fn into(self) -> Result<SetTopicResponse, SetTopicError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum SetTopicError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Topic was longer than 250 characters.
    TooLong,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetTopicError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => SetTopicError::ChannelNotFound,
            "not_in_channel" => SetTopicError::NotInChannel,
            "is_archived" => SetTopicError::IsArchived,
            "too_long" => SetTopicError::TooLong,
            "user_is_restricted" => SetTopicError::UserIsRestricted,
            "not_authed" => SetTopicError::NotAuthed,
            "invalid_auth" => SetTopicError::InvalidAuth,
            "account_inactive" => SetTopicError::AccountInactive,
            "invalid_arg_name" => SetTopicError::InvalidArgName,
            "invalid_array_arg" => SetTopicError::InvalidArrayArg,
            "invalid_charset" => SetTopicError::InvalidCharset,
            "invalid_form_data" => SetTopicError::InvalidFormData,
            "invalid_post_type" => SetTopicError::InvalidPostType,
            "missing_post_type" => SetTopicError::MissingPostType,
            "request_timeout" => SetTopicError::RequestTimeout,
            _ => SetTopicError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetTopicError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for SetTopicError<E> {
    fn description(&self) -> &str {
        match self {
            &SetTopicError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &SetTopicError::NotInChannel => "not_in_channel: Authenticated user is not in the channel.",
            &SetTopicError::IsArchived => "is_archived: Channel has been archived.",
            &SetTopicError::TooLong => "too_long: Topic was longer than 250 characters.",
            &SetTopicError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &SetTopicError::NotAuthed => "not_authed: No authentication token provided.",
            &SetTopicError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &SetTopicError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &SetTopicError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &SetTopicError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &SetTopicError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &SetTopicError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &SetTopicError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &SetTopicError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &SetTopicError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &SetTopicError::MalformedResponse(ref e) => e.description(),
            &SetTopicError::Unknown(ref s) => s,
            &SetTopicError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetTopicError::MalformedResponse(ref e) => Some(e),
            &SetTopicError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive

pub fn unarchive<R>(client: &R,
                    token: &str,
                    request: &UnarchiveRequest)
                    -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("channels.unarchive");
    client
        .send(&url, &params[..])
        .map_err(|err| UnarchiveError::Client(err))
        .and_then(|result| {
                      serde_json::from_str::<UnarchiveResponse>(&result)
                            .map_err(|e| UnarchiveError::MalformedResponse(e))
                  })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct UnarchiveRequest<'a> {
    /// Channel to unarchive
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnarchiveResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<UnarchiveResponse, UnarchiveError<E>>> for UnarchiveResponse {
    fn into(self) -> Result<UnarchiveResponse, UnarchiveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("")
                    .into())
        }
    }
}
#[derive(Debug)]
pub enum UnarchiveError<E: Error> {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel is not archived.
    NotArchived,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
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
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UnarchiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => UnarchiveError::ChannelNotFound,
            "not_archived" => UnarchiveError::NotArchived,
            "not_authed" => UnarchiveError::NotAuthed,
            "invalid_auth" => UnarchiveError::InvalidAuth,
            "account_inactive" => UnarchiveError::AccountInactive,
            "user_is_bot" => UnarchiveError::UserIsBot,
            "user_is_restricted" => UnarchiveError::UserIsRestricted,
            "invalid_arg_name" => UnarchiveError::InvalidArgName,
            "invalid_array_arg" => UnarchiveError::InvalidArrayArg,
            "invalid_charset" => UnarchiveError::InvalidCharset,
            "invalid_form_data" => UnarchiveError::InvalidFormData,
            "invalid_post_type" => UnarchiveError::InvalidPostType,
            "missing_post_type" => UnarchiveError::MissingPostType,
            "request_timeout" => UnarchiveError::RequestTimeout,
            _ => UnarchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnarchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for UnarchiveError<E> {
    fn description(&self) -> &str {
        match self {
            &UnarchiveError::ChannelNotFound => "channel_not_found: Value passed for channel was invalid.",
            &UnarchiveError::NotArchived => "not_archived: Channel is not archived.",
            &UnarchiveError::NotAuthed => "not_authed: No authentication token provided.",
            &UnarchiveError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            &UnarchiveError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
            &UnarchiveError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
            &UnarchiveError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
            &UnarchiveError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
            &UnarchiveError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
            &UnarchiveError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
            &UnarchiveError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
            &UnarchiveError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.",
            &UnarchiveError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
            &UnarchiveError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
            &UnarchiveError::MalformedResponse(ref e) => e.description(),
            &UnarchiveError::Unknown(ref s) => s,
            &UnarchiveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &UnarchiveError::MalformedResponse(ref e) => Some(e),
            &UnarchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
