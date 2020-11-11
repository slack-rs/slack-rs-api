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
pub struct ArchiveRequest<'a> {
    /// ID of conversation to archive
    pub channel: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArchiveResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum ArchiveError<E: Error> {
    AccountInactive,
    AlreadyArchived,
    CantArchiveGeneral,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingCharset,
    MissingPostType,
    MissingScope,
    NotAuthed,
    NotSupported,
    RequestTimeout,
    RestrictedAction,
    SuperfluousCharset,
    TeamAddedToOrg,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ArchiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ArchiveError::AccountInactive,
            "already_archived" => ArchiveError::AlreadyArchived,
            "cant_archive_general" => ArchiveError::CantArchiveGeneral,
            "channel_not_found" => ArchiveError::ChannelNotFound,
            "invalid_arg_name" => ArchiveError::InvalidArgName,
            "invalid_array_arg" => ArchiveError::InvalidArrayArg,
            "invalid_auth" => ArchiveError::InvalidAuth,
            "invalid_charset" => ArchiveError::InvalidCharset,
            "invalid_form_data" => ArchiveError::InvalidFormData,
            "invalid_json" => ArchiveError::InvalidJson,
            "invalid_post_type" => ArchiveError::InvalidPostType,
            "json_not_object" => ArchiveError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                ArchiveError::MethodNotSupportedForChannelType
            }
            "missing_charset" => ArchiveError::MissingCharset,
            "missing_post_type" => ArchiveError::MissingPostType,
            "missing_scope" => ArchiveError::MissingScope,
            "not_authed" => ArchiveError::NotAuthed,
            "not_supported" => ArchiveError::NotSupported,
            "request_timeout" => ArchiveError::RequestTimeout,
            "restricted_action" => ArchiveError::RestrictedAction,
            "superfluous_charset" => ArchiveError::SuperfluousCharset,
            "team_added_to_org" => ArchiveError::TeamAddedToOrg,
            "upgrade_required" => ArchiveError::UpgradeRequired,
            "user_is_bot" => ArchiveError::UserIsBot,
            "user_is_restricted" => ArchiveError::UserIsRestricted,
            "user_is_ultra_restricted" => ArchiveError::UserIsUltraRestricted,
            _ => ArchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ArchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArchiveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ArchiveError::AlreadyArchived => write!(f, "Server returned error already_archived"),
            ArchiveError::CantArchiveGeneral => {
                write!(f, "Server returned error cant_archive_general")
            }
            ArchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            ArchiveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ArchiveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ArchiveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ArchiveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ArchiveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ArchiveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ArchiveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ArchiveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ArchiveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            ArchiveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            ArchiveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ArchiveError::MissingScope => write!(f, "Server returned error missing_scope"),
            ArchiveError::NotAuthed => write!(f, "Server returned error not_authed"),
            ArchiveError::NotSupported => write!(f, "Server returned error not_supported"),
            ArchiveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ArchiveError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            ArchiveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            ArchiveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ArchiveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ArchiveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            ArchiveError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            ArchiveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            ArchiveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ArchiveError::Unknown(ref s) => write!(f, "{}", s),
            ArchiveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ArchiveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ArchiveError::MalformedResponse(_, ref e) => Some(e),
            ArchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct CloseRequest<'a> {
    /// Conversation to close.
    pub channel: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CloseResponse {
    pub already_closed: Option<bool>,
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    pub no_op: Option<bool>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
}

