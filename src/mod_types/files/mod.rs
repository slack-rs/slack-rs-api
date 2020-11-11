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

#![allow(unused_imports)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::blacklisted_name)]

pub mod comments_types;
pub mod remote_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct DeleteRequest<'a> {
    /// ID of file to delete.
    pub file: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeleteResponse, DeleteError<E>>> for DeleteResponse {
    fn into(self) -> Result<DeleteResponse, DeleteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeleteError<E: Error> {
    AccountInactive,
    CantDeleteFile,
    FileDeleted,
    FileNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    UpgradeRequired,
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
            "account_inactive" => DeleteError::AccountInactive,
            "cant_delete_file" => DeleteError::CantDeleteFile,
            "file_deleted" => DeleteError::FileDeleted,
            "file_not_found" => DeleteError::FileNotFound,
            "invalid_arg_name" => DeleteError::InvalidArgName,
            "invalid_array_arg" => DeleteError::InvalidArrayArg,
            "invalid_auth" => DeleteError::InvalidAuth,
            "invalid_charset" => DeleteError::InvalidCharset,
            "invalid_form_data" => DeleteError::InvalidFormData,
            "invalid_json" => DeleteError::InvalidJson,
            "invalid_post_type" => DeleteError::InvalidPostType,
            "json_not_object" => DeleteError::JsonNotObject,
            "missing_post_type" => DeleteError::MissingPostType,
            "no_permission" => DeleteError::NoPermission,
            "not_authed" => DeleteError::NotAuthed,
            "request_timeout" => DeleteError::RequestTimeout,
            "team_added_to_org" => DeleteError::TeamAddedToOrg,
            "upgrade_required" => DeleteError::UpgradeRequired,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            DeleteError::CantDeleteFile => write!(f, "Server returned error cant_delete_file"),
            DeleteError::FileDeleted => write!(f, "Server returned error file_deleted"),
            DeleteError::FileNotFound => write!(f, "Server returned error file_not_found"),
            DeleteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeleteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            DeleteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeleteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeleteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            DeleteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeleteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            DeleteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeleteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            DeleteError::NoPermission => write!(f, "Server returned error no_permission"),
            DeleteError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeleteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeleteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            DeleteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            DeleteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeleteError::Unknown(ref s) => write!(f, "{}", s),
            DeleteError::Client(ref inner) => write!(f, "{}", inner),
        }
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
    pub count: Option<Cow<'a, str>>,
    /// Parameter for pagination. File comments are paginated for a single file. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection of comments. See [pagination](/docs/pagination) for more details.
    pub cursor: Option<Cow<'a, str>>,
    /// Specify a file by providing its ID.
    pub file: Option<Cow<'a, str>>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
    pub limit: Option<u64>,
    pub page: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoFileInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<InfoReactionsInner>>,
    pub shares: Option<InfoSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoPagingInner {
    pub count: Option<u64>,
    pub page: u64,
    pub pages: Option<u64>,
    pub per_page: Option<u64>,
    pub spill: Option<u64>,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    pub comments: Vec<serde_json::Value>,
    pub content_html: Option<serde_json::Value>,
    pub editor: Option<String>,
    error: Option<String>,
    pub file: InfoFileInner,
    #[serde(default)]
    ok: bool,
    pub paging: Option<InfoPagingInner>,
    pub response_metadata: Option<Vec<InfoResponseMetadataInner>>,
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
    AccountInactive,
    FileDeleted,
    FileNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    TimezoneCountFailed,
    UpgradeRequired,
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
            "account_inactive" => InfoError::AccountInactive,
            "file_deleted" => InfoError::FileDeleted,
            "file_not_found" => InfoError::FileNotFound,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_auth" => InfoError::InvalidAuth,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_json" => InfoError::InvalidJson,
            "invalid_post_type" => InfoError::InvalidPostType,
            "json_not_object" => InfoError::JsonNotObject,
            "missing_post_type" => InfoError::MissingPostType,
            "no_permission" => InfoError::NoPermission,
            "not_authed" => InfoError::NotAuthed,
            "request_timeout" => InfoError::RequestTimeout,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "timezone_count_failed" => InfoError::TimezoneCountFailed,
            "upgrade_required" => InfoError::UpgradeRequired,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::FileDeleted => write!(f, "Server returned error file_deleted"),
            InfoError::FileNotFound => write!(f, "Server returned error file_not_found"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::NoPermission => write!(f, "Server returned error no_permission"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InfoError::TimezoneCountFailed => {
                write!(f, "Server returned error timezone_count_failed")
            }
            InfoError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InfoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            InfoError::Unknown(ref s) => write!(f, "{}", s),
            InfoError::Client(ref inner) => write!(f, "{}", inner),
        }
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
    /// Filter files appearing in a specific channel, indicated by its ID.
    pub channel: Option<Cow<'a, str>>,
    pub count: Option<Cow<'a, str>>,
    pub page: Option<Cow<'a, str>>,
    /// Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit.
    pub show_files_hidden_by_limit: Option<bool>,
    /// Filter files created after this timestamp (inclusive).
    pub ts_from: Option<f64>,
    /// Filter files created before this timestamp (inclusive).
    pub ts_to: Option<f64>,
    /// Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like `types=spaces,snippets`.The default value is `all`, which does not filter the list.
    pub types: Option<Cow<'a, str>>,
    /// Filter files created by a single user.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListFilesInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<ListReactionsInner>>,
    pub shares: Option<ListSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListPagingInner {
    pub count: Option<u64>,
    pub page: u64,
    pub pages: Option<u64>,
    pub per_page: Option<u64>,
    pub spill: Option<u64>,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub files: Vec<ListFilesInner>,
    #[serde(default)]
    ok: bool,
    pub paging: ListPagingInner,
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

#[derive(Debug)]
pub enum ListError<E: Error> {
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    UnknownType,
    UpgradeRequired,
    UserIsBot,
    UserNotFound,
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
            "account_inactive" => ListError::AccountInactive,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_auth" => ListError::InvalidAuth,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_json" => ListError::InvalidJson,
            "invalid_post_type" => ListError::InvalidPostType,
            "json_not_object" => ListError::JsonNotObject,
            "missing_post_type" => ListError::MissingPostType,
            "no_permission" => ListError::NoPermission,
            "not_authed" => ListError::NotAuthed,
            "request_timeout" => ListError::RequestTimeout,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "unknown_type" => ListError::UnknownType,
            "upgrade_required" => ListError::UpgradeRequired,
            "user_is_bot" => ListError::UserIsBot,
            "user_not_found" => ListError::UserNotFound,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::UnknownType => write!(f, "Server returned error unknown_type"),
            ListError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ListError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            ListError::UserNotFound => write!(f, "Server returned error user_not_found"),
            ListError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ListError::Unknown(ref s) => write!(f, "{}", s),
            ListError::Client(ref inner) => write!(f, "{}", inner),
        }
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
    pub file: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLFileInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<RevokePublicURLReactionsInner>>,
    pub shares: Option<RevokePublicURLSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokePublicURLResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub file: RevokePublicURLFileInner,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RevokePublicURLResponse, RevokePublicURLError<E>>>
    for RevokePublicURLResponse
{
    fn into(self) -> Result<RevokePublicURLResponse, RevokePublicURLError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum RevokePublicURLError<E: Error> {
    AccountInactive,
    FatalError,
    FileNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
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
            "account_inactive" => RevokePublicURLError::AccountInactive,
            "fatal_error" => RevokePublicURLError::FatalError,
            "file_not_found" => RevokePublicURLError::FileNotFound,
            "invalid_arg_name" => RevokePublicURLError::InvalidArgName,
            "invalid_array_arg" => RevokePublicURLError::InvalidArrayArg,
            "invalid_auth" => RevokePublicURLError::InvalidAuth,
            "invalid_charset" => RevokePublicURLError::InvalidCharset,
            "invalid_form_data" => RevokePublicURLError::InvalidFormData,
            "invalid_json" => RevokePublicURLError::InvalidJson,
            "invalid_post_type" => RevokePublicURLError::InvalidPostType,
            "json_not_object" => RevokePublicURLError::JsonNotObject,
            "missing_post_type" => RevokePublicURLError::MissingPostType,
            "no_permission" => RevokePublicURLError::NoPermission,
            "not_authed" => RevokePublicURLError::NotAuthed,
            "org_login_required" => RevokePublicURLError::OrgLoginRequired,
            "request_timeout" => RevokePublicURLError::RequestTimeout,
            "team_added_to_org" => RevokePublicURLError::TeamAddedToOrg,
            "token_revoked" => RevokePublicURLError::TokenRevoked,
            "upgrade_required" => RevokePublicURLError::UpgradeRequired,
            "user_is_bot" => RevokePublicURLError::UserIsBot,
            "user_is_restricted" => RevokePublicURLError::UserIsRestricted,
            _ => RevokePublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokePublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RevokePublicURLError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            RevokePublicURLError::FatalError => write!(f, "Server returned error fatal_error"),
            RevokePublicURLError::FileNotFound => write!(f, "Server returned error file_not_found"),
            RevokePublicURLError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            RevokePublicURLError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            RevokePublicURLError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RevokePublicURLError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            RevokePublicURLError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            RevokePublicURLError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RevokePublicURLError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            RevokePublicURLError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            RevokePublicURLError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            RevokePublicURLError::NoPermission => write!(f, "Server returned error no_permission"),
            RevokePublicURLError::NotAuthed => write!(f, "Server returned error not_authed"),
            RevokePublicURLError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            RevokePublicURLError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            RevokePublicURLError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            RevokePublicURLError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            RevokePublicURLError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            RevokePublicURLError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            RevokePublicURLError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            RevokePublicURLError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RevokePublicURLError::Unknown(ref s) => write!(f, "{}", s),
            RevokePublicURLError::Client(ref inner) => write!(f, "{}", inner),
        }
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
    pub file: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLFileInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<SharedPublicURLReactionsInner>>,
    pub shares: Option<SharedPublicURLSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedPublicURLResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub file: SharedPublicURLFileInner,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SharedPublicURLResponse, SharedPublicURLError<E>>>
    for SharedPublicURLResponse
{
    fn into(self) -> Result<SharedPublicURLResponse, SharedPublicURLError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SharedPublicURLError<E: Error> {
    AccountInactive,
    FatalError,
    FileNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAllowed,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
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
            "account_inactive" => SharedPublicURLError::AccountInactive,
            "fatal_error" => SharedPublicURLError::FatalError,
            "file_not_found" => SharedPublicURLError::FileNotFound,
            "invalid_arg_name" => SharedPublicURLError::InvalidArgName,
            "invalid_array_arg" => SharedPublicURLError::InvalidArrayArg,
            "invalid_auth" => SharedPublicURLError::InvalidAuth,
            "invalid_charset" => SharedPublicURLError::InvalidCharset,
            "invalid_form_data" => SharedPublicURLError::InvalidFormData,
            "invalid_json" => SharedPublicURLError::InvalidJson,
            "invalid_post_type" => SharedPublicURLError::InvalidPostType,
            "json_not_object" => SharedPublicURLError::JsonNotObject,
            "missing_post_type" => SharedPublicURLError::MissingPostType,
            "no_permission" => SharedPublicURLError::NoPermission,
            "not_allowed" => SharedPublicURLError::NotAllowed,
            "not_authed" => SharedPublicURLError::NotAuthed,
            "org_login_required" => SharedPublicURLError::OrgLoginRequired,
            "request_timeout" => SharedPublicURLError::RequestTimeout,
            "team_added_to_org" => SharedPublicURLError::TeamAddedToOrg,
            "token_revoked" => SharedPublicURLError::TokenRevoked,
            "upgrade_required" => SharedPublicURLError::UpgradeRequired,
            "user_is_bot" => SharedPublicURLError::UserIsBot,
            "user_is_restricted" => SharedPublicURLError::UserIsRestricted,
            _ => SharedPublicURLError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SharedPublicURLError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SharedPublicURLError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            SharedPublicURLError::FatalError => write!(f, "Server returned error fatal_error"),
            SharedPublicURLError::FileNotFound => write!(f, "Server returned error file_not_found"),
            SharedPublicURLError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            SharedPublicURLError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            SharedPublicURLError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SharedPublicURLError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            SharedPublicURLError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            SharedPublicURLError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SharedPublicURLError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            SharedPublicURLError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            SharedPublicURLError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            SharedPublicURLError::NoPermission => write!(f, "Server returned error no_permission"),
            SharedPublicURLError::NotAllowed => write!(f, "Server returned error not_allowed"),
            SharedPublicURLError::NotAuthed => write!(f, "Server returned error not_authed"),
            SharedPublicURLError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SharedPublicURLError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            SharedPublicURLError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            SharedPublicURLError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SharedPublicURLError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            SharedPublicURLError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            SharedPublicURLError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            SharedPublicURLError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SharedPublicURLError::Unknown(ref s) => write!(f, "{}", s),
            SharedPublicURLError::Client(ref inner) => write!(f, "{}", inner),
        }
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

#[derive(Clone, Default, Debug)]
pub struct UploadRequest<'a> {
    /// Comma-separated list of channel names or IDs where the file will be shared.
    pub channels: Option<Cow<'a, str>>,
    /// File contents via a POST variable. If omitting this parameter, you must provide a `file`.
    pub content: Option<Cow<'a, str>>,
    /// File contents via `multipart/form-data`. If omitting this parameter, you must submit `content`.
    pub file: Option<Cow<'a, str>>,
    /// Filename of file.
    pub filename: Option<Cow<'a, str>>,
    /// A [file type](/types/file#file_types) identifier.
    pub filetype: Option<Cow<'a, str>>,
    /// The message text introducing the file in specified `channels`.
    pub initial_comment: Option<Cow<'a, str>>,
    /// Provide another message's `ts` value to upload this file as a reply. Never use a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<f64>,
    /// Title of file.
    pub title: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadFileInner {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<u64>,
    pub created: Option<u64>,
    pub date_delete: Option<u64>,
    pub display_as_bot: Option<bool>,
    pub editable: Option<bool>,
    pub editor: Option<String>,
    pub external_id: Option<String>,
    pub external_type: Option<String>,
    pub external_url: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub has_rich_preview: Option<bool>,
    pub id: Option<String>,
    pub image_exif_rotation: Option<u64>,
    pub ims: Option<Vec<String>>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_tombstoned: Option<bool>,
    pub last_editor: Option<String>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub non_owner_editable: Option<bool>,
    pub num_stars: Option<u64>,
    pub original_h: Option<u64>,
    pub original_w: Option<u64>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub public_url_shared: Option<bool>,
    pub reactions: Option<Vec<UploadReactionsInner>>,
    pub shares: Option<UploadSharesInner>,
    pub size: Option<u64>,
    pub source_team: Option<String>,
    pub state: Option<String>,
    pub thumb_1024: Option<String>,
    pub thumb_1024_h: Option<u64>,
    pub thumb_1024_w: Option<u64>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_h: Option<u64>,
    pub thumb_360_w: Option<u64>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<u64>,
    pub thumb_480_w: Option<u64>,
    pub thumb_64: Option<String>,
    pub thumb_720: Option<String>,
    pub thumb_720_h: Option<u64>,
    pub thumb_720_w: Option<u64>,
    pub thumb_80: Option<String>,
    pub thumb_800: Option<String>,
    pub thumb_800_h: Option<u64>,
    pub thumb_800_w: Option<u64>,
    pub thumb_960: Option<String>,
    pub thumb_960_h: Option<u64>,
    pub thumb_960_w: Option<u64>,
    pub thumb_tiny: Option<String>,
    pub timestamp: Option<u64>,
    pub title: Option<String>,
    pub updated: Option<u64>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<String>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub file: UploadFileInner,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<UploadResponse, UploadError<E>>> for UploadResponse {
    fn into(self) -> Result<UploadResponse, UploadError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum UploadError<E: Error> {
    AccountInactive,
    FileUploadSizeRestricted,
    FileUploadsDisabled,
    FileUploadsExceptImagesDisabled,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidChannel,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    PostingToGeneralChannelDenied,
    RequestTimeout,
    StorageLimitReached,
    TeamAddedToOrg,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UploadError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => UploadError::AccountInactive,
            "file_upload_size_restricted" => UploadError::FileUploadSizeRestricted,
            "file_uploads_disabled" => UploadError::FileUploadsDisabled,
            "file_uploads_except_images_disabled" => UploadError::FileUploadsExceptImagesDisabled,
            "invalid_arg_name" => UploadError::InvalidArgName,
            "invalid_array_arg" => UploadError::InvalidArrayArg,
            "invalid_auth" => UploadError::InvalidAuth,
            "invalid_channel" => UploadError::InvalidChannel,
            "invalid_charset" => UploadError::InvalidCharset,
            "invalid_form_data" => UploadError::InvalidFormData,
            "invalid_json" => UploadError::InvalidJson,
            "invalid_post_type" => UploadError::InvalidPostType,
            "json_not_object" => UploadError::JsonNotObject,
            "missing_post_type" => UploadError::MissingPostType,
            "no_permission" => UploadError::NoPermission,
            "not_authed" => UploadError::NotAuthed,
            "posting_to_general_channel_denied" => UploadError::PostingToGeneralChannelDenied,
            "request_timeout" => UploadError::RequestTimeout,
            "storage_limit_reached" => UploadError::StorageLimitReached,
            "team_added_to_org" => UploadError::TeamAddedToOrg,
            "upgrade_required" => UploadError::UpgradeRequired,
            _ => UploadError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UploadError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UploadError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UploadError::FileUploadSizeRestricted => {
                write!(f, "Server returned error file_upload_size_restricted")
            }
            UploadError::FileUploadsDisabled => {
                write!(f, "Server returned error file_uploads_disabled")
            }
            UploadError::FileUploadsExceptImagesDisabled => write!(
                f,
                "Server returned error file_uploads_except_images_disabled"
            ),
            UploadError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UploadError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UploadError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UploadError::InvalidChannel => write!(f, "Server returned error invalid_channel"),
            UploadError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UploadError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UploadError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UploadError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UploadError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UploadError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UploadError::NoPermission => write!(f, "Server returned error no_permission"),
            UploadError::NotAuthed => write!(f, "Server returned error not_authed"),
            UploadError::PostingToGeneralChannelDenied => {
                write!(f, "Server returned error posting_to_general_channel_denied")
            }
            UploadError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UploadError::StorageLimitReached => {
                write!(f, "Server returned error storage_limit_reached")
            }
            UploadError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UploadError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UploadError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UploadError::Unknown(ref s) => write!(f, "{}", s),
            UploadError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UploadError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UploadError::MalformedResponse(_, ref e) => Some(e),
            UploadError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
