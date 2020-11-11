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

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct AddRequest<'a> {
    /// Channel to pin the item in.
    pub channel: Cow<'a, str>,
    /// Timestamp of the message to pin.
    pub timestamp: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AddResponse, AddError<E>>> for AddResponse {
    fn into(self) -> Result<AddResponse, AddError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum AddError<E: Error> {
    AccountInactive,
    AlreadyPinned,
    BadTimestamp,
    ChannelNotFound,
    FileNotShared,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MessageNotFound,
    MissingPostType,
    NoItemSpecified,
    NoPermission,
    NotAuthed,
    NotPinnable,
    PermissionDenied,
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

impl<'a, E: Error> From<&'a str> for AddError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => AddError::AccountInactive,
            "already_pinned" => AddError::AlreadyPinned,
            "bad_timestamp" => AddError::BadTimestamp,
            "channel_not_found" => AddError::ChannelNotFound,
            "file_not_shared" => AddError::FileNotShared,
            "invalid_arg_name" => AddError::InvalidArgName,
            "invalid_array_arg" => AddError::InvalidArrayArg,
            "invalid_auth" => AddError::InvalidAuth,
            "invalid_charset" => AddError::InvalidCharset,
            "invalid_form_data" => AddError::InvalidFormData,
            "invalid_json" => AddError::InvalidJson,
            "invalid_post_type" => AddError::InvalidPostType,
            "json_not_object" => AddError::JsonNotObject,
            "message_not_found" => AddError::MessageNotFound,
            "missing_post_type" => AddError::MissingPostType,
            "no_item_specified" => AddError::NoItemSpecified,
            "no_permission" => AddError::NoPermission,
            "not_authed" => AddError::NotAuthed,
            "not_pinnable" => AddError::NotPinnable,
            "permission_denied" => AddError::PermissionDenied,
            "request_timeout" => AddError::RequestTimeout,
            "team_added_to_org" => AddError::TeamAddedToOrg,
            "upgrade_required" => AddError::UpgradeRequired,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::AccountInactive => write!(f, "Server returned error account_inactive"),
            AddError::AlreadyPinned => write!(f, "Server returned error already_pinned"),
            AddError::BadTimestamp => write!(f, "Server returned error bad_timestamp"),
            AddError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            AddError::FileNotShared => write!(f, "Server returned error file_not_shared"),
            AddError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            AddError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            AddError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            AddError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            AddError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            AddError::InvalidJson => write!(f, "Server returned error invalid_json"),
            AddError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            AddError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            AddError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            AddError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            AddError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            AddError::NoPermission => write!(f, "Server returned error no_permission"),
            AddError::NotAuthed => write!(f, "Server returned error not_authed"),
            AddError::NotPinnable => write!(f, "Server returned error not_pinnable"),
            AddError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            AddError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            AddError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            AddError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            AddError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AddError::Unknown(ref s) => write!(f, "{}", s),
            AddError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AddError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AddError::MalformedResponse(_, ref e) => Some(e),
            AddError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Channel to get pinned items for.
    pub channel: Cow<'a, str>,
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
pub struct ListFileInner {
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
pub struct ListItemsInner {
    pub created: Option<u64>,
    pub created_by: Option<String>,
    pub file: Option<ListFileInner>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResultInner {
    pub items: Vec<ListItemsInner>,
    #[serde(default)]
    ok: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub result: ListResultInner,
}

impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.result.ok {
            Ok(self)
        } else {
            Err(ListError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum ListError<E: Error> {
    AccountInactive,
    ChannelNotFound,
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

impl<'a, E: Error> From<&'a str> for ListError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ListError::AccountInactive,
            "channel_not_found" => ListError::ChannelNotFound,
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
            "upgrade_required" => ListError::UpgradeRequired,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
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
            ListError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct RemoveRequest<'a> {
    /// Channel where the item is pinned to.
    pub channel: Cow<'a, str>,
    /// Timestamp of the message to un-pin.
    pub timestamp: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RemoveResponse, RemoveError<E>>> for RemoveResponse {
    fn into(self) -> Result<RemoveResponse, RemoveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum RemoveError<E: Error> {
    AccountInactive,
    BadTimestamp,
    FileCommentNotFound,
    FileNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostTyp,
    JsonNotObject,
    MessageNotFound,
    MissingPostTyp,
    NoItemSpecified,
    NoPermission,
    NotAuthed,
    NotPinned,
    PermissionDenied,
    RequestTimeou,
    TeamAddedToOrg,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RemoveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => RemoveError::AccountInactive,
            "bad_timestamp" => RemoveError::BadTimestamp,
            "file_comment_not_found" => RemoveError::FileCommentNotFound,
            "file_not_found" => RemoveError::FileNotFound,
            "invalid_arg_name" => RemoveError::InvalidArgName,
            "invalid_array_arg" => RemoveError::InvalidArrayArg,
            "invalid_auth" => RemoveError::InvalidAuth,
            "invalid_charset" => RemoveError::InvalidCharset,
            "invalid_form_data" => RemoveError::InvalidFormData,
            "invalid_json" => RemoveError::InvalidJson,
            "invalid_post_typ" => RemoveError::InvalidPostTyp,
            "json_not_object" => RemoveError::JsonNotObject,
            "message_not_found" => RemoveError::MessageNotFound,
            "missing_post_typ" => RemoveError::MissingPostTyp,
            "no_item_specified" => RemoveError::NoItemSpecified,
            "no_permission" => RemoveError::NoPermission,
            "not_authed" => RemoveError::NotAuthed,
            "not_pinned" => RemoveError::NotPinned,
            "permission_denied" => RemoveError::PermissionDenied,
            "request_timeou" => RemoveError::RequestTimeou,
            "team_added_to_org" => RemoveError::TeamAddedToOrg,
            "upgrade_required" => RemoveError::UpgradeRequired,
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RemoveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RemoveError::BadTimestamp => write!(f, "Server returned error bad_timestamp"),
            RemoveError::FileCommentNotFound => {
                write!(f, "Server returned error file_comment_not_found")
            }
            RemoveError::FileNotFound => write!(f, "Server returned error file_not_found"),
            RemoveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RemoveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RemoveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RemoveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RemoveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RemoveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RemoveError::InvalidPostTyp => write!(f, "Server returned error invalid_post_typ"),
            RemoveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RemoveError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            RemoveError::MissingPostTyp => write!(f, "Server returned error missing_post_typ"),
            RemoveError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            RemoveError::NoPermission => write!(f, "Server returned error no_permission"),
            RemoveError::NotAuthed => write!(f, "Server returned error not_authed"),
            RemoveError::NotPinned => write!(f, "Server returned error not_pinned"),
            RemoveError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            RemoveError::RequestTimeou => write!(f, "Server returned error request_timeou"),
            RemoveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RemoveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RemoveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RemoveError::Unknown(ref s) => write!(f, "{}", s),
            RemoveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RemoveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RemoveError::MalformedResponse(_, ref e) => Some(e),
            RemoveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
