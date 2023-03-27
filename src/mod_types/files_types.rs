//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

//! Get info on files uploaded to Slack, upload new files to Slack.

#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// ID of file to delete.
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> From<DeleteResponse> for Result<DeleteResponse, DeleteError<E>> {
    fn from(val: DeleteResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum DeleteError<E: Error> {
    /// The file does not exist, or is not visible to the calling user.
    FileNotFound,
    /// The file has already been deleted.
    FileDeleted,
    /// Authenticated user does not have permission to delete this file.
    CantDeleteFile,
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
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DeleteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => DeleteError::FileNotFound,
            "file_deleted" => DeleteError::FileDeleted,
            "cant_delete_file" => DeleteError::CantDeleteFile,
            "not_authed" => DeleteError::NotAuthed,
            "invalid_auth" => DeleteError::InvalidAuth,
            "account_inactive" => DeleteError::AccountInactive,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "missing_post_type" => DeleteError::MissingPostType,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "request_timeout" => DeleteError::RequestTimeout,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        DeleteError::FileNotFound => "file_not_found: The file does not exist, or is not visible to the calling user.",
DeleteError::FileDeleted => "file_deleted: The file has already been deleted.",
DeleteError::CantDeleteFile => "cant_delete_file: Authenticated user does not have permission to delete this file.",
DeleteError::NotAuthed => "not_authed: No authentication token provided.",
DeleteError::InvalidAuth => "invalid_auth: Invalid authentication token.",
DeleteError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
DeleteError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
DeleteError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
DeleteError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
DeleteError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
DeleteError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
DeleteError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
DeleteError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
DeleteError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        DeleteError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        DeleteError::Unknown(ref s) => return write!(f, "{}", s),
                        DeleteError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for DeleteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeleteError::MalformedResponse(_, ref e) => Some(e),
            DeleteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Specify a file by providing its ID.
    pub file: &'a str,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub comments: Option<Vec<crate::FileComment>>,
    error: Option<String>,
    pub file: Option<crate::File>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<crate::Paging>,
}

impl<E: Error> From<InfoResponse> for Result<InfoResponse, InfoError<E>> {
    fn from(val: InfoResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum InfoError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
    /// The requested file has been deleted
    FileDeleted,
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
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => InfoError::FileNotFound,
            "file_deleted" => InfoError::FileDeleted,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        InfoError::FileNotFound => "file_not_found: Value passed for file was invalid",
InfoError::FileDeleted => "file_deleted: The requested file has been deleted",
InfoError::NotAuthed => "not_authed: No authentication token provided.",
InfoError::InvalidAuth => "invalid_auth: Invalid authentication token.",
InfoError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
InfoError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
InfoError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
InfoError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
InfoError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
InfoError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
InfoError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
InfoError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
InfoError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        InfoError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        InfoError::Unknown(ref s) => return write!(f, "{}", s),
                        InfoError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for InfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InfoError::MalformedResponse(_, ref e) => Some(e),
            InfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Filter files created by a single user.
    pub user: Option<&'a str>,
    /// Filter files appearing in a specific channel, indicated by its ID.
    pub channel: Option<&'a str>,
    /// Filter files created after this timestamp (inclusive).
    pub ts_from: Option<u32>,
    /// Filter files created before this timestamp (inclusive).
    pub ts_to: Option<u32>,
    /// Filter files by type:
    ///
    ///
    /// all - All files
    /// spaces - Posts
    /// snippets - Snippets
    /// images - Image files
    /// gdocs - Google docs
    /// zips - Zip files
    /// pdfs - PDF files
    ///
    ///
    /// You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.
    pub types: Option<&'a str>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    error: Option<String>,
    pub files: Option<Vec<crate::File>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<crate::Paging>,
}

impl<E: Error> From<ListResponse> for Result<ListResponse, ListError<E>> {
    fn from(val: ListResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum ListError<E: Error> {
    /// Value passed for user was invalid
    UserNotFound,
    /// Value passed for types was invalid
    UnknownType,
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
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_not_found" => ListError::UserNotFound,
            "unknown_type" => ListError::UnknownType,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "user_is_bot" => ListError::UserIsBot,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "request_timeout" => ListError::RequestTimeout,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        ListError::UserNotFound => "user_not_found: Value passed for user was invalid",
ListError::UnknownType => "unknown_type: Value passed for types was invalid",
ListError::NotAuthed => "not_authed: No authentication token provided.",
ListError::InvalidAuth => "invalid_auth: Invalid authentication token.",
ListError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
ListError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
ListError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
ListError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
ListError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
ListError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
ListError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
ListError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
ListError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
ListError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        ListError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        ListError::Unknown(ref s) => return write!(f, "{}", s),
                        ListError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for ListError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ListError::MalformedResponse(_, ref e) => Some(e),
            ListError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RevokePublicURLRequest<'a> {
    /// File to revoke
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLResponse {
    error: Option<String>,
    pub file: Option<crate::File>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> From<RevokePublicURLResponse>
    for Result<RevokePublicURLResponse, RevokePublicURLError<E>>
{
    fn from(val: RevokePublicURLResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum RevokePublicURLError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RevokePublicURLError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => RevokePublicURLError::FileNotFound,
            "not_authed" => RevokePublicURLError::NotAuthed,
            "invalid_auth" => RevokePublicURLError::InvalidAuth,
            "account_inactive" => RevokePublicURLError::AccountInactive,
            "user_is_bot" => RevokePublicURLError::UserIsBot,
            "user_is_restricted" => RevokePublicURLError::UserIsRestricted,
            "invalid_arg_name" => RevokePublicURLError::InvalidArgName,
            "invalid_array_arg" => RevokePublicURLError::InvalidArrayArg,
            "invalid_charset" => RevokePublicURLError::InvalidCharset,
            "invalid_form_data" => RevokePublicURLError::InvalidFormData,
            "invalid_post_type" => RevokePublicURLError::InvalidPostType,
            "missing_post_type" => RevokePublicURLError::MissingPostType,
            "team_added_to_org" => RevokePublicURLError::TeamAddedToOrg,
            "request_timeout" => RevokePublicURLError::RequestTimeout,
            _ => RevokePublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokePublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        RevokePublicURLError::FileNotFound => "file_not_found: Value passed for file was invalid",
RevokePublicURLError::NotAuthed => "not_authed: No authentication token provided.",
RevokePublicURLError::InvalidAuth => "invalid_auth: Invalid authentication token.",
RevokePublicURLError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
RevokePublicURLError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
RevokePublicURLError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
RevokePublicURLError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
RevokePublicURLError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
RevokePublicURLError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
RevokePublicURLError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
RevokePublicURLError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
RevokePublicURLError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
RevokePublicURLError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
RevokePublicURLError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        RevokePublicURLError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        RevokePublicURLError::Unknown(ref s) => return write!(f, "{}", s),
                        RevokePublicURLError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for RevokePublicURLError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RevokePublicURLError::MalformedResponse(_, ref e) => Some(e),
            RevokePublicURLError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SharedPublicURLRequest<'a> {
    /// File to share
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLResponse {
    error: Option<String>,
    pub file: Option<crate::File>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> From<SharedPublicURLResponse>
    for Result<SharedPublicURLResponse, SharedPublicURLError<E>>
{
    fn from(val: SharedPublicURLResponse) -> Self {
        if val.ok {
            Ok(val)
        } else {
            Err(val.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Debug)]
pub enum SharedPublicURLError<E: Error> {
    /// Value passed for file was invalid
    FileNotFound,
    /// Public sharing has been disabled for this team
    NotAllowed,
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
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SharedPublicURLError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "file_not_found" => SharedPublicURLError::FileNotFound,
            "not_allowed" => SharedPublicURLError::NotAllowed,
            "not_authed" => SharedPublicURLError::NotAuthed,
            "invalid_auth" => SharedPublicURLError::InvalidAuth,
            "account_inactive" => SharedPublicURLError::AccountInactive,
            "user_is_bot" => SharedPublicURLError::UserIsBot,
            "user_is_restricted" => SharedPublicURLError::UserIsRestricted,
            "invalid_arg_name" => SharedPublicURLError::InvalidArgName,
            "invalid_array_arg" => SharedPublicURLError::InvalidArrayArg,
            "invalid_charset" => SharedPublicURLError::InvalidCharset,
            "invalid_form_data" => SharedPublicURLError::InvalidFormData,
            "invalid_post_type" => SharedPublicURLError::InvalidPostType,
            "missing_post_type" => SharedPublicURLError::MissingPostType,
            "team_added_to_org" => SharedPublicURLError::TeamAddedToOrg,
            "request_timeout" => SharedPublicURLError::RequestTimeout,
            _ => SharedPublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SharedPublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match *self {
                        SharedPublicURLError::FileNotFound => "file_not_found: Value passed for file was invalid",
SharedPublicURLError::NotAllowed => "not_allowed: Public sharing has been disabled for this team",
SharedPublicURLError::NotAuthed => "not_authed: No authentication token provided.",
SharedPublicURLError::InvalidAuth => "invalid_auth: Invalid authentication token.",
SharedPublicURLError::AccountInactive => "account_inactive: Authentication token is for a deleted user or team.",
SharedPublicURLError::UserIsBot => "user_is_bot: This method cannot be called by a bot user.",
SharedPublicURLError::UserIsRestricted => "user_is_restricted: This method cannot be called by a restricted user or single channel guest.",
SharedPublicURLError::InvalidArgName => "invalid_arg_name: The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.",
SharedPublicURLError::InvalidArrayArg => "invalid_array_arg: The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.",
SharedPublicURLError::InvalidCharset => "invalid_charset: The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.",
SharedPublicURLError::InvalidFormData => "invalid_form_data: The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.",
SharedPublicURLError::InvalidPostType => "invalid_post_type: The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.",
SharedPublicURLError::MissingPostType => "missing_post_type: The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.",
SharedPublicURLError::TeamAddedToOrg => "team_added_to_org: The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.",
SharedPublicURLError::RequestTimeout => "request_timeout: The method was called via a POST request, but the POST data was either missing or truncated.",
                        SharedPublicURLError::MalformedResponse(_, ref e) => return write!(f, "{}", e),
                        SharedPublicURLError::Unknown(ref s) => return write!(f, "{}", s),
                        SharedPublicURLError::Client(ref inner) => return write!(f, "{}", inner),
                    };
        write!(f, "{}", d)
    }
}

impl<E: Error + 'static> Error for SharedPublicURLError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SharedPublicURLError::MalformedResponse(_, ref e) => Some(e),
            SharedPublicURLError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
