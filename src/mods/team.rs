

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

use requests::SlackWebRequestSender;

/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs

pub fn access_logs<R>(
    client: &R,
    token: &str,
    request: &AccessLogsRequest,
) -> Result<AccessLogsResponse, AccessLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let before = request.before.map(|before| before.to_string());
    let params = vec![
        Some(("token", token)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
        before.as_ref().map(|before| ("before", &before[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("team.accessLogs");
    client
        .send(&url, &params[..])
        .map_err(AccessLogsError::Client)
        .and_then(|result| {
            serde_json::from_str::<AccessLogsResponse>(&result)
                .map_err(AccessLogsError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct AccessLogsRequest {
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
    /// End of time range of logs to include in results (inclusive).
    pub before: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessLogsResponse {
    error: Option<String>,
    pub logins: Option<Vec<AccessLogsResponseLogin>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<::Paging>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessLogsResponseLogin {
    pub count: Option<i32>,
    pub country: Option<String>,
    pub date_first: Option<f32>,
    pub date_last: Option<f32>,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub region: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}


impl<E: Error> Into<Result<AccessLogsResponse, AccessLogsError<E>>> for AccessLogsResponse {
    fn into(self) -> Result<AccessLogsResponse, AccessLogsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum AccessLogsError<E: Error> {
    /// This is only available to paid teams.
    PaidOnly,
    /// It is not possible to request more than 1000 items per page or more than 100 pages.
    OverPaginationLimit,
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AccessLogsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "paid_only" => AccessLogsError::PaidOnly,
            "over_pagination_limit" => AccessLogsError::OverPaginationLimit,
            "not_authed" => AccessLogsError::NotAuthed,
            "invalid_auth" => AccessLogsError::InvalidAuth,
            "account_inactive" => AccessLogsError::AccountInactive,
            "user_is_bot" => AccessLogsError::UserIsBot,
            "invalid_arg_name" => AccessLogsError::InvalidArgName,
            "invalid_array_arg" => AccessLogsError::InvalidArrayArg,
            "invalid_charset" => AccessLogsError::InvalidCharset,
            "invalid_form_data" => AccessLogsError::InvalidFormData,
            "invalid_post_type" => AccessLogsError::InvalidPostType,
            "missing_post_type" => AccessLogsError::MissingPostType,
            "team_added_to_org" => AccessLogsError::TeamAddedToOrg,
            "request_timeout" => AccessLogsError::RequestTimeout,
            _ => AccessLogsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AccessLogsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for AccessLogsError<E> {
    fn description(&self) -> &str {
        match *self {
            AccessLogsError::PaidOnly => "paid_only: This is only available to paid teams.",
            AccessLogsError::OverPaginationLimit => {
                "over_pagination_limit: It is not possible to request more than 1000 items per page or more than 100 pages."
            }
            AccessLogsError::NotAuthed => "not_authed: No authentication token provided.",
            AccessLogsError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            AccessLogsError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            AccessLogsError::UserIsBot => {
                "user_is_bot: This method cannot be called by a bot user."
            }
            AccessLogsError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            AccessLogsError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            AccessLogsError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            AccessLogsError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            AccessLogsError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            AccessLogsError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            AccessLogsError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            AccessLogsError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            AccessLogsError::MalformedResponse(ref e) => e.description(),
            AccessLogsError::Unknown(ref s) => s,
            AccessLogsError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AccessLogsError::MalformedResponse(ref e) => Some(e),
            AccessLogsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets billable users information for the current team.
///
/// Wraps https://api.slack.com/methods/team.billableInfo

pub fn billable_info<R>(
    client: &R,
    token: &str,
    request: &BillableInfoRequest,
) -> Result<BillableInfoResponse, BillableInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{

    let params = vec![
        Some(("token", token)),
        request.user.map(|user| ("user", user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("team.billableInfo");
    client
        .send(&url, &params[..])
        .map_err(BillableInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<BillableInfoResponse>(&result)
                .map_err(BillableInfoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct BillableInfoRequest<'a> {
    /// A user to retrieve the billable information for. Defaults to all users.
    pub user: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BillableInfoResponse {
    pub billable_info: Option<HashMap<String, bool>>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}


impl<E: Error> Into<Result<BillableInfoResponse, BillableInfoError<E>>> for BillableInfoResponse {
    fn into(self) -> Result<BillableInfoResponse, BillableInfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum BillableInfoError<E: Error> {
    /// Unable to find the requested user.
    UserNotFound,
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for BillableInfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => BillableInfoError::UserNotFound,
            "not_authed" => BillableInfoError::NotAuthed,
            "invalid_auth" => BillableInfoError::InvalidAuth,
            "account_inactive" => BillableInfoError::AccountInactive,
            "user_is_bot" => BillableInfoError::UserIsBot,
            "invalid_arg_name" => BillableInfoError::InvalidArgName,
            "invalid_array_arg" => BillableInfoError::InvalidArrayArg,
            "invalid_charset" => BillableInfoError::InvalidCharset,
            "invalid_form_data" => BillableInfoError::InvalidFormData,
            "invalid_post_type" => BillableInfoError::InvalidPostType,
            "missing_post_type" => BillableInfoError::MissingPostType,
            "team_added_to_org" => BillableInfoError::TeamAddedToOrg,
            "request_timeout" => BillableInfoError::RequestTimeout,
            _ => BillableInfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for BillableInfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for BillableInfoError<E> {
    fn description(&self) -> &str {
        match *self {
            BillableInfoError::UserNotFound => "user_not_found: Unable to find the requested user.",
            BillableInfoError::NotAuthed => "not_authed: No authentication token provided.",
            BillableInfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            BillableInfoError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            BillableInfoError::UserIsBot => {
                "user_is_bot: This method cannot be called by a bot user."
            }
            BillableInfoError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            BillableInfoError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            BillableInfoError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            BillableInfoError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            BillableInfoError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            BillableInfoError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            BillableInfoError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            BillableInfoError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            BillableInfoError::MalformedResponse(ref e) => e.description(),
            BillableInfoError::Unknown(ref s) => s,
            BillableInfoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BillableInfoError::MalformedResponse(ref e) => Some(e),
            BillableInfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets information about the current team.
///
/// Wraps https://api.slack.com/methods/team.info

pub fn info<R>(client: &R, token: &str) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = ::get_slack_url_for_method("team.info");
    client
        .send(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(InfoError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub team: Option<::Team>,
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
#[derive(Debug)]
pub enum InfoError<E: Error> {
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
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
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
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
        match *self {
            InfoError::NotAuthed => "not_authed: No authentication token provided.",
            InfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            InfoError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            InfoError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            InfoError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            InfoError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            InfoError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            InfoError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            InfoError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            InfoError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            InfoError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            InfoError::MalformedResponse(ref e) => e.description(),
            InfoError::Unknown(ref s) => s,
            InfoError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InfoError::MalformedResponse(ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Gets the integration logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.integrationLogs

pub fn integration_logs<R>(
    client: &R,
    token: &str,
    request: &IntegrationLogsRequest,
) -> Result<IntegrationLogsResponse, IntegrationLogsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        request.service_id.map(
            |service_id| ("service_id", service_id)
        ),
        request.app_id.map(|app_id| ("app_id", app_id)),
        request.user.map(|user| ("user", user)),
        request.change_type.map(|change_type| {
            ("change_type", change_type)
        }),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = ::get_slack_url_for_method("team.integrationLogs");
    client
        .send(&url, &params[..])
        .map_err(IntegrationLogsError::Client)
        .and_then(|result| {
            serde_json::from_str::<IntegrationLogsResponse>(&result)
                .map_err(IntegrationLogsError::MalformedResponse)
        })
        .and_then(|o| o.into())
}

#[derive(Clone, Default, Debug)]
pub struct IntegrationLogsRequest<'a> {
    /// Filter logs to this service. Defaults to all logs.
    pub service_id: Option<&'a str>,
    /// Filter logs to this Slack app. Defaults to all logs.
    pub app_id: Option<&'a str>,
    /// Filter logs generated by this user’s actions. Defaults to all logs.
    pub user: Option<&'a str>,
    /// Filter logs with this change type. Defaults to all logs.
    pub change_type: Option<&'a str>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationLogsResponse {
    error: Option<String>,
    pub logs: Option<Vec<IntegrationLogsResponseLog>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<::Paging>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationLogsResponseLog {
    pub app_id: Option<String>,
    pub app_type: Option<String>,
    pub change_type: Option<String>,
    pub channel: Option<String>,
    pub date: Option<String>,
    pub reason: Option<String>,
    pub scope: Option<String>,
    pub service_id: Option<String>,
    pub service_type: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}


impl<E: Error> Into<Result<IntegrationLogsResponse, IntegrationLogsError<E>>>
    for IntegrationLogsResponse {
    fn into(self) -> Result<IntegrationLogsResponse, IntegrationLogsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum IntegrationLogsError<E: Error> {
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for IntegrationLogsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => IntegrationLogsError::NotAuthed,
            "invalid_auth" => IntegrationLogsError::InvalidAuth,
            "account_inactive" => IntegrationLogsError::AccountInactive,
            "user_is_bot" => IntegrationLogsError::UserIsBot,
            "invalid_arg_name" => IntegrationLogsError::InvalidArgName,
            "invalid_array_arg" => IntegrationLogsError::InvalidArrayArg,
            "invalid_charset" => IntegrationLogsError::InvalidCharset,
            "invalid_form_data" => IntegrationLogsError::InvalidFormData,
            "invalid_post_type" => IntegrationLogsError::InvalidPostType,
            "missing_post_type" => IntegrationLogsError::MissingPostType,
            "team_added_to_org" => IntegrationLogsError::TeamAddedToOrg,
            "request_timeout" => IntegrationLogsError::RequestTimeout,
            _ => IntegrationLogsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for IntegrationLogsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for IntegrationLogsError<E> {
    fn description(&self) -> &str {
        match *self {
            IntegrationLogsError::NotAuthed => "not_authed: No authentication token provided.",
            IntegrationLogsError::InvalidAuth => "invalid_auth: Invalid authentication token.",
            IntegrationLogsError::AccountInactive => {
                "account_inactive: Authentication token is for a deleted user or team."
            }
            IntegrationLogsError::UserIsBot => {
                "user_is_bot: This method cannot be called by a bot user."
            }
            IntegrationLogsError::InvalidArgName => {
                "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call."
            }
            IntegrationLogsError::InvalidArrayArg => {
                "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API."
            }
            IntegrationLogsError::InvalidCharset => {
                "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1."
            }
            IntegrationLogsError::InvalidFormData => {
                "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid."
            }
            IntegrationLogsError::InvalidPostType => {
                "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain."
            }
            IntegrationLogsError::MissingPostType => {
                "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header."
            }
            IntegrationLogsError::TeamAddedToOrg => {
                "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete."
            }
            IntegrationLogsError::RequestTimeout => {
                "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated."
            }
            IntegrationLogsError::MalformedResponse(ref e) => e.description(),
            IntegrationLogsError::Unknown(ref s) => s,
            IntegrationLogsError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            IntegrationLogsError::MalformedResponse(ref e) => Some(e),
            IntegrationLogsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
