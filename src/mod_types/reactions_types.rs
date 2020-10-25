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
pub struct ListRequest {
    /// Show reactions made by this user. Defaults to the authed user.
    pub user: Option<String>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
    pub count: Option<u64>,
    pub page: Option<u64>,
    /// Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](/docs/pagination) for more details.
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: ListIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<ListReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListReactions1Inner {
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
    pub reactions: Option<Vec<ListReactions1Inner>>,
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
pub struct ListReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListShares1Inner {
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
    pub reactions: Option<Vec<ListReactions2Inner>>,
    pub shares: Option<ListShares1Inner>,
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
pub struct ListIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListUserProfileInner {
    pub avatar_hash: String,
    pub display_name: String,
    pub display_name_normalized: Option<String>,
    pub first_name: String,
    pub image_72: String,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub name: String,
    pub real_name: String,
    pub real_name_normalized: Option<String>,
    pub team: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListMessageInner {
    pub attachments: Option<Vec<ListAttachmentsInner>>,
    pub blocks: Option<Vec<ListBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<ListBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<ListCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<ListFileInner>,
    pub files: Option<Vec<ListFilesInner>>,
    pub icons: Option<ListIcons1Inner>,
    pub inviter: Option<String>,
    pub is_delayed_message: Option<bool>,
    pub is_intro: Option<bool>,
    pub is_starred: Option<bool>,
    pub last_read: Option<String>,
    pub latest_reply: Option<String>,
    pub name: Option<String>,
    pub old_name: Option<String>,
    pub parent_user_id: Option<String>,
    pub permalink: Option<String>,
    pub pinned_to: Option<Vec<String>>,
    pub purpose: Option<String>,
    pub reactions: Option<Vec<ListReactions3Inner>>,
    pub reply_count: Option<u64>,
    pub reply_users: Option<Vec<String>>,
    pub reply_users_count: Option<u64>,
    pub source_team: Option<String>,
    pub subscribed: Option<bool>,
    pub subtype: Option<String>,
    pub team: Option<String>,
    pub text: String,
    pub thread_ts: Option<String>,
    pub topic: Option<String>,
    pub ts: String,
    pub r#type: String,
    pub unread_count: Option<u64>,
    pub upload: Option<bool>,
    pub user: Option<String>,
    pub user_profile: Option<ListUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListItemsInner {
    pub channel: String,
    pub message: ListMessageInner,
    pub r#type: String,
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
pub struct ListResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub items: Vec<Vec<ListItemsInner>>,
    #[serde(default)]
    ok: bool,
    pub paging: Option<ListPagingInner>,
    pub response_metadata: Option<Vec<ListResponseMetadataInner>>,
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
    UserNotFound,
    NotAuthed,
    InvalidAuth,
    AccountInactiv,
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
    FatalError,
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
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactiv" => ListError::AccountInactiv,
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
            "fatal_error" => ListError::FatalError,
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ListError::UserNotFound => write!(f, "Server returned error user_not_found"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::AccountInactiv => write!(f, "Server returned error account_inactiv"),
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
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
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
pub struct GetRequest {
    /// Channel where the message to get reactions for was posted.
    pub channel: Option<String>,
    /// File to get reactions for.
    pub file: Option<String>,
    /// File comment to get reactions for.
    pub file_comment: Option<String>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
    /// Timestamp of the message to get reactions for.
    pub timestamp: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<GetResponse, GetError<E>>> for GetResponse {
    fn into(self) -> Result<GetResponse, GetError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetError<E: Error> {
    BadTimestamp,
    FileNotFound,
    FileCommentNotFound,
    MessageNotFound,
    NoItemSpecified,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    NoPermission,
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

impl<'a, E: Error> From<&'a str> for GetError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "bad_timestamp" => GetError::BadTimestamp,
            "file_not_found" => GetError::FileNotFound,
            "file_comment_not_found" => GetError::FileCommentNotFound,
            "message_not_found" => GetError::MessageNotFound,
            "no_item_specified" => GetError::NoItemSpecified,
            "not_authed" => GetError::NotAuthed,
            "invalid_auth" => GetError::InvalidAuth,
            "account_inactive" => GetError::AccountInactive,
            "no_permission" => GetError::NoPermission,
            "invalid_array_arg" => GetError::InvalidArrayArg,
            "invalid_charset" => GetError::InvalidCharset,
            "invalid_form_data" => GetError::InvalidFormData,
            "invalid_post_type" => GetError::InvalidPostType,
            "missing_post_type" => GetError::MissingPostType,
            "team_added_to_org" => GetError::TeamAddedToOrg,
            "invalid_json" => GetError::InvalidJson,
            "json_not_object" => GetError::JsonNotObject,
            "request_timeout" => GetError::RequestTimeout,
            "upgrade_required" => GetError::UpgradeRequired,
            _ => GetError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetError::BadTimestamp => write!(f, "Server returned error bad_timestamp"),
            GetError::FileNotFound => write!(f, "Server returned error file_not_found"),
            GetError::FileCommentNotFound => {
                write!(f, "Server returned error file_comment_not_found")
            }
            GetError::MessageNotFound => write!(f, "Server returned error message_not_found"),
            GetError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            GetError::NotAuthed => write!(f, "Server returned error not_authed"),
            GetError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            GetError::AccountInactive => write!(f, "Server returned error account_inactive"),
            GetError::NoPermission => write!(f, "Server returned error no_permission"),
            GetError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            GetError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            GetError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            GetError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            GetError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            GetError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            GetError::InvalidJson => write!(f, "Server returned error invalid_json"),
            GetError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            GetError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            GetError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            GetError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetError::Unknown(ref s) => write!(f, "{}", s),
            GetError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetError::MalformedResponse(_, ref e) => Some(e),
            GetError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AddRequest {
    /// Channel where the message to add reaction to was posted.
    pub channel: String,
    /// Reaction (emoji) name.
    pub name: String,
    /// Timestamp of the message to add reaction to.
    pub timestamp: String,
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
    NoItemSpecified,
    InvalidName,
    AlreadyReacted,
    TooManyEmoji,
    TooManyReactions,
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
            "no_item_specified" => AddError::NoItemSpecified,
            "invalid_name" => AddError::InvalidName,
            "already_reacted" => AddError::AlreadyReacted,
            "too_many_emoji" => AddError::TooManyEmoji,
            "too_many_reactions" => AddError::TooManyReactions,
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
            AddError::NoItemSpecified => write!(f, "Server returned error no_item_specified"),
            AddError::InvalidName => write!(f, "Server returned error invalid_name"),
            AddError::AlreadyReacted => write!(f, "Server returned error already_reacted"),
            AddError::TooManyEmoji => write!(f, "Server returned error too_many_emoji"),
            AddError::TooManyReactions => write!(f, "Server returned error too_many_reactions"),
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
    /// Reaction (emoji) name.
    pub name: String,
    /// File to remove reaction from.
    pub file: Option<String>,
    /// File comment to remove reaction from.
    pub file_comment: Option<String>,
    /// Channel where the message to remove reaction from was posted.
    pub channel: Option<String>,
    /// Timestamp of the message to remove reaction from.
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
    InvalidName,
    NoReaction,
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
    FatalError,
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
            "invalid_name" => RemoveError::InvalidName,
            "no_reaction" => RemoveError::NoReaction,
            "not_authed" => RemoveError::NotAuthed,
            "invalid_auth" => RemoveError::InvalidAuth,
            "account_inactive" => RemoveError::AccountInactive,
            "no_permission" => RemoveError::NoPermission,
            "invalid_arg_name" => RemoveError::InvalidArgName,
            "invalid_array_arg" => RemoveError::InvalidArrayArg,
            "invalid_charset" => RemoveError::InvalidCharset,
            "invalid_form_data" => RemoveError::InvalidFormData,
            "invalid_post_type" => RemoveError::InvalidPostType,
            "missing_post_type" => RemoveError::MissingPostType,
            "team_added_to_org" => RemoveError::TeamAddedToOrg,
            "invalid_json" => RemoveError::InvalidJson,
            "json_not_object" => RemoveError::JsonNotObject,
            "request_timeout" => RemoveError::RequestTimeout,
            "upgrade_required" => RemoveError::UpgradeRequired,
            "fatal_error" => RemoveError::FatalError,
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
            RemoveError::InvalidName => write!(f, "Server returned error invalid_name"),
            RemoveError::NoReaction => write!(f, "Server returned error no_reaction"),
            RemoveError::NotAuthed => write!(f, "Server returned error not_authed"),
            RemoveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RemoveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RemoveError::NoPermission => write!(f, "Server returned error no_permission"),
            RemoveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RemoveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RemoveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RemoveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RemoveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RemoveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RemoveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RemoveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RemoveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RemoveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RemoveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RemoveError::FatalError => write!(f, "Server returned error fatal_error"),
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
