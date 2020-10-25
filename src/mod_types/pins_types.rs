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

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct AddRequest {
    /// Channel to pin the item in.
    pub channel: String,
    /// Timestamp of the message to pin.
    pub timestamp: Option<String>,
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
    BadTimestamp,
    MessageNotFound,
    ChannelNotFound,
    NoItemSpecified,
    AlreadyPinned,
    PermissionDenied,
    FileNotShared,
    NotPinnable,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
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
            "bad_timestamp" => AddError::BadTimestamp,
            "message_not_found" => AddError::MessageNotFound,
            "channel_not_found" => AddError::ChannelNotFound,
            "no_item_specified" => AddError::NoItemSpecified,
            "already_pinned" => AddError::AlreadyPinned,
            "permission_denied" => AddError::PermissionDenied,
            "file_not_shared" => AddError::FileNotShared,
            "not_pinnable" => AddError::NotPinnable,
            "not_authed" => AddError::NotAuthed,
            "invalid_auth" => AddError::InvalidAuth,
            "account_inactive" => AddError::AccountInactive,
            "no_permission" => AddError::NoPermission,
            "invalid_arg_name" => AddError::InvalidArgName,
            "invalid_array_arg" => AddError::InvalidArrayArg,
            "invalid_charset" => AddError::InvalidCharset,
            "invalid_form_data" => AddError::InvalidFormData,
            "invalid_post_type" => AddError::InvalidPostType,
            "missing_post_type" => AddError::MissingPostType,
            "team_added_to_org" => AddError::TeamAddedToOrg,
            "invalid_json" => AddError::InvalidJson,
            "json_not_object" => AddError::JsonNotObject,
            "request_timeout" => AddError::RequestTimeout,
            "upgrade_required" => AddError::UpgradeRequired,
            _ => AddError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AddError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AddError::BadTimestamp => write!(f, "Server returned error bad_timestamp"),
            AddError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            AddError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            AddError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            AddError::AlreadyPinned => write!(f, "Server returned error already_pinned"),
            AddError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            AddError::FileNotShared => write!(f, "Server returned error file_not_shared"),
            AddError::NotPinnable => write!(f, "Server returned error not_pinnable"),
            AddError::NotAuthed => write!(f, "Server returned error not_authed"),
            AddError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            AddError::AccountInactive => write!(f, "Server returned error account_inactive"),
            AddError::NoPermission => write!(f, "Server returned error no_permission"),
            AddError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            AddError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            AddError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            AddError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            AddError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            AddError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            AddError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            AddError::InvalidJson => write!(f, "Server returned error invalid_json"),
            AddError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            AddError::RequestTimeout => write!(f, "Server returned error request_timeout"),
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
pub struct RemoveRequest {
    /// Channel where the item is pinned to.
    pub channel: String,
    /// Timestamp of the message to un-pin.
    pub timestamp: Option<String>,
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
    BadTimestamp,
    FileNotFound,
    FileCommentNotFound,
    MessageNotFound,
    NoItemSpecified,
    NotPinned,
    PermissionDenied,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostTyp,
    MissingPostTyp,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeou,
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
            "bad_timestamp" => RemoveError::BadTimestamp,
            "file_not_found" => RemoveError::FileNotFound,
            "file_comment_not_found" => RemoveError::FileCommentNotFound,
            "message_not_found" => RemoveError::MessageNotFound,
            "no_item_specified" => RemoveError::NoItemSpecified,
            "not_pinned" => RemoveError::NotPinned,
            "permission_denied" => RemoveError::PermissionDenied,
            "not_authed" => RemoveError::NotAuthed,
            "invalid_auth" => RemoveError::InvalidAuth,
            "account_inactive" => RemoveError::AccountInactive,
            "no_permission" => RemoveError::NoPermission,
            "invalid_arg_name" => RemoveError::InvalidArgName,
            "invalid_array_arg" => RemoveError::InvalidArrayArg,
            "invalid_charset" => RemoveError::InvalidCharset,
            "invalid_form_data" => RemoveError::InvalidFormData,
            "invalid_post_typ" => RemoveError::InvalidPostTyp,
            "missing_post_typ" => RemoveError::MissingPostTyp,
            "team_added_to_org" => RemoveError::TeamAddedToOrg,
            "invalid_json" => RemoveError::InvalidJson,
            "json_not_object" => RemoveError::JsonNotObject,
            "request_timeou" => RemoveError::RequestTimeou,
            "upgrade_required" => RemoveError::UpgradeRequired,
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RemoveError::BadTimestamp => write!(f, "Server returned error bad_timestamp"),
            RemoveError::FileNotFound => write!(f, "Server returned error file_not_found"),
            RemoveError::FileCommentNotFound => {
                write!(f, "Server returned error file_comment_not_found")
            }
            RemoveError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            RemoveError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            RemoveError::NotPinned => write!(f, "Server returned error not_pinned"),
            RemoveError::PermissionDenied => write!(f, "Server returned error permission_denied"),
            RemoveError::NotAuthed => write!(f, "Server returned error not_authed"),
            RemoveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RemoveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RemoveError::NoPermission => write!(f, "Server returned error no_permission"),
            RemoveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RemoveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RemoveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RemoveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RemoveError::InvalidPostTyp => write!(f, "Server returned error invalid_post_typ"),
            RemoveError::MissingPostTyp => write!(f, "Server returned error missing_post_typ"),
            RemoveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RemoveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RemoveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RemoveError::RequestTimeou => write!(f, "Server returned error request_timeou"),
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

#[derive(Clone, Default, Debug)]
pub struct ListRequest {
    /// Channel to get pinned items for.
    pub channel: String,
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
    ChannelNotFound,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    NoPermission,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    TeamAddedToOrg,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
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
            "channel_not_found" => ListError::ChannelNotFound,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "no_permission" => ListError::NoPermission,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
            "team_added_to_org" => ListError::TeamAddedToOrg,
            "invalid_json" => ListError::InvalidJson,
            "json_not_object" => ListError::JsonNotObject,
            "request_timeout" => ListError::RequestTimeout,
            "upgrade_required" => ListError::UpgradeRequired,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::NoPermission => write!(f, "Server returned error no_permission"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ListError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::RequestTimeout => write!(f, "Server returned error request_timeout"),
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
