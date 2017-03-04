
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

pub fn archive<R>(client: &R, request: &ArchiveRequest) -> Result<ArchiveResponse, ArchiveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.archive", &params[..])
        .map_err(|err| ArchiveError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|_| ArchiveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ArchiveRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &ArchiveError::ChannelNotFound => "channel_not_found",
            &ArchiveError::AlreadyArchived => "already_archived",
            &ArchiveError::CantArchiveGeneral => "cant_archive_general",
            &ArchiveError::LastRaChannel => "last_ra_channel",
            &ArchiveError::RestrictedAction => "restricted_action",
            &ArchiveError::NotAuthed => "not_authed",
            &ArchiveError::InvalidAuth => "invalid_auth",
            &ArchiveError::AccountInactive => "account_inactive",
            &ArchiveError::UserIsBot => "user_is_bot",
            &ArchiveError::UserIsRestricted => "user_is_restricted",
            &ArchiveError::InvalidArgName => "invalid_arg_name",
            &ArchiveError::InvalidArrayArg => "invalid_array_arg",
            &ArchiveError::InvalidCharset => "invalid_charset",
            &ArchiveError::InvalidFormData => "invalid_form_data",
            &ArchiveError::InvalidPostType => "invalid_post_type",
            &ArchiveError::MissingPostType => "missing_post_type",
            &ArchiveError::RequestTimeout => "request_timeout",
            &ArchiveError::MalformedResponse => "Malformed response data from Slack.",
            &ArchiveError::Unknown(ref s) => s,
            &ArchiveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ArchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create

pub fn create<R>(client: &R, request: &CreateRequest) -> Result<CreateResponse, CreateError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.create", &params[..])
        .map_err(|err| CreateError::Client(err))
        .and_then(|result| serde_json::from_str::<CreateResponse>(&result).map_err(|_| CreateError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct CreateRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &CreateError::NameTaken => "name_taken",
            &CreateError::RestrictedAction => "restricted_action",
            &CreateError::NoChannel => "no_channel",
            &CreateError::NotAuthed => "not_authed",
            &CreateError::InvalidAuth => "invalid_auth",
            &CreateError::AccountInactive => "account_inactive",
            &CreateError::UserIsBot => "user_is_bot",
            &CreateError::UserIsRestricted => "user_is_restricted",
            &CreateError::InvalidArgName => "invalid_arg_name",
            &CreateError::InvalidArrayArg => "invalid_array_arg",
            &CreateError::InvalidCharset => "invalid_charset",
            &CreateError::InvalidFormData => "invalid_form_data",
            &CreateError::InvalidPostType => "invalid_post_type",
            &CreateError::MissingPostType => "missing_post_type",
            &CreateError::RequestTimeout => "request_timeout",
            &CreateError::MalformedResponse => "Malformed response data from Slack.",
            &CreateError::Unknown(ref s) => s,
            &CreateError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &CreateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history

pub fn history<R>(client: &R, request: &HistoryRequest) -> Result<HistoryResponse, HistoryError<R::Error>>
    where R: SlackWebRequestSender
{
    let count = request.count.map(|count| count.to_string());
    let params = vec![Some(("token", request.token)),
                      Some(("channel", request.channel)),
                      request.latest.map(|latest| ("latest", latest)),
                      request.oldest.map(|oldest| ("oldest", oldest)),
                      request.inclusive
                          .map(|inclusive| ("inclusive", if inclusive { "1" } else { "0" })),
                      count.as_ref().map(|count| ("count", &count[..])),
                      request.unreads.map(|unreads| ("unreads", if unreads { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.history", &params[..])
        .map_err(|err| HistoryError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(|_| HistoryError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct HistoryRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:history
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &HistoryError::ChannelNotFound => "channel_not_found",
            &HistoryError::InvalidTsLatest => "invalid_ts_latest",
            &HistoryError::InvalidTsOldest => "invalid_ts_oldest",
            &HistoryError::NotAuthed => "not_authed",
            &HistoryError::InvalidAuth => "invalid_auth",
            &HistoryError::AccountInactive => "account_inactive",
            &HistoryError::InvalidArgName => "invalid_arg_name",
            &HistoryError::InvalidArrayArg => "invalid_array_arg",
            &HistoryError::InvalidCharset => "invalid_charset",
            &HistoryError::InvalidFormData => "invalid_form_data",
            &HistoryError::InvalidPostType => "invalid_post_type",
            &HistoryError::MissingPostType => "missing_post_type",
            &HistoryError::RequestTimeout => "request_timeout",
            &HistoryError::MalformedResponse => "Malformed response data from Slack.",
            &HistoryError::Unknown(ref s) => s,
            &HistoryError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &HistoryError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info

pub fn info<R>(client: &R, request: &InfoRequest) -> Result<InfoResponse, InfoError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.info", &params[..])
        .map_err(|err| InfoError::Client(err))
        .and_then(|result| serde_json::from_str::<InfoResponse>(&result).map_err(|_| InfoError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:read
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &InfoError::ChannelNotFound => "channel_not_found",
            &InfoError::NotAuthed => "not_authed",
            &InfoError::InvalidAuth => "invalid_auth",
            &InfoError::AccountInactive => "account_inactive",
            &InfoError::InvalidArgName => "invalid_arg_name",
            &InfoError::InvalidArrayArg => "invalid_array_arg",
            &InfoError::InvalidCharset => "invalid_charset",
            &InfoError::InvalidFormData => "invalid_form_data",
            &InfoError::InvalidPostType => "invalid_post_type",
            &InfoError::MissingPostType => "missing_post_type",
            &InfoError::RequestTimeout => "request_timeout",
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

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite

pub fn invite<R>(client: &R, request: &InviteRequest) -> Result<InviteResponse, InviteError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.invite", &params[..])
        .map_err(|err| InviteError::Client(err))
        .and_then(|result| serde_json::from_str::<InviteResponse>(&result).map_err(|_| InviteError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct InviteRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &InviteError::ChannelNotFound => "channel_not_found",
            &InviteError::UserNotFound => "user_not_found",
            &InviteError::CantInviteSelf => "cant_invite_self",
            &InviteError::NotInChannel => "not_in_channel",
            &InviteError::AlreadyInChannel => "already_in_channel",
            &InviteError::IsArchived => "is_archived",
            &InviteError::CantInvite => "cant_invite",
            &InviteError::NotAuthed => "not_authed",
            &InviteError::InvalidAuth => "invalid_auth",
            &InviteError::AccountInactive => "account_inactive",
            &InviteError::UserIsBot => "user_is_bot",
            &InviteError::UserIsUltraRestricted => "user_is_ultra_restricted",
            &InviteError::InvalidArgName => "invalid_arg_name",
            &InviteError::InvalidArrayArg => "invalid_array_arg",
            &InviteError::InvalidCharset => "invalid_charset",
            &InviteError::InvalidFormData => "invalid_form_data",
            &InviteError::InvalidPostType => "invalid_post_type",
            &InviteError::MissingPostType => "missing_post_type",
            &InviteError::RequestTimeout => "request_timeout",
            &InviteError::MalformedResponse => "Malformed response data from Slack.",
            &InviteError::Unknown(ref s) => s,
            &InviteError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &InviteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join

pub fn join<R>(client: &R, request: &JoinRequest) -> Result<JoinResponse, JoinError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.join", &params[..])
        .map_err(|err| JoinError::Client(err))
        .and_then(|result| serde_json::from_str::<JoinResponse>(&result).map_err(|_| JoinError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct JoinRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &JoinError::ChannelNotFound => "channel_not_found",
            &JoinError::NameTaken => "name_taken",
            &JoinError::RestrictedAction => "restricted_action",
            &JoinError::NoChannel => "no_channel",
            &JoinError::IsArchived => "is_archived",
            &JoinError::NotAuthed => "not_authed",
            &JoinError::InvalidAuth => "invalid_auth",
            &JoinError::AccountInactive => "account_inactive",
            &JoinError::UserIsBot => "user_is_bot",
            &JoinError::UserIsRestricted => "user_is_restricted",
            &JoinError::InvalidArgName => "invalid_arg_name",
            &JoinError::InvalidArrayArg => "invalid_array_arg",
            &JoinError::InvalidCharset => "invalid_charset",
            &JoinError::InvalidFormData => "invalid_form_data",
            &JoinError::InvalidPostType => "invalid_post_type",
            &JoinError::MissingPostType => "missing_post_type",
            &JoinError::RequestTimeout => "request_timeout",
            &JoinError::MalformedResponse => "Malformed response data from Slack.",
            &JoinError::Unknown(ref s) => s,
            &JoinError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &JoinError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick

pub fn kick<R>(client: &R, request: &KickRequest) -> Result<KickResponse, KickError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("user", request.user))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.kick", &params[..])
        .map_err(|err| KickError::Client(err))
        .and_then(|result| serde_json::from_str::<KickResponse>(&result).map_err(|_| KickError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct KickRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &KickError::ChannelNotFound => "channel_not_found",
            &KickError::UserNotFound => "user_not_found",
            &KickError::CantKickSelf => "cant_kick_self",
            &KickError::NotInChannel => "not_in_channel",
            &KickError::CantKickFromGeneral => "cant_kick_from_general",
            &KickError::CantKickFromLastChannel => "cant_kick_from_last_channel",
            &KickError::RestrictedAction => "restricted_action",
            &KickError::NotAuthed => "not_authed",
            &KickError::InvalidAuth => "invalid_auth",
            &KickError::AccountInactive => "account_inactive",
            &KickError::UserIsBot => "user_is_bot",
            &KickError::UserIsRestricted => "user_is_restricted",
            &KickError::InvalidArgName => "invalid_arg_name",
            &KickError::InvalidArrayArg => "invalid_array_arg",
            &KickError::InvalidCharset => "invalid_charset",
            &KickError::InvalidFormData => "invalid_form_data",
            &KickError::InvalidPostType => "invalid_post_type",
            &KickError::MissingPostType => "missing_post_type",
            &KickError::RequestTimeout => "request_timeout",
            &KickError::MalformedResponse => "Malformed response data from Slack.",
            &KickError::Unknown(ref s) => s,
            &KickError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &KickError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave

pub fn leave<R>(client: &R, request: &LeaveRequest) -> Result<LeaveResponse, LeaveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.leave", &params[..])
        .map_err(|err| LeaveError::Client(err))
        .and_then(|result| serde_json::from_str::<LeaveResponse>(&result).map_err(|_| LeaveError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct LeaveRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &LeaveError::ChannelNotFound => "channel_not_found",
            &LeaveError::IsArchived => "is_archived",
            &LeaveError::CantLeaveGeneral => "cant_leave_general",
            &LeaveError::NotAuthed => "not_authed",
            &LeaveError::InvalidAuth => "invalid_auth",
            &LeaveError::AccountInactive => "account_inactive",
            &LeaveError::UserIsBot => "user_is_bot",
            &LeaveError::UserIsRestricted => "user_is_restricted",
            &LeaveError::InvalidArgName => "invalid_arg_name",
            &LeaveError::InvalidArrayArg => "invalid_array_arg",
            &LeaveError::InvalidCharset => "invalid_charset",
            &LeaveError::InvalidFormData => "invalid_form_data",
            &LeaveError::InvalidPostType => "invalid_post_type",
            &LeaveError::MissingPostType => "missing_post_type",
            &LeaveError::RequestTimeout => "request_timeout",
            &LeaveError::MalformedResponse => "Malformed response data from Slack.",
            &LeaveError::Unknown(ref s) => s,
            &LeaveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LeaveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      request.exclude_archived
                          .map(|exclude_archived| ("exclude_archived", if exclude_archived { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.list", &params[..])
        .map_err(|err| ListError::Client(err))
        .and_then(|result| serde_json::from_str::<ListResponse>(&result).map_err(|_| ListError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:read
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &ListError::NotAuthed => "not_authed",
            &ListError::InvalidAuth => "invalid_auth",
            &ListError::AccountInactive => "account_inactive",
            &ListError::InvalidArgName => "invalid_arg_name",
            &ListError::InvalidArrayArg => "invalid_array_arg",
            &ListError::InvalidCharset => "invalid_charset",
            &ListError::InvalidFormData => "invalid_form_data",
            &ListError::InvalidPostType => "invalid_post_type",
            &ListError::MissingPostType => "missing_post_type",
            &ListError::RequestTimeout => "request_timeout",
            &ListError::MalformedResponse => "Malformed response data from Slack.",
            &ListError::Unknown(ref s) => s,
            &ListError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/channels.mark

pub fn mark<R>(client: &R, request: &MarkRequest) -> Result<MarkResponse, MarkError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("ts", request.ts))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.mark", &params[..])
        .map_err(|err| MarkError::Client(err))
        .and_then(|result| serde_json::from_str::<MarkResponse>(&result).map_err(|_| MarkError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct MarkRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &MarkError::ChannelNotFound => "channel_not_found",
            &MarkError::InvalidTimestamp => "invalid_timestamp",
            &MarkError::NotInChannel => "not_in_channel",
            &MarkError::NotAuthed => "not_authed",
            &MarkError::InvalidAuth => "invalid_auth",
            &MarkError::AccountInactive => "account_inactive",
            &MarkError::InvalidArgName => "invalid_arg_name",
            &MarkError::InvalidArrayArg => "invalid_array_arg",
            &MarkError::InvalidCharset => "invalid_charset",
            &MarkError::InvalidFormData => "invalid_form_data",
            &MarkError::InvalidPostType => "invalid_post_type",
            &MarkError::MissingPostType => "missing_post_type",
            &MarkError::RequestTimeout => "request_timeout",
            &MarkError::MalformedResponse => "Malformed response data from Slack.",
            &MarkError::Unknown(ref s) => s,
            &MarkError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &MarkError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename

pub fn rename<R>(client: &R, request: &RenameRequest) -> Result<RenameResponse, RenameError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("name", request.name))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.rename", &params[..])
        .map_err(|err| RenameError::Client(err))
        .and_then(|result| serde_json::from_str::<RenameResponse>(&result).map_err(|_| RenameError::MalformedResponse))
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RenameRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &RenameError::ChannelNotFound => "channel_not_found",
            &RenameError::NotInChannel => "not_in_channel",
            &RenameError::NotAuthorized => "not_authorized",
            &RenameError::InvalidName => "invalid_name",
            &RenameError::NameTaken => "name_taken",
            &RenameError::NotAuthed => "not_authed",
            &RenameError::InvalidAuth => "invalid_auth",
            &RenameError::AccountInactive => "account_inactive",
            &RenameError::UserIsBot => "user_is_bot",
            &RenameError::UserIsRestricted => "user_is_restricted",
            &RenameError::InvalidArgName => "invalid_arg_name",
            &RenameError::InvalidArrayArg => "invalid_array_arg",
            &RenameError::InvalidCharset => "invalid_charset",
            &RenameError::InvalidFormData => "invalid_form_data",
            &RenameError::InvalidPostType => "invalid_post_type",
            &RenameError::MissingPostType => "missing_post_type",
            &RenameError::RequestTimeout => "request_timeout",
            &RenameError::MalformedResponse => "Malformed response data from Slack.",
            &RenameError::Unknown(ref s) => s,
            &RenameError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RenameError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Retrieve a thread of messages posted to a channel
///
/// Wraps https://api.slack.com/methods/channels.replies

pub fn replies<R>(client: &R, request: &RepliesRequest) -> Result<RepliesResponse, RepliesError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)),
                      Some(("channel", request.channel)),
                      Some(("thread_ts", request.thread_ts))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.replies", &params[..])
        .map_err(|err| RepliesError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(|_| RepliesError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct RepliesRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:history
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &RepliesError::ChannelNotFound => "channel_not_found",
            &RepliesError::ThreadNotFound => "thread_not_found",
            &RepliesError::NotAuthed => "not_authed",
            &RepliesError::InvalidAuth => "invalid_auth",
            &RepliesError::AccountInactive => "account_inactive",
            &RepliesError::InvalidArgName => "invalid_arg_name",
            &RepliesError::InvalidArrayArg => "invalid_array_arg",
            &RepliesError::InvalidCharset => "invalid_charset",
            &RepliesError::InvalidFormData => "invalid_form_data",
            &RepliesError::InvalidPostType => "invalid_post_type",
            &RepliesError::MissingPostType => "missing_post_type",
            &RepliesError::RequestTimeout => "request_timeout",
            &RepliesError::MalformedResponse => "Malformed response data from Slack.",
            &RepliesError::Unknown(ref s) => s,
            &RepliesError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RepliesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose

pub fn set_purpose<R>(client: &R, request: &SetPurposeRequest) -> Result<SetPurposeResponse, SetPurposeError<R::Error>>
    where R: SlackWebRequestSender
{

    let params =
        vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("purpose", request.purpose))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.setPurpose", &params[..])
        .map_err(|err| SetPurposeError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result).map_err(|_| SetPurposeError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetPurposeRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &SetPurposeError::ChannelNotFound => "channel_not_found",
            &SetPurposeError::NotInChannel => "not_in_channel",
            &SetPurposeError::IsArchived => "is_archived",
            &SetPurposeError::TooLong => "too_long",
            &SetPurposeError::UserIsRestricted => "user_is_restricted",
            &SetPurposeError::NotAuthed => "not_authed",
            &SetPurposeError::InvalidAuth => "invalid_auth",
            &SetPurposeError::AccountInactive => "account_inactive",
            &SetPurposeError::InvalidArgName => "invalid_arg_name",
            &SetPurposeError::InvalidArrayArg => "invalid_array_arg",
            &SetPurposeError::InvalidCharset => "invalid_charset",
            &SetPurposeError::InvalidFormData => "invalid_form_data",
            &SetPurposeError::InvalidPostType => "invalid_post_type",
            &SetPurposeError::MissingPostType => "missing_post_type",
            &SetPurposeError::RequestTimeout => "request_timeout",
            &SetPurposeError::MalformedResponse => "Malformed response data from Slack.",
            &SetPurposeError::Unknown(ref s) => s,
            &SetPurposeError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetPurposeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic

pub fn set_topic<R>(client: &R, request: &SetTopicRequest) -> Result<SetTopicResponse, SetTopicError<R::Error>>
    where R: SlackWebRequestSender
{

    let params =
        vec![Some(("token", request.token)), Some(("channel", request.channel)), Some(("topic", request.topic))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.setTopic", &params[..])
        .map_err(|err| SetTopicError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result).map_err(|_| SetTopicError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct SetTopicRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &SetTopicError::ChannelNotFound => "channel_not_found",
            &SetTopicError::NotInChannel => "not_in_channel",
            &SetTopicError::IsArchived => "is_archived",
            &SetTopicError::TooLong => "too_long",
            &SetTopicError::UserIsRestricted => "user_is_restricted",
            &SetTopicError::NotAuthed => "not_authed",
            &SetTopicError::InvalidAuth => "invalid_auth",
            &SetTopicError::AccountInactive => "account_inactive",
            &SetTopicError::InvalidArgName => "invalid_arg_name",
            &SetTopicError::InvalidArrayArg => "invalid_array_arg",
            &SetTopicError::InvalidCharset => "invalid_charset",
            &SetTopicError::InvalidFormData => "invalid_form_data",
            &SetTopicError::InvalidPostType => "invalid_post_type",
            &SetTopicError::MissingPostType => "missing_post_type",
            &SetTopicError::RequestTimeout => "request_timeout",
            &SetTopicError::MalformedResponse => "Malformed response data from Slack.",
            &SetTopicError::Unknown(ref s) => s,
            &SetTopicError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SetTopicError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive

pub fn unarchive<R>(client: &R, request: &UnarchiveRequest) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("channels.unarchive", &params[..])
        .map_err(|err| UnarchiveError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result).map_err(|_| UnarchiveError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct UnarchiveRequest<'a> {
    /// Authentication token.
    /// Requires scope: channels:write
    pub token: &'a str,
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
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
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
    MalformedResponse,
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
            &UnarchiveError::ChannelNotFound => "channel_not_found",
            &UnarchiveError::NotArchived => "not_archived",
            &UnarchiveError::NotAuthed => "not_authed",
            &UnarchiveError::InvalidAuth => "invalid_auth",
            &UnarchiveError::AccountInactive => "account_inactive",
            &UnarchiveError::UserIsBot => "user_is_bot",
            &UnarchiveError::UserIsRestricted => "user_is_restricted",
            &UnarchiveError::InvalidArgName => "invalid_arg_name",
            &UnarchiveError::InvalidArrayArg => "invalid_array_arg",
            &UnarchiveError::InvalidCharset => "invalid_charset",
            &UnarchiveError::InvalidFormData => "invalid_form_data",
            &UnarchiveError::InvalidPostType => "invalid_post_type",
            &UnarchiveError::MissingPostType => "missing_post_type",
            &UnarchiveError::RequestTimeout => "request_timeout",
            &UnarchiveError::MalformedResponse => "Malformed response data from Slack.",
            &UnarchiveError::Unknown(ref s) => s,
            &UnarchiveError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &UnarchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