impl<E: Error> Into<Result<CloseResponse, CloseError<E>>> for CloseResponse {
    fn into(self) -> Result<CloseResponse, CloseError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum CloseError<E: Error> {
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
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    UpgradeRequired,
    UserDoesNotOwnChannel,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for CloseError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => CloseError::AccountInactive,
            "channel_not_found" => CloseError::ChannelNotFound,
            "invalid_arg_name" => CloseError::InvalidArgName,
            "invalid_array_arg" => CloseError::InvalidArrayArg,
            "invalid_auth" => CloseError::InvalidAuth,
            "invalid_charset" => CloseError::InvalidCharset,
            "invalid_form_data" => CloseError::InvalidFormData,
            "invalid_json" => CloseError::InvalidJson,
            "invalid_post_type" => CloseError::InvalidPostType,
            "json_not_object" => CloseError::JsonNotObject,
            "method_not_supported_for_channel_type" => CloseError::MethodNotSupportedForChannelType,
            "missing_post_type" => CloseError::MissingPostType,
            "missing_scope" => CloseError::MissingScope,
            "not_authed" => CloseError::NotAuthed,
            "request_timeout" => CloseError::RequestTimeout,
            "team_added_to_org" => CloseError::TeamAddedToOrg,
            "upgrade_required" => CloseError::UpgradeRequired,
            "user_does_not_own_channel" => CloseError::UserDoesNotOwnChannel,
            _ => CloseError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CloseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CloseError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CloseError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            CloseError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CloseError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CloseError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CloseError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CloseError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CloseError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CloseError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CloseError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CloseError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            CloseError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CloseError::MissingScope => write!(f, "Server returned error missing_scope"),
            CloseError::NotAuthed => write!(f, "Server returned error not_authed"),
            CloseError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CloseError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CloseError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CloseError::UserDoesNotOwnChannel => {
                write!(f, "Server returned error user_does_not_own_channel")
            }
            CloseError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            CloseError::Unknown(ref s) => write!(f, "{}", s),
            CloseError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for CloseError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CloseError::MalformedResponse(_, ref e) => Some(e),
            CloseError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct CreateRequest<'a> {
    /// Create a private channel instead of a public one
    pub is_private: Option<bool>,
    /// Name of the public or private channel to create
    pub name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: CreateIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<CreateReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateFileInner {
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
    pub reactions: Option<Vec<CreateReactions1Inner>>,
    pub shares: Option<CreateSharesInner>,
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
pub struct CreateReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateFilesInner {
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
    pub reactions: Option<Vec<CreateReactions2Inner>>,
    pub shares: Option<CreateShares1Inner>,
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
pub struct CreateIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateUserProfileInner {
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
pub struct CreateLatestInner {
    pub attachments: Option<Vec<CreateAttachmentsInner>>,
    pub blocks: Option<Vec<CreateBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<CreateBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<CreateCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<CreateFileInner>,
    pub files: Option<Vec<CreateFilesInner>>,
    pub icons: Option<CreateIcons1Inner>,
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
    pub reactions: Option<Vec<CreateReactions3Inner>>,
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
    pub user_profile: Option<CreateUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateExternalOrgMigrationsInner {
    pub current: Vec<CreateCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<CreateExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: CreateIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<CreatePrimaryOwnerInner>,
    pub sso_provider: Option<CreateSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: CreateTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<CreateDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<CreateLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: CreatePurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<CreateShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: CreateTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    pub callstack: Option<String>,
    pub channel: Vec<CreateChannelInner>,
    pub detail: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum CreateError<E: Error> {
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidName,
    InvalidNameMaxlength,
    InvalidNamePunctuation,
    InvalidNameRequired,
    InvalidNameSpecials,
    InvalidPostType,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NameTaken,
    NoChannel,
    NotAuthed,
    RequestTimeout,
    RestrictedAction,
    TeamAddedToOrg,
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

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => CreateError::AccountInactive,
            "invalid_arg_name" => CreateError::InvalidArgName,
            "invalid_array_arg" => CreateError::InvalidArrayArg,
            "invalid_auth" => CreateError::InvalidAuth,
            "invalid_charset" => CreateError::InvalidCharset,
            "invalid_form_data" => CreateError::InvalidFormData,
            "invalid_json" => CreateError::InvalidJson,
            "invalid_name" => CreateError::InvalidName,
            "invalid_name_maxlength" => CreateError::InvalidNameMaxlength,
            "invalid_name_punctuation" => CreateError::InvalidNamePunctuation,
            "invalid_name_required" => CreateError::InvalidNameRequired,
            "invalid_name_specials" => CreateError::InvalidNameSpecials,
            "invalid_post_type" => CreateError::InvalidPostType,
            "json_not_object" => CreateError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                CreateError::MethodNotSupportedForChannelType
            }
            "missing_post_type" => CreateError::MissingPostType,
            "missing_scope" => CreateError::MissingScope,
            "name_taken" => CreateError::NameTaken,
            "no_channel" => CreateError::NoChannel,
            "not_authed" => CreateError::NotAuthed,
            "request_timeout" => CreateError::RequestTimeout,
            "restricted_action" => CreateError::RestrictedAction,
            "team_added_to_org" => CreateError::TeamAddedToOrg,
            "upgrade_required" => CreateError::UpgradeRequired,
            "user_is_bot" => CreateError::UserIsBot,
            "user_is_restricted" => CreateError::UserIsRestricted,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CreateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CreateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CreateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CreateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CreateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CreateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CreateError::InvalidName => write!(f, "Server returned error invalid_name"),
            CreateError::InvalidNameMaxlength => {
                write!(f, "Server returned error invalid_name_maxlength")
            }
            CreateError::InvalidNamePunctuation => {
                write!(f, "Server returned error invalid_name_punctuation")
            }
            CreateError::InvalidNameRequired => {
                write!(f, "Server returned error invalid_name_required")
            }
            CreateError::InvalidNameSpecials => {
                write!(f, "Server returned error invalid_name_specials")
            }
            CreateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CreateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CreateError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            CreateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CreateError::MissingScope => write!(f, "Server returned error missing_scope"),
            CreateError::NameTaken => write!(f, "Server returned error name_taken"),
            CreateError::NoChannel => write!(f, "Server returned error no_channel"),
            CreateError::NotAuthed => write!(f, "Server returned error not_authed"),
            CreateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CreateError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            CreateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CreateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            CreateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CreateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            CreateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            CreateError::Unknown(ref s) => write!(f, "{}", s),
            CreateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for CreateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CreateError::MalformedResponse(_, ref e) => Some(e),
            CreateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct HistoryRequest<'a> {
    /// Conversation ID to fetch history for.
    pub channel: Cow<'a, str>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    pub inclusive: Option<bool>,
    /// End of time range of messages to include in results.
    pub latest: Option<f64>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryAttachmentsInner {
    pub fallback: Option<String>,
    pub id: Option<u64>,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: HistoryIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<HistoryReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistorySharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryFileInner {
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
    pub reactions: Option<Vec<HistoryReactions1Inner>>,
    pub shares: Option<HistorySharesInner>,
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
pub struct HistoryReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryFilesInner {
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
    pub reactions: Option<Vec<HistoryReactions2Inner>>,
    pub shares: Option<HistoryShares1Inner>,
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
pub struct HistoryIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryUserProfileInner {
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
pub struct HistoryMessagesInner {
    pub attachments: Option<Vec<HistoryAttachmentsInner>>,
    pub blocks: Option<Vec<HistoryBlocksInner>>,
    pub bot_id: Option<String>,
    pub bot_profile: Option<HistoryBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<HistoryCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<HistoryFileInner>,
    pub files: Option<Vec<HistoryFilesInner>>,
    pub icons: Option<HistoryIcons1Inner>,
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
    pub reactions: Option<Vec<HistoryReactions3Inner>>,
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
    pub user_profile: Option<HistoryUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryResponse {
    pub callstack: Option<String>,
    pub channel_actions_count: u64,
    pub channel_actions_ts: Option<Vec<u64>>,
    error: Option<String>,
    pub has_more: bool,
    pub messages: Option<Vec<HistoryMessagesInner>>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub pin_count: u64,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum HistoryError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidTsLatest,
    InvalidTsOldest,
    JsonNotObject,
    MissingPostType,
    MissingScope,
    NotAuthed,
    RequestTimeout,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for HistoryError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => HistoryError::AccountInactive,
            "channel_not_found" => HistoryError::ChannelNotFound,
            "invalid_arg_name" => HistoryError::InvalidArgName,
            "invalid_array_arg" => HistoryError::InvalidArrayArg,
            "invalid_auth" => HistoryError::InvalidAuth,
            "invalid_charset" => HistoryError::InvalidCharset,
            "invalid_form_data" => HistoryError::InvalidFormData,
            "invalid_json" => HistoryError::InvalidJson,
            "invalid_post_type" => HistoryError::InvalidPostType,
            "invalid_ts_latest" => HistoryError::InvalidTsLatest,
            "invalid_ts_oldest" => HistoryError::InvalidTsOldest,
            "json_not_object" => HistoryError::JsonNotObject,
            "missing_post_type" => HistoryError::MissingPostType,
            "missing_scope" => HistoryError::MissingScope,
            "not_authed" => HistoryError::NotAuthed,
            "request_timeout" => HistoryError::RequestTimeout,
            "upgrade_required" => HistoryError::UpgradeRequired,
            _ => HistoryError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for HistoryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HistoryError::AccountInactive => write!(f, "Server returned error account_inactive"),
            HistoryError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            HistoryError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            HistoryError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            HistoryError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            HistoryError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            HistoryError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            HistoryError::InvalidJson => write!(f, "Server returned error invalid_json"),
            HistoryError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            HistoryError::InvalidTsLatest => write!(f, "Server returned error invalid_ts_latest"),
            HistoryError::InvalidTsOldest => write!(f, "Server returned error invalid_ts_oldest"),
            HistoryError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            HistoryError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            HistoryError::MissingScope => write!(f, "Server returned error missing_scope"),
            HistoryError::NotAuthed => write!(f, "Server returned error not_authed"),
            HistoryError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            HistoryError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            HistoryError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            HistoryError::Unknown(ref s) => write!(f, "{}", s),
            HistoryError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for HistoryError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            HistoryError::MalformedResponse(_, ref e) => Some(e),
            HistoryError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Conversation ID to learn more about
    pub channel: Cow<'a, str>,
    /// Set this to `true` to receive the locale for this conversation. Defaults to `false`
    pub include_locale: Option<bool>,
    /// Set to `true` to include the member count for the specified conversation. Defaults to `false`
    pub include_num_members: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: InfoIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<InfoReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoReactions1Inner {
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
    pub reactions: Option<Vec<InfoReactions1Inner>>,
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
pub struct InfoReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoFilesInner {
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
    pub reactions: Option<Vec<InfoReactions2Inner>>,
    pub shares: Option<InfoShares1Inner>,
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
pub struct InfoIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoUserProfileInner {
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
pub struct InfoLatestInner {
    pub attachments: Option<Vec<InfoAttachmentsInner>>,
    pub blocks: Option<Vec<InfoBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<InfoBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<InfoCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<InfoFileInner>,
    pub files: Option<Vec<InfoFilesInner>>,
    pub icons: Option<InfoIcons1Inner>,
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
    pub reactions: Option<Vec<InfoReactions3Inner>>,
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
    pub user_profile: Option<InfoUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoExternalOrgMigrationsInner {
    pub current: Vec<InfoCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<InfoExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: InfoIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<InfoPrimaryOwnerInner>,
    pub sso_provider: Option<InfoSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: InfoTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<InfoDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<InfoLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: InfoPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<InfoShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: InfoTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    pub channel: InfoChannelInner,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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
    MissingScope,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => InfoError::AccountInactive,
            "channel_not_found" => InfoError::ChannelNotFound,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_auth" => InfoError::InvalidAuth,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_json" => InfoError::InvalidJson,
            "invalid_post_type" => InfoError::InvalidPostType,
            "json_not_object" => InfoError::JsonNotObject,
            "missing_post_type" => InfoError::MissingPostType,
            "missing_scope" => InfoError::MissingScope,
            "not_authed" => InfoError::NotAuthed,
            "request_timeout" => InfoError::RequestTimeout,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "upgrade_required" => InfoError::UpgradeRequired,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::MissingScope => write!(f, "Server returned error missing_scope"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
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
pub struct InviteRequest<'a> {
    /// The ID of the public or private channel to invite user(s) to.
    pub channel: Option<Cow<'a, str>>,
    /// A comma separated list of user IDs. Up to 1000 users may be listed.
    pub users: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: InviteIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<InviteReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteFileInner {
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
    pub reactions: Option<Vec<InviteReactions1Inner>>,
    pub shares: Option<InviteSharesInner>,
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
pub struct InviteReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteFilesInner {
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
    pub reactions: Option<Vec<InviteReactions2Inner>>,
    pub shares: Option<InviteShares1Inner>,
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
pub struct InviteIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteUserProfileInner {
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
pub struct InviteLatestInner {
    pub attachments: Option<Vec<InviteAttachmentsInner>>,
    pub blocks: Option<Vec<InviteBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<InviteBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<InviteCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<InviteFileInner>,
    pub files: Option<Vec<InviteFilesInner>>,
    pub icons: Option<InviteIcons1Inner>,
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
    pub reactions: Option<Vec<InviteReactions3Inner>>,
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
    pub user_profile: Option<InviteUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvitePurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteExternalOrgMigrationsInner {
    pub current: Vec<InviteCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvitePrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<InviteExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: InviteIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<InvitePrimaryOwnerInner>,
    pub sso_provider: Option<InviteSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: InviteTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<InviteDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<InviteLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: InvitePurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<InviteShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: InviteTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteErrorsInner {
    error: String,
    #[serde(default)]
    ok: bool,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteResponse {
    pub callstack: Option<String>,
    pub channel: Vec<InviteChannelInner>,
    error: Option<String>,
    pub errors: Option<Vec<InviteErrorsInner>>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum InviteError<E: Error> {
    AccountInactive,
    AlreadyInChannel,
    CantInvite,
    CantInviteSelf,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingCharset,
    MissingPostType,
    MissingScope,
    NoUser,
    NotAuthed,
    NotInChannel,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    TooManyUsers,
    UpgradeRequired,
    UraMaxChannels,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    UserNotFound,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for InviteError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => InviteError::AccountInactive,
            "already_in_channel" => InviteError::AlreadyInChannel,
            "cant_invite" => InviteError::CantInvite,
            "cant_invite_self" => InviteError::CantInviteSelf,
            "channel_not_found" => InviteError::ChannelNotFound,
            "invalid_arg_name" => InviteError::InvalidArgName,
            "invalid_array_arg" => InviteError::InvalidArrayArg,
            "invalid_auth" => InviteError::InvalidAuth,
            "invalid_charset" => InviteError::InvalidCharset,
            "invalid_form_data" => InviteError::InvalidFormData,
            "invalid_json" => InviteError::InvalidJson,
            "invalid_post_type" => InviteError::InvalidPostType,
            "is_archived" => InviteError::IsArchived,
            "json_not_object" => InviteError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                InviteError::MethodNotSupportedForChannelType
            }
            "missing_charset" => InviteError::MissingCharset,
            "missing_post_type" => InviteError::MissingPostType,
            "missing_scope" => InviteError::MissingScope,
            "no_user" => InviteError::NoUser,
            "not_authed" => InviteError::NotAuthed,
            "not_in_channel" => InviteError::NotInChannel,
            "request_timeout" => InviteError::RequestTimeout,
            "superfluous_charset" => InviteError::SuperfluousCharset,
            "team_added_to_org" => InviteError::TeamAddedToOrg,
            "too_many_users" => InviteError::TooManyUsers,
            "upgrade_required" => InviteError::UpgradeRequired,
            "ura_max_channels" => InviteError::UraMaxChannels,
            "user_is_bot" => InviteError::UserIsBot,
            "user_is_restricted" => InviteError::UserIsRestricted,
            "user_is_ultra_restricted" => InviteError::UserIsUltraRestricted,
            "user_not_found" => InviteError::UserNotFound,
            _ => InviteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InviteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InviteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InviteError::AlreadyInChannel => write!(f, "Server returned error already_in_channel"),
            InviteError::CantInvite => write!(f, "Server returned error cant_invite"),
            InviteError::CantInviteSelf => write!(f, "Server returned error cant_invite_self"),
            InviteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            InviteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InviteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InviteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InviteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InviteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InviteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InviteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InviteError::IsArchived => write!(f, "Server returned error is_archived"),
            InviteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InviteError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            InviteError::MissingCharset => write!(f, "Server returned error missing_charset"),
            InviteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InviteError::MissingScope => write!(f, "Server returned error missing_scope"),
            InviteError::NoUser => write!(f, "Server returned error no_user"),
            InviteError::NotAuthed => write!(f, "Server returned error not_authed"),
            InviteError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            InviteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InviteError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            InviteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InviteError::TooManyUsers => write!(f, "Server returned error too_many_users"),
            InviteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InviteError::UraMaxChannels => write!(f, "Server returned error ura_max_channels"),
            InviteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            InviteError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            InviteError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            InviteError::UserNotFound => write!(f, "Server returned error user_not_found"),
            InviteError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            InviteError::Unknown(ref s) => write!(f, "{}", s),
            InviteError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for InviteError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InviteError::MalformedResponse(_, ref e) => Some(e),
            InviteError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct JoinRequest<'a> {
    /// ID of conversation to join
    pub channel: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: JoinIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<JoinReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinFileInner {
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
    pub reactions: Option<Vec<JoinReactions1Inner>>,
    pub shares: Option<JoinSharesInner>,
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
pub struct JoinReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinFilesInner {
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
    pub reactions: Option<Vec<JoinReactions2Inner>>,
    pub shares: Option<JoinShares1Inner>,
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
pub struct JoinIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinUserProfileInner {
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
pub struct JoinLatestInner {
    pub attachments: Option<Vec<JoinAttachmentsInner>>,
    pub blocks: Option<Vec<JoinBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<JoinBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<JoinCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<JoinFileInner>,
    pub files: Option<Vec<JoinFilesInner>>,
    pub icons: Option<JoinIcons1Inner>,
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
    pub reactions: Option<Vec<JoinReactions3Inner>>,
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
    pub user_profile: Option<JoinUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinExternalOrgMigrationsInner {
    pub current: Vec<JoinCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<JoinExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: JoinIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<JoinPrimaryOwnerInner>,
    pub sso_provider: Option<JoinSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: JoinTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<JoinDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<JoinLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: JoinPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<JoinShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: JoinTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinResponseMetadataInner {
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinResponse {
    pub callstack: Option<String>,
    pub channel: Vec<JoinChannelInner>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
    pub response_metadata: Option<JoinResponseMetadataInner>,
    pub warning: Option<String>,
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

#[derive(Debug)]
pub enum JoinError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingCharset,
    MissingPostType,
    MissingScope,
    NotAuthed,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for JoinError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => JoinError::AccountInactive,
            "channel_not_found" => JoinError::ChannelNotFound,
            "invalid_arg_name" => JoinError::InvalidArgName,
            "invalid_array_arg" => JoinError::InvalidArrayArg,
            "invalid_auth" => JoinError::InvalidAuth,
            "invalid_charset" => JoinError::InvalidCharset,
            "invalid_form_data" => JoinError::InvalidFormData,
            "invalid_json" => JoinError::InvalidJson,
            "invalid_post_type" => JoinError::InvalidPostType,
            "is_archived" => JoinError::IsArchived,
            "json_not_object" => JoinError::JsonNotObject,
            "method_not_supported_for_channel_type" => JoinError::MethodNotSupportedForChannelType,
            "missing_charset" => JoinError::MissingCharset,
            "missing_post_type" => JoinError::MissingPostType,
            "missing_scope" => JoinError::MissingScope,
            "not_authed" => JoinError::NotAuthed,
            "request_timeout" => JoinError::RequestTimeout,
            "superfluous_charset" => JoinError::SuperfluousCharset,
            "team_added_to_org" => JoinError::TeamAddedToOrg,
            "upgrade_required" => JoinError::UpgradeRequired,
            "user_is_bot" => JoinError::UserIsBot,
            "user_is_restricted" => JoinError::UserIsRestricted,
            "user_is_ultra_restricted" => JoinError::UserIsUltraRestricted,
            _ => JoinError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for JoinError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            JoinError::AccountInactive => write!(f, "Server returned error account_inactive"),
            JoinError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            JoinError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            JoinError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            JoinError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            JoinError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            JoinError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            JoinError::InvalidJson => write!(f, "Server returned error invalid_json"),
            JoinError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            JoinError::IsArchived => write!(f, "Server returned error is_archived"),
            JoinError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            JoinError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            JoinError::MissingCharset => write!(f, "Server returned error missing_charset"),
            JoinError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            JoinError::MissingScope => write!(f, "Server returned error missing_scope"),
            JoinError::NotAuthed => write!(f, "Server returned error not_authed"),
            JoinError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            JoinError::SuperfluousCharset => write!(f, "Server returned error superfluous_charset"),
            JoinError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            JoinError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            JoinError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            JoinError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            JoinError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            JoinError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            JoinError::Unknown(ref s) => write!(f, "{}", s),
            JoinError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for JoinError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            JoinError::MalformedResponse(_, ref e) => Some(e),
            JoinError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct KickRequest<'a> {
    /// ID of conversation to remove user from.
    pub channel: Option<Cow<'a, str>>,
    /// User ID to be removed.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KickResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum KickError<E: Error> {
    AccountInactive,
    CantKickFromGeneral,
    CantKickSelf,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NotAuthed,
    NotInChannel,
    RequestTimeout,
    RestrictedAction,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    UserNotFound,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for KickError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => KickError::AccountInactive,
            "cant_kick_from_general" => KickError::CantKickFromGeneral,
            "cant_kick_self" => KickError::CantKickSelf,
            "channel_not_found" => KickError::ChannelNotFound,
            "invalid_arg_name" => KickError::InvalidArgName,
            "invalid_array_arg" => KickError::InvalidArrayArg,
            "invalid_auth" => KickError::InvalidAuth,
            "invalid_charset" => KickError::InvalidCharset,
            "invalid_form_data" => KickError::InvalidFormData,
            "invalid_json" => KickError::InvalidJson,
            "invalid_post_type" => KickError::InvalidPostType,
            "json_not_object" => KickError::JsonNotObject,
            "method_not_supported_for_channel_type" => KickError::MethodNotSupportedForChannelType,
            "missing_post_type" => KickError::MissingPostType,
            "missing_scope" => KickError::MissingScope,
            "not_authed" => KickError::NotAuthed,
            "not_in_channel" => KickError::NotInChannel,
            "request_timeout" => KickError::RequestTimeout,
            "restricted_action" => KickError::RestrictedAction,
            "upgrade_required" => KickError::UpgradeRequired,
            "user_is_bot" => KickError::UserIsBot,
            "user_is_restricted" => KickError::UserIsRestricted,
            "user_not_found" => KickError::UserNotFound,
            _ => KickError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for KickError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KickError::AccountInactive => write!(f, "Server returned error account_inactive"),
            KickError::CantKickFromGeneral => {
                write!(f, "Server returned error cant_kick_from_general")
            }
            KickError::CantKickSelf => write!(f, "Server returned error cant_kick_self"),
            KickError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            KickError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            KickError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            KickError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            KickError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            KickError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            KickError::InvalidJson => write!(f, "Server returned error invalid_json"),
            KickError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            KickError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            KickError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            KickError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            KickError::MissingScope => write!(f, "Server returned error missing_scope"),
            KickError::NotAuthed => write!(f, "Server returned error not_authed"),
            KickError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            KickError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            KickError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            KickError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            KickError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            KickError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            KickError::UserNotFound => write!(f, "Server returned error user_not_found"),
            KickError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            KickError::Unknown(ref s) => write!(f, "{}", s),
            KickError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for KickError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            KickError::MalformedResponse(_, ref e) => Some(e),
            KickError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct LeaveRequest<'a> {
    /// Conversation to leave
    pub channel: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LeaveResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    pub not_in_channel: Option<bool>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum LeaveError<E: Error> {
    AccountInactive,
    CantLeaveGeneral,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    LastMember,
    MethodNotSupportedForChannelType,
    MissingCharset,
    MissingPostType,
    MissingScope,
    NotAuthed,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for LeaveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => LeaveError::AccountInactive,
            "cant_leave_general" => LeaveError::CantLeaveGeneral,
            "channel_not_found" => LeaveError::ChannelNotFound,
            "invalid_arg_name" => LeaveError::InvalidArgName,
            "invalid_array_arg" => LeaveError::InvalidArrayArg,
            "invalid_auth" => LeaveError::InvalidAuth,
            "invalid_charset" => LeaveError::InvalidCharset,
            "invalid_form_data" => LeaveError::InvalidFormData,
            "invalid_json" => LeaveError::InvalidJson,
            "invalid_post_type" => LeaveError::InvalidPostType,
            "is_archived" => LeaveError::IsArchived,
            "json_not_object" => LeaveError::JsonNotObject,
            "last_member" => LeaveError::LastMember,
            "method_not_supported_for_channel_type" => LeaveError::MethodNotSupportedForChannelType,
            "missing_charset" => LeaveError::MissingCharset,
            "missing_post_type" => LeaveError::MissingPostType,
            "missing_scope" => LeaveError::MissingScope,
            "not_authed" => LeaveError::NotAuthed,
            "request_timeout" => LeaveError::RequestTimeout,
            "superfluous_charset" => LeaveError::SuperfluousCharset,
            "team_added_to_org" => LeaveError::TeamAddedToOrg,
            "upgrade_required" => LeaveError::UpgradeRequired,
            "user_is_bot" => LeaveError::UserIsBot,
            "user_is_restricted" => LeaveError::UserIsRestricted,
            "user_is_ultra_restricted" => LeaveError::UserIsUltraRestricted,
            _ => LeaveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for LeaveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LeaveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            LeaveError::CantLeaveGeneral => write!(f, "Server returned error cant_leave_general"),
            LeaveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            LeaveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            LeaveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            LeaveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            LeaveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            LeaveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            LeaveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            LeaveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            LeaveError::IsArchived => write!(f, "Server returned error is_archived"),
            LeaveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            LeaveError::LastMember => write!(f, "Server returned error last_member"),
            LeaveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            LeaveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            LeaveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            LeaveError::MissingScope => write!(f, "Server returned error missing_scope"),
            LeaveError::NotAuthed => write!(f, "Server returned error not_authed"),
            LeaveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            LeaveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            LeaveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            LeaveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            LeaveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            LeaveError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            LeaveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            LeaveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            LeaveError::Unknown(ref s) => write!(f, "{}", s),
            LeaveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for LeaveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LeaveError::MalformedResponse(_, ref e) => Some(e),
            LeaveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ListRequest<'a> {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// Set to `true` to exclude archived channels from the list
    pub exclude_archived: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
    pub limit: Option<u64>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    pub types: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
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
pub struct ListLatestInner {
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
pub struct ListPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListExternalOrgMigrationsInner {
    pub current: Vec<ListCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<ListExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: ListIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<ListPrimaryOwnerInner>,
    pub sso_provider: Option<ListSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: ListTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListChannelsInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<ListDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<ListLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: ListPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<ListShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: ListTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub callstack: Option<String>,
    pub channels: Vec<ListChannelsInner>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
    pub response_metadata: Option<ListResponseMetadataInner>,
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
    MissingScope,
    NotAuthed,
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
            "missing_scope" => ListError::MissingScope,
            "not_authed" => ListError::NotAuthed,
            "request_timeout" => ListError::RequestTimeout,
            "upgrade_required" => ListError::UpgradeRequired,
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
            ListError::MissingScope => write!(f, "Server returned error missing_scope"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
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

#[derive(Clone, Default, Debug)]
pub struct MarkRequest<'a> {
    /// Channel or conversation to set the read cursor for.
    pub channel: Option<Cow<'a, str>>,
    /// Unique identifier of message you want marked as most recently seen in this conversation.
    pub ts: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarkResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum MarkError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidTimestamp,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NotAllowedTokenType,
    NotAuthed,
    NotInChannel,
    RequestTimeout,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for MarkError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => MarkError::AccountInactive,
            "channel_not_found" => MarkError::ChannelNotFound,
            "invalid_arg_name" => MarkError::InvalidArgName,
            "invalid_array_arg" => MarkError::InvalidArrayArg,
            "invalid_auth" => MarkError::InvalidAuth,
            "invalid_charset" => MarkError::InvalidCharset,
            "invalid_form_data" => MarkError::InvalidFormData,
            "invalid_json" => MarkError::InvalidJson,
            "invalid_post_type" => MarkError::InvalidPostType,
            "invalid_timestamp" => MarkError::InvalidTimestamp,
            "json_not_object" => MarkError::JsonNotObject,
            "method_not_supported_for_channel_type" => MarkError::MethodNotSupportedForChannelType,
            "missing_post_type" => MarkError::MissingPostType,
            "missing_scope" => MarkError::MissingScope,
            "not_allowed_token_type" => MarkError::NotAllowedTokenType,
            "not_authed" => MarkError::NotAuthed,
            "not_in_channel" => MarkError::NotInChannel,
            "request_timeout" => MarkError::RequestTimeout,
            "upgrade_required" => MarkError::UpgradeRequired,
            _ => MarkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MarkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MarkError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MarkError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MarkError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MarkError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MarkError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MarkError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MarkError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MarkError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MarkError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MarkError::InvalidTimestamp => write!(f, "Server returned error invalid_timestamp"),
            MarkError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MarkError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            MarkError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MarkError::MissingScope => write!(f, "Server returned error missing_scope"),
            MarkError::NotAllowedTokenType => {
                write!(f, "Server returned error not_allowed_token_type")
            }
            MarkError::NotAuthed => write!(f, "Server returned error not_authed"),
            MarkError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            MarkError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            MarkError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            MarkError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            MarkError::Unknown(ref s) => write!(f, "{}", s),
            MarkError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for MarkError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MarkError::MalformedResponse(_, ref e) => Some(e),
            MarkError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct MembersRequest<'a> {
    /// ID of the conversation to retrieve members for
    pub channel: Option<Cow<'a, str>>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MembersResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MembersResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub members: Vec<String>,
    #[serde(default)]
    ok: bool,
    pub response_metadata: MembersResponseMetadataInner,
}

impl<E: Error> Into<Result<MembersResponse, MembersError<E>>> for MembersResponse {
    fn into(self) -> Result<MembersResponse, MembersError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum MembersError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    FetchMembersFailed,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidCursor,
    InvalidFormData,
    InvalidJson,
    InvalidLimit,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
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

impl<'a, E: Error> From<&'a str> for MembersError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => MembersError::AccountInactive,
            "channel_not_found" => MembersError::ChannelNotFound,
            "fetch_members_failed" => MembersError::FetchMembersFailed,
            "invalid_arg_name" => MembersError::InvalidArgName,
            "invalid_array_arg" => MembersError::InvalidArrayArg,
            "invalid_auth" => MembersError::InvalidAuth,
            "invalid_charset" => MembersError::InvalidCharset,
            "invalid_cursor" => MembersError::InvalidCursor,
            "invalid_form_data" => MembersError::InvalidFormData,
            "invalid_json" => MembersError::InvalidJson,
            "invalid_limit" => MembersError::InvalidLimit,
            "invalid_post_type" => MembersError::InvalidPostType,
            "json_not_object" => MembersError::JsonNotObject,
            "missing_post_type" => MembersError::MissingPostType,
            "not_authed" => MembersError::NotAuthed,
            "request_timeout" => MembersError::RequestTimeout,
            "team_added_to_org" => MembersError::TeamAddedToOrg,
            "upgrade_required" => MembersError::UpgradeRequired,
            _ => MembersError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MembersError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MembersError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MembersError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MembersError::FetchMembersFailed => {
                write!(f, "Server returned error fetch_members_failed")
            }
            MembersError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MembersError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MembersError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MembersError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MembersError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            MembersError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MembersError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MembersError::InvalidLimit => write!(f, "Server returned error invalid_limit"),
            MembersError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MembersError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MembersError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MembersError::NotAuthed => write!(f, "Server returned error not_authed"),
            MembersError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            MembersError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            MembersError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            MembersError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            MembersError::Unknown(ref s) => write!(f, "{}", s),
            MembersError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for MembersError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MembersError::MalformedResponse(_, ref e) => Some(e),
            MembersError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct OpenRequest<'a> {
    /// Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead.
    pub channel: Option<Cow<'a, str>>,
    /// Boolean, indicates you want the full IM channel definition in the response.
    pub return_im: Option<bool>,
    /// Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`.
    pub users: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: OpenIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<OpenReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenFileInner {
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
    pub reactions: Option<Vec<OpenReactions1Inner>>,
    pub shares: Option<OpenSharesInner>,
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
pub struct OpenReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenFilesInner {
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
    pub reactions: Option<Vec<OpenReactions2Inner>>,
    pub shares: Option<OpenShares1Inner>,
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
pub struct OpenIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenUserProfileInner {
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
pub struct OpenLatestInner {
    pub attachments: Option<Vec<OpenAttachmentsInner>>,
    pub blocks: Option<Vec<OpenBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<OpenBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<OpenCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<OpenFileInner>,
    pub files: Option<Vec<OpenFilesInner>>,
    pub icons: Option<OpenIcons1Inner>,
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
    pub reactions: Option<Vec<OpenReactions3Inner>>,
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
    pub user_profile: Option<OpenUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenExternalOrgMigrationsInner {
    pub current: Vec<OpenCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<OpenExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: OpenIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<OpenPrimaryOwnerInner>,
    pub sso_provider: Option<OpenSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: OpenTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<OpenDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<OpenLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: OpenPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<OpenShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: OpenTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenResponse {
    pub already_open: Option<bool>,
    pub callstack: Option<String>,
    pub channel: Vec<Vec<OpenChannelInner>>,
    error: Option<String>,
    pub no_op: Option<bool>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<OpenResponse, OpenError<E>>> for OpenResponse {
    fn into(self) -> Result<OpenResponse, OpenError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum OpenError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidUserCombination,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    NotAuthed,
    NotEnoughUsers,
    RequestTimeout,
    TeamAddedToOrg,
    TooManyUsers,
    UpgradeRequired,
    UserDisabled,
    UserNotFound,
    UserNotVisible,
    UsersListNotSupplied,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for OpenError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => OpenError::AccountInactive,
            "channel_not_found" => OpenError::ChannelNotFound,
            "invalid_arg_name" => OpenError::InvalidArgName,
            "invalid_array_arg" => OpenError::InvalidArrayArg,
            "invalid_auth" => OpenError::InvalidAuth,
            "invalid_charset" => OpenError::InvalidCharset,
            "invalid_form_data" => OpenError::InvalidFormData,
            "invalid_json" => OpenError::InvalidJson,
            "invalid_post_type" => OpenError::InvalidPostType,
            "invalid_user_combination" => OpenError::InvalidUserCombination,
            "json_not_object" => OpenError::JsonNotObject,
            "method_not_supported_for_channel_type" => OpenError::MethodNotSupportedForChannelType,
            "missing_post_type" => OpenError::MissingPostType,
            "not_authed" => OpenError::NotAuthed,
            "not_enough_users" => OpenError::NotEnoughUsers,
            "request_timeout" => OpenError::RequestTimeout,
            "team_added_to_org" => OpenError::TeamAddedToOrg,
            "too_many_users" => OpenError::TooManyUsers,
            "upgrade_required" => OpenError::UpgradeRequired,
            "user_disabled" => OpenError::UserDisabled,
            "user_not_found" => OpenError::UserNotFound,
            "user_not_visible" => OpenError::UserNotVisible,
            "users_list_not_supplied" => OpenError::UsersListNotSupplied,
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpenError::AccountInactive => write!(f, "Server returned error account_inactive"),
            OpenError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            OpenError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            OpenError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            OpenError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            OpenError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            OpenError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            OpenError::InvalidJson => write!(f, "Server returned error invalid_json"),
            OpenError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            OpenError::InvalidUserCombination => {
                write!(f, "Server returned error invalid_user_combination")
            }
            OpenError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            OpenError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            OpenError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            OpenError::NotAuthed => write!(f, "Server returned error not_authed"),
            OpenError::NotEnoughUsers => write!(f, "Server returned error not_enough_users"),
            OpenError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            OpenError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            OpenError::TooManyUsers => write!(f, "Server returned error too_many_users"),
            OpenError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            OpenError::UserDisabled => write!(f, "Server returned error user_disabled"),
            OpenError::UserNotFound => write!(f, "Server returned error user_not_found"),
            OpenError::UserNotVisible => write!(f, "Server returned error user_not_visible"),
            OpenError::UsersListNotSupplied => {
                write!(f, "Server returned error users_list_not_supplied")
            }
            OpenError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            OpenError::Unknown(ref s) => write!(f, "{}", s),
            OpenError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for OpenError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OpenError::MalformedResponse(_, ref e) => Some(e),
            OpenError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RenameRequest<'a> {
    /// ID of conversation to rename
    pub channel: Option<Cow<'a, str>>,
    /// New name for conversation.
    pub name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: RenameIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<RenameReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameFileInner {
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
    pub reactions: Option<Vec<RenameReactions1Inner>>,
    pub shares: Option<RenameSharesInner>,
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
pub struct RenameReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameFilesInner {
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
    pub reactions: Option<Vec<RenameReactions2Inner>>,
    pub shares: Option<RenameShares1Inner>,
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
pub struct RenameIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameUserProfileInner {
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
pub struct RenameLatestInner {
    pub attachments: Option<Vec<RenameAttachmentsInner>>,
    pub blocks: Option<Vec<RenameBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<RenameBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<RenameCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<RenameFileInner>,
    pub files: Option<Vec<RenameFilesInner>>,
    pub icons: Option<RenameIcons1Inner>,
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
    pub reactions: Option<Vec<RenameReactions3Inner>>,
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
    pub user_profile: Option<RenameUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenamePurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameExternalOrgMigrationsInner {
    pub current: Vec<RenameCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenamePrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<RenameExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: RenameIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<RenamePrimaryOwnerInner>,
    pub sso_provider: Option<RenameSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: RenameTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<RenameDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<RenameLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: RenamePurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<RenameShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: RenameTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameResponse {
    pub callstack: Option<String>,
    pub channel: Vec<RenameChannelInner>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum RenameError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidName,
    InvalidNameMaxlength,
    InvalidNamePunctuation,
    InvalidNameRequired,
    InvalidNameSpecials,
    InvalidPostType,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NameTaken,
    NotAuthed,
    NotAuthorized,
    NotInChannel,
    RequestTimeout,
    UpgradeRequired,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RenameError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => RenameError::AccountInactive,
            "channel_not_found" => RenameError::ChannelNotFound,
            "invalid_arg_name" => RenameError::InvalidArgName,
            "invalid_array_arg" => RenameError::InvalidArrayArg,
            "invalid_auth" => RenameError::InvalidAuth,
            "invalid_charset" => RenameError::InvalidCharset,
            "invalid_form_data" => RenameError::InvalidFormData,
            "invalid_json" => RenameError::InvalidJson,
            "invalid_name" => RenameError::InvalidName,
            "invalid_name_maxlength" => RenameError::InvalidNameMaxlength,
            "invalid_name_punctuation" => RenameError::InvalidNamePunctuation,
            "invalid_name_required" => RenameError::InvalidNameRequired,
            "invalid_name_specials" => RenameError::InvalidNameSpecials,
            "invalid_post_type" => RenameError::InvalidPostType,
            "json_not_object" => RenameError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                RenameError::MethodNotSupportedForChannelType
            }
            "missing_post_type" => RenameError::MissingPostType,
            "missing_scope" => RenameError::MissingScope,
            "name_taken" => RenameError::NameTaken,
            "not_authed" => RenameError::NotAuthed,
            "not_authorized" => RenameError::NotAuthorized,
            "not_in_channel" => RenameError::NotInChannel,
            "request_timeout" => RenameError::RequestTimeout,
            "upgrade_required" => RenameError::UpgradeRequired,
            "user_is_restricted" => RenameError::UserIsRestricted,
            _ => RenameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RenameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RenameError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RenameError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            RenameError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RenameError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RenameError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RenameError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RenameError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RenameError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RenameError::InvalidName => write!(f, "Server returned error invalid_name"),
            RenameError::InvalidNameMaxlength => {
                write!(f, "Server returned error invalid_name_maxlength")
            }
            RenameError::InvalidNamePunctuation => {
                write!(f, "Server returned error invalid_name_punctuation")
            }
            RenameError::InvalidNameRequired => {
                write!(f, "Server returned error invalid_name_required")
            }
            RenameError::InvalidNameSpecials => {
                write!(f, "Server returned error invalid_name_specials")
            }
            RenameError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RenameError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RenameError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            RenameError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RenameError::MissingScope => write!(f, "Server returned error missing_scope"),
            RenameError::NameTaken => write!(f, "Server returned error name_taken"),
            RenameError::NotAuthed => write!(f, "Server returned error not_authed"),
            RenameError::NotAuthorized => write!(f, "Server returned error not_authorized"),
            RenameError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            RenameError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RenameError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RenameError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            RenameError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RenameError::Unknown(ref s) => write!(f, "{}", s),
            RenameError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RenameError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RenameError::MalformedResponse(_, ref e) => Some(e),
            RenameError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RepliesRequest<'a> {
    /// Conversation ID to fetch thread from.
    pub channel: Option<Cow<'a, str>>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    pub inclusive: Option<bool>,
    /// End of time range of messages to include in results.
    pub latest: Option<f64>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<f64>,
    /// Unique identifier of a thread's parent message. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message.
    pub ts: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepliesUserProfileInner {
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
pub struct RepliesMessagesInner {
    pub last_read: Option<String>,
    pub latest_reply: Option<String>,
    pub reply_count: u64,
    pub reply_users: Option<Vec<String>>,
    pub reply_users_count: Option<u64>,
    pub source_team: Option<String>,
    pub subscribed: bool,
    pub team: Option<String>,
    pub text: String,
    pub thread_ts: String,
    pub ts: String,
    pub r#type: String,
    pub unread_count: Option<u64>,
    pub user: String,
    pub user_profile: Option<RepliesUserProfileInner>,
    pub user_team: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepliesResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub has_more: Option<bool>,
    pub messages: Vec<Vec<RepliesMessagesInner>>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum RepliesError<E: Error> {
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
    MissingScope,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    ThreadNotFound,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RepliesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => RepliesError::AccountInactive,
            "channel_not_found" => RepliesError::ChannelNotFound,
            "invalid_arg_name" => RepliesError::InvalidArgName,
            "invalid_array_arg" => RepliesError::InvalidArrayArg,
            "invalid_auth" => RepliesError::InvalidAuth,
            "invalid_charset" => RepliesError::InvalidCharset,
            "invalid_form_data" => RepliesError::InvalidFormData,
            "invalid_json" => RepliesError::InvalidJson,
            "invalid_post_type" => RepliesError::InvalidPostType,
            "json_not_object" => RepliesError::JsonNotObject,
            "missing_post_type" => RepliesError::MissingPostType,
            "missing_scope" => RepliesError::MissingScope,
            "not_authed" => RepliesError::NotAuthed,
            "request_timeout" => RepliesError::RequestTimeout,
            "team_added_to_org" => RepliesError::TeamAddedToOrg,
            "thread_not_found" => RepliesError::ThreadNotFound,
            "upgrade_required" => RepliesError::UpgradeRequired,
            _ => RepliesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RepliesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RepliesError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RepliesError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            RepliesError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RepliesError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RepliesError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RepliesError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RepliesError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RepliesError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RepliesError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RepliesError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RepliesError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RepliesError::MissingScope => write!(f, "Server returned error missing_scope"),
            RepliesError::NotAuthed => write!(f, "Server returned error not_authed"),
            RepliesError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RepliesError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RepliesError::ThreadNotFound => write!(f, "Server returned error thread_not_found"),
            RepliesError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            RepliesError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            RepliesError::Unknown(ref s) => write!(f, "{}", s),
            RepliesError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for RepliesError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RepliesError::MalformedResponse(_, ref e) => Some(e),
            RepliesError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetPurposeRequest<'a> {
    /// Conversation to set the purpose of
    pub channel: Option<Cow<'a, str>>,
    /// A new, specialer purpose
    pub purpose: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: SetPurposeIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<SetPurposeReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeFileInner {
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
    pub reactions: Option<Vec<SetPurposeReactions1Inner>>,
    pub shares: Option<SetPurposeSharesInner>,
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
pub struct SetPurposeReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeFilesInner {
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
    pub reactions: Option<Vec<SetPurposeReactions2Inner>>,
    pub shares: Option<SetPurposeShares1Inner>,
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
pub struct SetPurposeIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeUserProfileInner {
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
pub struct SetPurposeLatestInner {
    pub attachments: Option<Vec<SetPurposeAttachmentsInner>>,
    pub blocks: Option<Vec<SetPurposeBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<SetPurposeBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<SetPurposeCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<SetPurposeFileInner>,
    pub files: Option<Vec<SetPurposeFilesInner>>,
    pub icons: Option<SetPurposeIcons1Inner>,
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
    pub reactions: Option<Vec<SetPurposeReactions3Inner>>,
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
    pub user_profile: Option<SetPurposeUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposePurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeExternalOrgMigrationsInner {
    pub current: Vec<SetPurposeCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposePrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<SetPurposeExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: SetPurposeIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<SetPurposePrimaryOwnerInner>,
    pub sso_provider: Option<SetPurposeSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: SetPurposeTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<SetPurposeDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<SetPurposeLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: SetPurposePurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<SetPurposeShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: SetPurposeTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPurposeResponse {
    pub callstack: Option<String>,
    pub channel: Vec<SetPurposeChannelInner>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum SetPurposeError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NotAuthed,
    NotInChannel,
    RequestTimeout,
    TeamAddedToOrg,
    TooLong,
    UpgradeRequired,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetPurposeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetPurposeError::AccountInactive,
            "channel_not_found" => SetPurposeError::ChannelNotFound,
            "invalid_arg_name" => SetPurposeError::InvalidArgName,
            "invalid_array_arg" => SetPurposeError::InvalidArrayArg,
            "invalid_auth" => SetPurposeError::InvalidAuth,
            "invalid_charset" => SetPurposeError::InvalidCharset,
            "invalid_form_data" => SetPurposeError::InvalidFormData,
            "invalid_json" => SetPurposeError::InvalidJson,
            "invalid_post_type" => SetPurposeError::InvalidPostType,
            "is_archived" => SetPurposeError::IsArchived,
            "json_not_object" => SetPurposeError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                SetPurposeError::MethodNotSupportedForChannelType
            }
            "missing_post_type" => SetPurposeError::MissingPostType,
            "missing_scope" => SetPurposeError::MissingScope,
            "not_authed" => SetPurposeError::NotAuthed,
            "not_in_channel" => SetPurposeError::NotInChannel,
            "request_timeout" => SetPurposeError::RequestTimeout,
            "team_added_to_org" => SetPurposeError::TeamAddedToOrg,
            "too_long" => SetPurposeError::TooLong,
            "upgrade_required" => SetPurposeError::UpgradeRequired,
            "user_is_restricted" => SetPurposeError::UserIsRestricted,
            _ => SetPurposeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPurposeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetPurposeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetPurposeError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            SetPurposeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetPurposeError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            SetPurposeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetPurposeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetPurposeError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            SetPurposeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetPurposeError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            SetPurposeError::IsArchived => write!(f, "Server returned error is_archived"),
            SetPurposeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetPurposeError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            SetPurposeError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            SetPurposeError::MissingScope => write!(f, "Server returned error missing_scope"),
            SetPurposeError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetPurposeError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            SetPurposeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetPurposeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetPurposeError::TooLong => write!(f, "Server returned error too_long"),
            SetPurposeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetPurposeError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            SetPurposeError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetPurposeError::Unknown(ref s) => write!(f, "{}", s),
            SetPurposeError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetPurposeError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetPurposeError::MalformedResponse(_, ref e) => Some(e),
            SetPurposeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetTopicRequest<'a> {
    /// Conversation to set the topic of
    pub channel: Option<Cow<'a, str>>,
    /// The new topic string. Does not support formatting or linkification.
    pub topic: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: SetTopicIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<SetTopicReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicFileInner {
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
    pub reactions: Option<Vec<SetTopicReactions1Inner>>,
    pub shares: Option<SetTopicSharesInner>,
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
pub struct SetTopicReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicFilesInner {
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
    pub reactions: Option<Vec<SetTopicReactions2Inner>>,
    pub shares: Option<SetTopicShares1Inner>,
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
pub struct SetTopicIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicUserProfileInner {
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
pub struct SetTopicLatestInner {
    pub attachments: Option<Vec<SetTopicAttachmentsInner>>,
    pub blocks: Option<Vec<SetTopicBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<SetTopicBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<SetTopicCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<SetTopicFileInner>,
    pub files: Option<Vec<SetTopicFilesInner>>,
    pub icons: Option<SetTopicIcons1Inner>,
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
    pub reactions: Option<Vec<SetTopicReactions3Inner>>,
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
    pub user_profile: Option<SetTopicUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicExternalOrgMigrationsInner {
    pub current: Vec<SetTopicCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicIconInner {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_230: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicTeamInner {
    pub archived: Option<bool>,
    pub avatar_base_url: Option<String>,
    pub created: Option<u64>,
    pub date_create: Option<u64>,
    pub deleted: Option<bool>,
    pub description: Option<String>,
    pub discoverable: Option<Vec<String>>,
    pub domain: String,
    pub email_domain: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub external_org_migrations: Option<SetTopicExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: SetTopicIconInner,
    pub id: String,
    pub is_assigned: Option<bool>,
    pub is_enterprise: Option<u64>,
    pub is_over_storage_limit: Option<bool>,
    pub limit_ts: Option<u64>,
    pub locale: Option<String>,
    pub messages_count: Option<u64>,
    pub msg_edit_window_mins: Option<u64>,
    pub name: String,
    pub over_integrations_limit: Option<bool>,
    pub over_storage_limit: Option<bool>,
    pub pay_prod_cur: Option<String>,
    pub plan: Option<String>,
    pub primary_owner: Option<SetTopicPrimaryOwnerInner>,
    pub sso_provider: Option<SetTopicSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: SetTopicTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicChannelInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<SetTopicDisplayCountsInner>,
    pub enterprise_id: Option<String>,
    pub has_pins: Option<bool>,
    pub id: String,
    pub internal_team_ids: Option<Vec<String>>,
    pub is_archived: bool,
    pub is_channel: bool,
    pub is_ext_shared: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_general: bool,
    pub is_global_shared: Option<bool>,
    pub is_group: bool,
    pub is_im: bool,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_open: Option<bool>,
    pub is_org_default: Option<bool>,
    pub is_org_mandatory: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_starred: Option<bool>,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<SetTopicLatestInner>>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub parent_conversation: Option<Vec<String>>,
    pub pending_connected_team_ids: Option<Vec<String>>,
    pub pending_shared: Option<Vec<String>>,
    pub pin_count: Option<u64>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<f64>,
    pub purpose: SetTopicPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<SetTopicShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: SetTopicTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTopicResponse {
    pub callstack: Option<String>,
    pub channel: Vec<SetTopicChannelInner>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum SetTopicError<E: Error> {
    AccountInactive,
    ChannelNotFound,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    IsArchived,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NotAuthed,
    NotInChannel,
    RequestTimeout,
    TeamAddedToOrg,
    TooLong,
    UpgradeRequired,
    UserIsRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetTopicError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetTopicError::AccountInactive,
            "channel_not_found" => SetTopicError::ChannelNotFound,
            "invalid_arg_name" => SetTopicError::InvalidArgName,
            "invalid_array_arg" => SetTopicError::InvalidArrayArg,
            "invalid_auth" => SetTopicError::InvalidAuth,
            "invalid_charset" => SetTopicError::InvalidCharset,
            "invalid_form_data" => SetTopicError::InvalidFormData,
            "invalid_json" => SetTopicError::InvalidJson,
            "invalid_post_type" => SetTopicError::InvalidPostType,
            "is_archived" => SetTopicError::IsArchived,
            "json_not_object" => SetTopicError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                SetTopicError::MethodNotSupportedForChannelType
            }
            "missing_post_type" => SetTopicError::MissingPostType,
            "missing_scope" => SetTopicError::MissingScope,
            "not_authed" => SetTopicError::NotAuthed,
            "not_in_channel" => SetTopicError::NotInChannel,
            "request_timeout" => SetTopicError::RequestTimeout,
            "team_added_to_org" => SetTopicError::TeamAddedToOrg,
            "too_long" => SetTopicError::TooLong,
            "upgrade_required" => SetTopicError::UpgradeRequired,
            "user_is_restricted" => SetTopicError::UserIsRestricted,
            _ => SetTopicError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetTopicError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetTopicError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetTopicError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            SetTopicError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetTopicError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetTopicError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetTopicError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetTopicError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetTopicError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetTopicError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetTopicError::IsArchived => write!(f, "Server returned error is_archived"),
            SetTopicError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetTopicError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            SetTopicError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetTopicError::MissingScope => write!(f, "Server returned error missing_scope"),
            SetTopicError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetTopicError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            SetTopicError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetTopicError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetTopicError::TooLong => write!(f, "Server returned error too_long"),
            SetTopicError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetTopicError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            SetTopicError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetTopicError::Unknown(ref s) => write!(f, "{}", s),
            SetTopicError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetTopicError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetTopicError::MalformedResponse(_, ref e) => Some(e),
            SetTopicError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct UnarchiveRequest<'a> {
    /// ID of conversation to unarchive
    pub channel: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnarchiveResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub needed: Option<String>,
    #[serde(default)]
    ok: bool,
    pub provided: Option<String>,
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

#[derive(Debug)]
pub enum UnarchiveError<E: Error> {
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
    MethodNotSupportedForChannelType,
    MissingCharset,
    MissingPostType,
    MissingScope,
    NotArchived,
    NotAuthed,
    RequestTimeout,
    SuperfluousCharset,
    TeamAddedToOrg,
    UpgradeRequired,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UnarchiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => UnarchiveError::AccountInactive,
            "channel_not_found" => UnarchiveError::ChannelNotFound,
            "invalid_arg_name" => UnarchiveError::InvalidArgName,
            "invalid_array_arg" => UnarchiveError::InvalidArrayArg,
            "invalid_auth" => UnarchiveError::InvalidAuth,
            "invalid_charset" => UnarchiveError::InvalidCharset,
            "invalid_form_data" => UnarchiveError::InvalidFormData,
            "invalid_json" => UnarchiveError::InvalidJson,
            "invalid_post_type" => UnarchiveError::InvalidPostType,
            "json_not_object" => UnarchiveError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                UnarchiveError::MethodNotSupportedForChannelType
            }
            "missing_charset" => UnarchiveError::MissingCharset,
            "missing_post_type" => UnarchiveError::MissingPostType,
            "missing_scope" => UnarchiveError::MissingScope,
            "not_archived" => UnarchiveError::NotArchived,
            "not_authed" => UnarchiveError::NotAuthed,
            "request_timeout" => UnarchiveError::RequestTimeout,
            "superfluous_charset" => UnarchiveError::SuperfluousCharset,
            "team_added_to_org" => UnarchiveError::TeamAddedToOrg,
            "upgrade_required" => UnarchiveError::UpgradeRequired,
            "user_is_bot" => UnarchiveError::UserIsBot,
            "user_is_restricted" => UnarchiveError::UserIsRestricted,
            "user_is_ultra_restricted" => UnarchiveError::UserIsUltraRestricted,
            _ => UnarchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnarchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnarchiveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UnarchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            UnarchiveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UnarchiveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UnarchiveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UnarchiveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UnarchiveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UnarchiveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UnarchiveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UnarchiveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UnarchiveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            UnarchiveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            UnarchiveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UnarchiveError::MissingScope => write!(f, "Server returned error missing_scope"),
            UnarchiveError::NotArchived => write!(f, "Server returned error not_archived"),
            UnarchiveError::NotAuthed => write!(f, "Server returned error not_authed"),
            UnarchiveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UnarchiveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
            UnarchiveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UnarchiveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UnarchiveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UnarchiveError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            UnarchiveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            UnarchiveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UnarchiveError::Unknown(ref s) => write!(f, "{}", s),
            UnarchiveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UnarchiveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UnarchiveError::MalformedResponse(_, ref e) => Some(e),
            UnarchiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
