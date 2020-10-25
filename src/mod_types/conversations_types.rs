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
pub struct JoinRequest {
    /// ID of conversation to join
    pub channel: Option<String>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    IsArchived,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    TeamAddedToOrg,
    MissingCharset,
    SuperfluousCharset,
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
            "method_not_supported_for_channel_type" => JoinError::MethodNotSupportedForChannelType,
            "missing_scope" => JoinError::MissingScope,
            "channel_not_found" => JoinError::ChannelNotFound,
            "is_archived" => JoinError::IsArchived,
            "not_authed" => JoinError::NotAuthed,
            "invalid_auth" => JoinError::InvalidAuth,
            "account_inactive" => JoinError::AccountInactive,
            "user_is_bot" => JoinError::UserIsBot,
            "user_is_restricted" => JoinError::UserIsRestricted,
            "user_is_ultra_restricted" => JoinError::UserIsUltraRestricted,
            "invalid_arg_name" => JoinError::InvalidArgName,
            "invalid_array_arg" => JoinError::InvalidArrayArg,
            "invalid_charset" => JoinError::InvalidCharset,
            "invalid_form_data" => JoinError::InvalidFormData,
            "invalid_post_type" => JoinError::InvalidPostType,
            "missing_post_type" => JoinError::MissingPostType,
            "invalid_json" => JoinError::InvalidJson,
            "json_not_object" => JoinError::JsonNotObject,
            "request_timeout" => JoinError::RequestTimeout,
            "upgrade_required" => JoinError::UpgradeRequired,
            "team_added_to_org" => JoinError::TeamAddedToOrg,
            "missing_charset" => JoinError::MissingCharset,
            "superfluous_charset" => JoinError::SuperfluousCharset,
            _ => JoinError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for JoinError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            JoinError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            JoinError::MissingScope => write!(f, "Server returned error missing_scope"),
            JoinError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            JoinError::IsArchived => write!(f, "Server returned error is_archived"),
            JoinError::NotAuthed => write!(f, "Server returned error not_authed"),
            JoinError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            JoinError::AccountInactive => write!(f, "Server returned error account_inactive"),
            JoinError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            JoinError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            JoinError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            JoinError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            JoinError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            JoinError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            JoinError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            JoinError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            JoinError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            JoinError::InvalidJson => write!(f, "Server returned error invalid_json"),
            JoinError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            JoinError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            JoinError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            JoinError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            JoinError::MissingCharset => write!(f, "Server returned error missing_charset"),
            JoinError::SuperfluousCharset => write!(f, "Server returned error superfluous_charset"),
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
pub struct CloseRequest {
    /// Conversation to close.
    pub channel: Option<String>,
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
    MethodNotSupportedForChannelType,
    ChannelNotFound,
    UserDoesNotOwnChannel,
    MissingScope,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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

impl<'a, E: Error> From<&'a str> for CloseError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "method_not_supported_for_channel_type" => CloseError::MethodNotSupportedForChannelType,
            "channel_not_found" => CloseError::ChannelNotFound,
            "user_does_not_own_channel" => CloseError::UserDoesNotOwnChannel,
            "missing_scope" => CloseError::MissingScope,
            "not_authed" => CloseError::NotAuthed,
            "invalid_auth" => CloseError::InvalidAuth,
            "account_inactive" => CloseError::AccountInactive,
            "invalid_arg_name" => CloseError::InvalidArgName,
            "invalid_array_arg" => CloseError::InvalidArrayArg,
            "invalid_charset" => CloseError::InvalidCharset,
            "invalid_form_data" => CloseError::InvalidFormData,
            "invalid_post_type" => CloseError::InvalidPostType,
            "missing_post_type" => CloseError::MissingPostType,
            "team_added_to_org" => CloseError::TeamAddedToOrg,
            "invalid_json" => CloseError::InvalidJson,
            "json_not_object" => CloseError::JsonNotObject,
            "request_timeout" => CloseError::RequestTimeout,
            "upgrade_required" => CloseError::UpgradeRequired,
            _ => CloseError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CloseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CloseError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            CloseError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            CloseError::UserDoesNotOwnChannel => {
                write!(f, "Server returned error user_does_not_own_channel")
            }
            CloseError::MissingScope => write!(f, "Server returned error missing_scope"),
            CloseError::NotAuthed => write!(f, "Server returned error not_authed"),
            CloseError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CloseError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CloseError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CloseError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CloseError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CloseError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CloseError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CloseError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CloseError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CloseError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CloseError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CloseError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CloseError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct InfoRequest {
    /// Conversation ID to learn more about
    pub channel: Option<String>,
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
    pub priority: Option<u64>,
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
    pub channel: Vec<InfoChannelInner>,
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
    MissingScope,
    ChannelNotFound,
    TeamAddedToOrg,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
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

impl<'a, E: Error> From<&'a str> for InfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "missing_scope" => InfoError::MissingScope,
            "channel_not_found" => InfoError::ChannelNotFound,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "not_authed" => InfoError::NotAuthed,
            "invalid_auth" => InfoError::InvalidAuth,
            "account_inactive" => InfoError::AccountInactive,
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_post_type" => InfoError::InvalidPostType,
            "missing_post_type" => InfoError::MissingPostType,
            "invalid_json" => InfoError::InvalidJson,
            "json_not_object" => InfoError::JsonNotObject,
            "request_timeout" => InfoError::RequestTimeout,
            "upgrade_required" => InfoError::UpgradeRequired,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::MissingScope => write!(f, "Server returned error missing_scope"),
            InfoError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
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
pub struct ListRequest {
    /// Set to `true` to exclude archived channels from the list
    pub exclude_archived: Option<bool>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    pub types: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
    pub limit: Option<u64>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
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
    pub priority: Option<u64>,
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
    pub channels: Vec<Vec<ListChannelsInner>>,
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
    MissingScope,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
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
            "missing_scope" => ListError::MissingScope,
            "not_authed" => ListError::NotAuthed,
            "invalid_auth" => ListError::InvalidAuth,
            "account_inactive" => ListError::AccountInactive,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_post_type" => ListError::InvalidPostType,
            "missing_post_type" => ListError::MissingPostType,
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
            ListError::MissingScope => write!(f, "Server returned error missing_scope"),
            ListError::NotAuthed => write!(f, "Server returned error not_authed"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::MissingPostType => write!(f, "Server returned error missing_post_type"),
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

#[derive(Clone, Default, Debug)]
pub struct RepliesRequest {
    /// Conversation ID to fetch thread from.
    pub channel: Option<String>,
    /// Unique identifier of a thread's parent message. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message.
    pub ts: Option<u64>,
    /// End of time range of messages to include in results.
    pub latest: Option<u64>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<u64>,
    /// Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    pub inclusive: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
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
    MissingScope,
    ChannelNotFound,
    ThreadNotFound,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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

impl<'a, E: Error> From<&'a str> for RepliesError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "missing_scope" => RepliesError::MissingScope,
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
            "team_added_to_org" => RepliesError::TeamAddedToOrg,
            "invalid_json" => RepliesError::InvalidJson,
            "json_not_object" => RepliesError::JsonNotObject,
            "request_timeout" => RepliesError::RequestTimeout,
            "upgrade_required" => RepliesError::UpgradeRequired,
            _ => RepliesError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RepliesError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RepliesError::MissingScope => write!(f, "Server returned error missing_scope"),
            RepliesError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            RepliesError::ThreadNotFound => write!(f, "Server returned error thread_not_found"),
            RepliesError::NotAuthed => write!(f, "Server returned error not_authed"),
            RepliesError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RepliesError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RepliesError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RepliesError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RepliesError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RepliesError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RepliesError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RepliesError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RepliesError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            RepliesError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RepliesError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RepliesError::RequestTimeout => write!(f, "Server returned error request_timeout"),
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
pub struct UnarchiveRequest {
    /// ID of conversation to unarchive
    pub channel: Option<String>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    NotArchived,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    TeamAddedToOrg,
    MissingCharset,
    SuperfluousCharset,
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
            "method_not_supported_for_channel_type" => {
                UnarchiveError::MethodNotSupportedForChannelType
            }
            "missing_scope" => UnarchiveError::MissingScope,
            "channel_not_found" => UnarchiveError::ChannelNotFound,
            "not_archived" => UnarchiveError::NotArchived,
            "not_authed" => UnarchiveError::NotAuthed,
            "invalid_auth" => UnarchiveError::InvalidAuth,
            "account_inactive" => UnarchiveError::AccountInactive,
            "user_is_bot" => UnarchiveError::UserIsBot,
            "user_is_restricted" => UnarchiveError::UserIsRestricted,
            "user_is_ultra_restricted" => UnarchiveError::UserIsUltraRestricted,
            "invalid_arg_name" => UnarchiveError::InvalidArgName,
            "invalid_array_arg" => UnarchiveError::InvalidArrayArg,
            "invalid_charset" => UnarchiveError::InvalidCharset,
            "invalid_form_data" => UnarchiveError::InvalidFormData,
            "invalid_post_type" => UnarchiveError::InvalidPostType,
            "missing_post_type" => UnarchiveError::MissingPostType,
            "invalid_json" => UnarchiveError::InvalidJson,
            "json_not_object" => UnarchiveError::JsonNotObject,
            "request_timeout" => UnarchiveError::RequestTimeout,
            "upgrade_required" => UnarchiveError::UpgradeRequired,
            "team_added_to_org" => UnarchiveError::TeamAddedToOrg,
            "missing_charset" => UnarchiveError::MissingCharset,
            "superfluous_charset" => UnarchiveError::SuperfluousCharset,
            _ => UnarchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnarchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnarchiveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            UnarchiveError::MissingScope => write!(f, "Server returned error missing_scope"),
            UnarchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            UnarchiveError::NotArchived => write!(f, "Server returned error not_archived"),
            UnarchiveError::NotAuthed => write!(f, "Server returned error not_authed"),
            UnarchiveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            UnarchiveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            UnarchiveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            UnarchiveError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            UnarchiveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            UnarchiveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            UnarchiveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            UnarchiveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            UnarchiveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            UnarchiveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            UnarchiveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            UnarchiveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            UnarchiveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            UnarchiveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            UnarchiveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            UnarchiveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            UnarchiveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            UnarchiveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
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

#[derive(Clone, Default, Debug)]
pub struct LeaveRequest {
    /// Conversation to leave
    pub channel: Option<String>,
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
    MethodNotSupportedForChannelType,
    LastMember,
    MissingScope,
    ChannelNotFound,
    IsArchived,
    CantLeaveGeneral,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    TeamAddedToOrg,
    MissingCharset,
    SuperfluousCharset,
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
            "method_not_supported_for_channel_type" => LeaveError::MethodNotSupportedForChannelType,
            "last_member" => LeaveError::LastMember,
            "missing_scope" => LeaveError::MissingScope,
            "channel_not_found" => LeaveError::ChannelNotFound,
            "is_archived" => LeaveError::IsArchived,
            "cant_leave_general" => LeaveError::CantLeaveGeneral,
            "not_authed" => LeaveError::NotAuthed,
            "invalid_auth" => LeaveError::InvalidAuth,
            "account_inactive" => LeaveError::AccountInactive,
            "user_is_bot" => LeaveError::UserIsBot,
            "user_is_restricted" => LeaveError::UserIsRestricted,
            "user_is_ultra_restricted" => LeaveError::UserIsUltraRestricted,
            "invalid_arg_name" => LeaveError::InvalidArgName,
            "invalid_array_arg" => LeaveError::InvalidArrayArg,
            "invalid_charset" => LeaveError::InvalidCharset,
            "invalid_form_data" => LeaveError::InvalidFormData,
            "invalid_post_type" => LeaveError::InvalidPostType,
            "missing_post_type" => LeaveError::MissingPostType,
            "invalid_json" => LeaveError::InvalidJson,
            "json_not_object" => LeaveError::JsonNotObject,
            "request_timeout" => LeaveError::RequestTimeout,
            "upgrade_required" => LeaveError::UpgradeRequired,
            "team_added_to_org" => LeaveError::TeamAddedToOrg,
            "missing_charset" => LeaveError::MissingCharset,
            "superfluous_charset" => LeaveError::SuperfluousCharset,
            _ => LeaveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for LeaveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LeaveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            LeaveError::LastMember => write!(f, "Server returned error last_member"),
            LeaveError::MissingScope => write!(f, "Server returned error missing_scope"),
            LeaveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            LeaveError::IsArchived => write!(f, "Server returned error is_archived"),
            LeaveError::CantLeaveGeneral => write!(f, "Server returned error cant_leave_general"),
            LeaveError::NotAuthed => write!(f, "Server returned error not_authed"),
            LeaveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            LeaveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            LeaveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            LeaveError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            LeaveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            LeaveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            LeaveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            LeaveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            LeaveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            LeaveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            LeaveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            LeaveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            LeaveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            LeaveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            LeaveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            LeaveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            LeaveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            LeaveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
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
pub struct SetPurposeRequest {
    /// Conversation to set the purpose of
    pub channel: Option<String>,
    /// A new, specialer purpose
    pub purpose: Option<String>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    NotInChannel,
    IsArchived,
    TooLong,
    UserIsRestricted,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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

impl<'a, E: Error> From<&'a str> for SetPurposeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "method_not_supported_for_channel_type" => {
                SetPurposeError::MethodNotSupportedForChannelType
            }
            "missing_scope" => SetPurposeError::MissingScope,
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
            "team_added_to_org" => SetPurposeError::TeamAddedToOrg,
            "invalid_json" => SetPurposeError::InvalidJson,
            "json_not_object" => SetPurposeError::JsonNotObject,
            "request_timeout" => SetPurposeError::RequestTimeout,
            "upgrade_required" => SetPurposeError::UpgradeRequired,
            _ => SetPurposeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPurposeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetPurposeError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            SetPurposeError::MissingScope => write!(f, "Server returned error missing_scope"),
            SetPurposeError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            SetPurposeError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            SetPurposeError::IsArchived => write!(f, "Server returned error is_archived"),
            SetPurposeError::TooLong => write!(f, "Server returned error too_long"),
            SetPurposeError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            SetPurposeError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetPurposeError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetPurposeError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetPurposeError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetPurposeError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            SetPurposeError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetPurposeError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            SetPurposeError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            SetPurposeError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            SetPurposeError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetPurposeError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetPurposeError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetPurposeError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetPurposeError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct HistoryRequest {
    /// Conversation ID to fetch history for.
    pub channel: Option<String>,
    /// End of time range of messages to include in results.
    pub latest: Option<u64>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<u64>,
    /// Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    pub inclusive: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
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
    pub bot_id: Option<Vec<String>>,
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
    pub channel_actions_ts: Vec<u64>,
    error: Option<String>,
    pub has_more: bool,
    pub messages: Vec<HistoryMessagesInner>,
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
    MissingScope,
    ChannelNotFound,
    InvalidTsLatest,
    InvalidTsOldest,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
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

impl<'a, E: Error> From<&'a str> for HistoryError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "missing_scope" => HistoryError::MissingScope,
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
            "invalid_json" => HistoryError::InvalidJson,
            "json_not_object" => HistoryError::JsonNotObject,
            "request_timeout" => HistoryError::RequestTimeout,
            "upgrade_required" => HistoryError::UpgradeRequired,
            _ => HistoryError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for HistoryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HistoryError::MissingScope => write!(f, "Server returned error missing_scope"),
            HistoryError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            HistoryError::InvalidTsLatest => write!(f, "Server returned error invalid_ts_latest"),
            HistoryError::InvalidTsOldest => write!(f, "Server returned error invalid_ts_oldest"),
            HistoryError::NotAuthed => write!(f, "Server returned error not_authed"),
            HistoryError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            HistoryError::AccountInactive => write!(f, "Server returned error account_inactive"),
            HistoryError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            HistoryError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            HistoryError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            HistoryError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            HistoryError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            HistoryError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            HistoryError::InvalidJson => write!(f, "Server returned error invalid_json"),
            HistoryError::JsonNotObject => write!(f, "Server returned error json_not_object"),
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
pub struct CreateRequest {
    /// Name of the public or private channel to create
    pub name: Option<String>,
    /// Create a private channel instead of a public one
    pub is_private: Option<bool>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    NameTaken,
    RestrictedAction,
    NoChannel,
    InvalidNameRequired,
    InvalidNamePunctuation,
    InvalidNameMaxlength,
    InvalidNameSpecials,
    InvalidName,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
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

impl<'a, E: Error> From<&'a str> for CreateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "method_not_supported_for_channel_type" => {
                CreateError::MethodNotSupportedForChannelType
            }
            "missing_scope" => CreateError::MissingScope,
            "name_taken" => CreateError::NameTaken,
            "restricted_action" => CreateError::RestrictedAction,
            "no_channel" => CreateError::NoChannel,
            "invalid_name_required" => CreateError::InvalidNameRequired,
            "invalid_name_punctuation" => CreateError::InvalidNamePunctuation,
            "invalid_name_maxlength" => CreateError::InvalidNameMaxlength,
            "invalid_name_specials" => CreateError::InvalidNameSpecials,
            "invalid_name" => CreateError::InvalidName,
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
            "team_added_to_org" => CreateError::TeamAddedToOrg,
            "invalid_json" => CreateError::InvalidJson,
            "json_not_object" => CreateError::JsonNotObject,
            "request_timeout" => CreateError::RequestTimeout,
            "upgrade_required" => CreateError::UpgradeRequired,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreateError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            CreateError::MissingScope => write!(f, "Server returned error missing_scope"),
            CreateError::NameTaken => write!(f, "Server returned error name_taken"),
            CreateError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            CreateError::NoChannel => write!(f, "Server returned error no_channel"),
            CreateError::InvalidNameRequired => {
                write!(f, "Server returned error invalid_name_required")
            }
            CreateError::InvalidNamePunctuation => {
                write!(f, "Server returned error invalid_name_punctuation")
            }
            CreateError::InvalidNameMaxlength => {
                write!(f, "Server returned error invalid_name_maxlength")
            }
            CreateError::InvalidNameSpecials => {
                write!(f, "Server returned error invalid_name_specials")
            }
            CreateError::InvalidName => write!(f, "Server returned error invalid_name"),
            CreateError::NotAuthed => write!(f, "Server returned error not_authed"),
            CreateError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            CreateError::AccountInactive => write!(f, "Server returned error account_inactive"),
            CreateError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            CreateError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            CreateError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            CreateError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            CreateError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            CreateError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            CreateError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            CreateError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            CreateError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            CreateError::InvalidJson => write!(f, "Server returned error invalid_json"),
            CreateError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            CreateError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            CreateError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct SetTopicRequest {
    /// Conversation to set the topic of
    pub channel: Option<String>,
    /// The new topic string. Does not support formatting or linkification.
    pub topic: Option<String>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    NotInChannel,
    IsArchived,
    TooLong,
    UserIsRestricted,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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

impl<'a, E: Error> From<&'a str> for SetTopicError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "method_not_supported_for_channel_type" => {
                SetTopicError::MethodNotSupportedForChannelType
            }
            "missing_scope" => SetTopicError::MissingScope,
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
            "team_added_to_org" => SetTopicError::TeamAddedToOrg,
            "invalid_json" => SetTopicError::InvalidJson,
            "json_not_object" => SetTopicError::JsonNotObject,
            "request_timeout" => SetTopicError::RequestTimeout,
            "upgrade_required" => SetTopicError::UpgradeRequired,
            _ => SetTopicError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetTopicError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetTopicError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            SetTopicError::MissingScope => write!(f, "Server returned error missing_scope"),
            SetTopicError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            SetTopicError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            SetTopicError::IsArchived => write!(f, "Server returned error is_archived"),
            SetTopicError::TooLong => write!(f, "Server returned error too_long"),
            SetTopicError::UserIsRestricted => {
                write!(f, "Server returned error user_is_restricted")
            }
            SetTopicError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetTopicError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetTopicError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetTopicError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetTopicError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetTopicError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetTopicError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetTopicError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetTopicError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetTopicError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetTopicError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetTopicError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetTopicError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetTopicError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct RenameRequest {
    /// ID of conversation to rename
    pub channel: Option<String>,
    /// New name for conversation.
    pub name: Option<String>,
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
    pub priority: Option<u64>,
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
    UserIsRestricted,
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    NotInChannel,
    NotAuthorized,
    InvalidName,
    NameTaken,
    InvalidNameRequired,
    InvalidNamePunctuation,
    InvalidNameMaxlength,
    InvalidNameSpecials,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
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

impl<'a, E: Error> From<&'a str> for RenameError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "user_is_restricted" => RenameError::UserIsRestricted,
            "method_not_supported_for_channel_type" => {
                RenameError::MethodNotSupportedForChannelType
            }
            "missing_scope" => RenameError::MissingScope,
            "channel_not_found" => RenameError::ChannelNotFound,
            "not_in_channel" => RenameError::NotInChannel,
            "not_authorized" => RenameError::NotAuthorized,
            "invalid_name" => RenameError::InvalidName,
            "name_taken" => RenameError::NameTaken,
            "invalid_name_required" => RenameError::InvalidNameRequired,
            "invalid_name_punctuation" => RenameError::InvalidNamePunctuation,
            "invalid_name_maxlength" => RenameError::InvalidNameMaxlength,
            "invalid_name_specials" => RenameError::InvalidNameSpecials,
            "not_authed" => RenameError::NotAuthed,
            "invalid_auth" => RenameError::InvalidAuth,
            "account_inactive" => RenameError::AccountInactive,
            "invalid_arg_name" => RenameError::InvalidArgName,
            "invalid_array_arg" => RenameError::InvalidArrayArg,
            "invalid_charset" => RenameError::InvalidCharset,
            "invalid_form_data" => RenameError::InvalidFormData,
            "invalid_post_type" => RenameError::InvalidPostType,
            "missing_post_type" => RenameError::MissingPostType,
            "invalid_json" => RenameError::InvalidJson,
            "json_not_object" => RenameError::JsonNotObject,
            "request_timeout" => RenameError::RequestTimeout,
            "upgrade_required" => RenameError::UpgradeRequired,
            _ => RenameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RenameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RenameError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            RenameError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            RenameError::MissingScope => write!(f, "Server returned error missing_scope"),
            RenameError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            RenameError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            RenameError::NotAuthorized => write!(f, "Server returned error not_authorized"),
            RenameError::InvalidName => write!(f, "Server returned error invalid_name"),
            RenameError::NameTaken => write!(f, "Server returned error name_taken"),
            RenameError::InvalidNameRequired => {
                write!(f, "Server returned error invalid_name_required")
            }
            RenameError::InvalidNamePunctuation => {
                write!(f, "Server returned error invalid_name_punctuation")
            }
            RenameError::InvalidNameMaxlength => {
                write!(f, "Server returned error invalid_name_maxlength")
            }
            RenameError::InvalidNameSpecials => {
                write!(f, "Server returned error invalid_name_specials")
            }
            RenameError::NotAuthed => write!(f, "Server returned error not_authed"),
            RenameError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            RenameError::AccountInactive => write!(f, "Server returned error account_inactive"),
            RenameError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            RenameError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            RenameError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            RenameError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            RenameError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            RenameError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            RenameError::InvalidJson => write!(f, "Server returned error invalid_json"),
            RenameError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            RenameError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            RenameError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct OpenRequest {
    /// Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead.
    pub channel: Option<String>,
    /// Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`.
    pub users: Option<String>,
    /// Boolean, indicates you want the full IM channel definition in the response.
    pub return_im: Option<bool>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    UserNotFound,
    UserNotVisible,
    UserDisabled,
    UsersListNotSupplied,
    NotEnoughUsers,
    TooManyUsers,
    InvalidUserCombination,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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
    ChannelNotFound,
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
            "method_not_supported_for_channel_type" => OpenError::MethodNotSupportedForChannelType,
            "user_not_found" => OpenError::UserNotFound,
            "user_not_visible" => OpenError::UserNotVisible,
            "user_disabled" => OpenError::UserDisabled,
            "users_list_not_supplied" => OpenError::UsersListNotSupplied,
            "not_enough_users" => OpenError::NotEnoughUsers,
            "too_many_users" => OpenError::TooManyUsers,
            "invalid_user_combination" => OpenError::InvalidUserCombination,
            "not_authed" => OpenError::NotAuthed,
            "invalid_auth" => OpenError::InvalidAuth,
            "account_inactive" => OpenError::AccountInactive,
            "invalid_arg_name" => OpenError::InvalidArgName,
            "invalid_array_arg" => OpenError::InvalidArrayArg,
            "invalid_charset" => OpenError::InvalidCharset,
            "invalid_form_data" => OpenError::InvalidFormData,
            "invalid_post_type" => OpenError::InvalidPostType,
            "missing_post_type" => OpenError::MissingPostType,
            "team_added_to_org" => OpenError::TeamAddedToOrg,
            "invalid_json" => OpenError::InvalidJson,
            "json_not_object" => OpenError::JsonNotObject,
            "request_timeout" => OpenError::RequestTimeout,
            "upgrade_required" => OpenError::UpgradeRequired,
            "channel_not_found" => OpenError::ChannelNotFound,
            _ => OpenError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for OpenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpenError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            OpenError::UserNotFound => write!(f, "Server returned error user_not_found"),
            OpenError::UserNotVisible => write!(f, "Server returned error user_not_visible"),
            OpenError::UserDisabled => write!(f, "Server returned error user_disabled"),
            OpenError::UsersListNotSupplied => {
                write!(f, "Server returned error users_list_not_supplied")
            }
            OpenError::NotEnoughUsers => write!(f, "Server returned error not_enough_users"),
            OpenError::TooManyUsers => write!(f, "Server returned error too_many_users"),
            OpenError::InvalidUserCombination => {
                write!(f, "Server returned error invalid_user_combination")
            }
            OpenError::NotAuthed => write!(f, "Server returned error not_authed"),
            OpenError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            OpenError::AccountInactive => write!(f, "Server returned error account_inactive"),
            OpenError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            OpenError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            OpenError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            OpenError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            OpenError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            OpenError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            OpenError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            OpenError::InvalidJson => write!(f, "Server returned error invalid_json"),
            OpenError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            OpenError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            OpenError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            OpenError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
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
pub struct ArchiveRequest {
    /// ID of conversation to archive
    pub channel: Option<String>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    NotSupported,
    ChannelNotFound,
    AlreadyArchived,
    CantArchiveGeneral,
    RestrictedAction,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    TeamAddedToOrg,
    MissingCharset,
    SuperfluousCharset,
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
            "method_not_supported_for_channel_type" => {
                ArchiveError::MethodNotSupportedForChannelType
            }
            "missing_scope" => ArchiveError::MissingScope,
            "not_supported" => ArchiveError::NotSupported,
            "channel_not_found" => ArchiveError::ChannelNotFound,
            "already_archived" => ArchiveError::AlreadyArchived,
            "cant_archive_general" => ArchiveError::CantArchiveGeneral,
            "restricted_action" => ArchiveError::RestrictedAction,
            "not_authed" => ArchiveError::NotAuthed,
            "invalid_auth" => ArchiveError::InvalidAuth,
            "account_inactive" => ArchiveError::AccountInactive,
            "user_is_bot" => ArchiveError::UserIsBot,
            "user_is_restricted" => ArchiveError::UserIsRestricted,
            "user_is_ultra_restricted" => ArchiveError::UserIsUltraRestricted,
            "invalid_arg_name" => ArchiveError::InvalidArgName,
            "invalid_array_arg" => ArchiveError::InvalidArrayArg,
            "invalid_charset" => ArchiveError::InvalidCharset,
            "invalid_form_data" => ArchiveError::InvalidFormData,
            "invalid_post_type" => ArchiveError::InvalidPostType,
            "missing_post_type" => ArchiveError::MissingPostType,
            "invalid_json" => ArchiveError::InvalidJson,
            "json_not_object" => ArchiveError::JsonNotObject,
            "request_timeout" => ArchiveError::RequestTimeout,
            "upgrade_required" => ArchiveError::UpgradeRequired,
            "team_added_to_org" => ArchiveError::TeamAddedToOrg,
            "missing_charset" => ArchiveError::MissingCharset,
            "superfluous_charset" => ArchiveError::SuperfluousCharset,
            _ => ArchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ArchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArchiveError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            ArchiveError::MissingScope => write!(f, "Server returned error missing_scope"),
            ArchiveError::NotSupported => write!(f, "Server returned error not_supported"),
            ArchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            ArchiveError::AlreadyArchived => write!(f, "Server returned error already_archived"),
            ArchiveError::CantArchiveGeneral => {
                write!(f, "Server returned error cant_archive_general")
            }
            ArchiveError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            ArchiveError::NotAuthed => write!(f, "Server returned error not_authed"),
            ArchiveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ArchiveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            ArchiveError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            ArchiveError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            ArchiveError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            ArchiveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ArchiveError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ArchiveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ArchiveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ArchiveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ArchiveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            ArchiveError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ArchiveError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ArchiveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            ArchiveError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            ArchiveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            ArchiveError::MissingCharset => write!(f, "Server returned error missing_charset"),
            ArchiveError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
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
pub struct MarkRequest {
    /// Channel or conversation to set the read cursor for.
    pub channel: Option<String>,
    /// Unique identifier of message you want marked as most recently seen in this conversation.
    pub ts: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    InvalidTimestamp,
    NotInChannel,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    NotAllowedTokenType,
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
            "method_not_supported_for_channel_type" => MarkError::MethodNotSupportedForChannelType,
            "missing_scope" => MarkError::MissingScope,
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
            "invalid_json" => MarkError::InvalidJson,
            "json_not_object" => MarkError::JsonNotObject,
            "request_timeout" => MarkError::RequestTimeout,
            "upgrade_required" => MarkError::UpgradeRequired,
            "not_allowed_token_type" => MarkError::NotAllowedTokenType,
            _ => MarkError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MarkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MarkError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            MarkError::MissingScope => write!(f, "Server returned error missing_scope"),
            MarkError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MarkError::InvalidTimestamp => write!(f, "Server returned error invalid_timestamp"),
            MarkError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            MarkError::NotAuthed => write!(f, "Server returned error not_authed"),
            MarkError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MarkError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MarkError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MarkError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MarkError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MarkError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MarkError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MarkError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MarkError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MarkError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MarkError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            MarkError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            MarkError::NotAllowedTokenType => {
                write!(f, "Server returned error not_allowed_token_type")
            }
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
pub struct InviteRequest {
    /// The ID of the public or private channel to invite user(s) to.
    pub channel: Option<String>,
    /// A comma separated list of user IDs. Up to 1000 users may be listed.
    pub users: Option<String>,
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
    pub priority: Option<u64>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    UserNotFound,
    NoUser,
    CantInviteSelf,
    NotInChannel,
    AlreadyInChannel,
    IsArchived,
    CantInvite,
    TooManyUsers,
    UraMaxChannels,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    UserIsUltraRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    InvalidJson,
    JsonNotObject,
    RequestTimeout,
    UpgradeRequired,
    TeamAddedToOrg,
    MissingCharset,
    SuperfluousCharset,
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
            "method_not_supported_for_channel_type" => {
                InviteError::MethodNotSupportedForChannelType
            }
            "missing_scope" => InviteError::MissingScope,
            "channel_not_found" => InviteError::ChannelNotFound,
            "user_not_found" => InviteError::UserNotFound,
            "no_user" => InviteError::NoUser,
            "cant_invite_self" => InviteError::CantInviteSelf,
            "not_in_channel" => InviteError::NotInChannel,
            "already_in_channel" => InviteError::AlreadyInChannel,
            "is_archived" => InviteError::IsArchived,
            "cant_invite" => InviteError::CantInvite,
            "too_many_users" => InviteError::TooManyUsers,
            "ura_max_channels" => InviteError::UraMaxChannels,
            "not_authed" => InviteError::NotAuthed,
            "invalid_auth" => InviteError::InvalidAuth,
            "account_inactive" => InviteError::AccountInactive,
            "user_is_bot" => InviteError::UserIsBot,
            "user_is_restricted" => InviteError::UserIsRestricted,
            "user_is_ultra_restricted" => InviteError::UserIsUltraRestricted,
            "invalid_arg_name" => InviteError::InvalidArgName,
            "invalid_array_arg" => InviteError::InvalidArrayArg,
            "invalid_charset" => InviteError::InvalidCharset,
            "invalid_form_data" => InviteError::InvalidFormData,
            "invalid_post_type" => InviteError::InvalidPostType,
            "missing_post_type" => InviteError::MissingPostType,
            "invalid_json" => InviteError::InvalidJson,
            "json_not_object" => InviteError::JsonNotObject,
            "request_timeout" => InviteError::RequestTimeout,
            "upgrade_required" => InviteError::UpgradeRequired,
            "team_added_to_org" => InviteError::TeamAddedToOrg,
            "missing_charset" => InviteError::MissingCharset,
            "superfluous_charset" => InviteError::SuperfluousCharset,
            _ => InviteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InviteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InviteError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            InviteError::MissingScope => write!(f, "Server returned error missing_scope"),
            InviteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            InviteError::UserNotFound => write!(f, "Server returned error user_not_found"),
            InviteError::NoUser => write!(f, "Server returned error no_user"),
            InviteError::CantInviteSelf => write!(f, "Server returned error cant_invite_self"),
            InviteError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            InviteError::AlreadyInChannel => write!(f, "Server returned error already_in_channel"),
            InviteError::IsArchived => write!(f, "Server returned error is_archived"),
            InviteError::CantInvite => write!(f, "Server returned error cant_invite"),
            InviteError::TooManyUsers => write!(f, "Server returned error too_many_users"),
            InviteError::UraMaxChannels => write!(f, "Server returned error ura_max_channels"),
            InviteError::NotAuthed => write!(f, "Server returned error not_authed"),
            InviteError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InviteError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InviteError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            InviteError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            InviteError::UserIsUltraRestricted => {
                write!(f, "Server returned error user_is_ultra_restricted")
            }
            InviteError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InviteError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InviteError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InviteError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InviteError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InviteError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InviteError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InviteError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InviteError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InviteError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InviteError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InviteError::MissingCharset => write!(f, "Server returned error missing_charset"),
            InviteError::SuperfluousCharset => {
                write!(f, "Server returned error superfluous_charset")
            }
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
pub struct KickRequest {
    /// ID of conversation to remove user from.
    pub channel: Option<String>,
    /// User ID to be removed.
    pub user: Option<String>,
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
    MethodNotSupportedForChannelType,
    MissingScope,
    ChannelNotFound,
    UserNotFound,
    CantKickSelf,
    NotInChannel,
    CantKickFromGeneral,
    RestrictedAction,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    UserIsBot,
    UserIsRestricted,
    InvalidArgName,
    InvalidArrayArg,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
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

impl<'a, E: Error> From<&'a str> for KickError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "method_not_supported_for_channel_type" => KickError::MethodNotSupportedForChannelType,
            "missing_scope" => KickError::MissingScope,
            "channel_not_found" => KickError::ChannelNotFound,
            "user_not_found" => KickError::UserNotFound,
            "cant_kick_self" => KickError::CantKickSelf,
            "not_in_channel" => KickError::NotInChannel,
            "cant_kick_from_general" => KickError::CantKickFromGeneral,
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
            "invalid_json" => KickError::InvalidJson,
            "json_not_object" => KickError::JsonNotObject,
            "request_timeout" => KickError::RequestTimeout,
            "upgrade_required" => KickError::UpgradeRequired,
            _ => KickError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for KickError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KickError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            KickError::MissingScope => write!(f, "Server returned error missing_scope"),
            KickError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            KickError::UserNotFound => write!(f, "Server returned error user_not_found"),
            KickError::CantKickSelf => write!(f, "Server returned error cant_kick_self"),
            KickError::NotInChannel => write!(f, "Server returned error not_in_channel"),
            KickError::CantKickFromGeneral => {
                write!(f, "Server returned error cant_kick_from_general")
            }
            KickError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            KickError::NotAuthed => write!(f, "Server returned error not_authed"),
            KickError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            KickError::AccountInactive => write!(f, "Server returned error account_inactive"),
            KickError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            KickError::UserIsRestricted => write!(f, "Server returned error user_is_restricted"),
            KickError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            KickError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            KickError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            KickError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            KickError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            KickError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            KickError::InvalidJson => write!(f, "Server returned error invalid_json"),
            KickError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            KickError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            KickError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
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
pub struct MembersRequest {
    /// ID of the conversation to retrieve members for
    pub channel: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
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
    ChannelNotFound,
    InvalidLimit,
    InvalidCursor,
    FetchMembersFailed,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
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

impl<'a, E: Error> From<&'a str> for MembersError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "channel_not_found" => MembersError::ChannelNotFound,
            "invalid_limit" => MembersError::InvalidLimit,
            "invalid_cursor" => MembersError::InvalidCursor,
            "fetch_members_failed" => MembersError::FetchMembersFailed,
            "not_authed" => MembersError::NotAuthed,
            "invalid_auth" => MembersError::InvalidAuth,
            "account_inactive" => MembersError::AccountInactive,
            "invalid_arg_name" => MembersError::InvalidArgName,
            "invalid_array_arg" => MembersError::InvalidArrayArg,
            "invalid_charset" => MembersError::InvalidCharset,
            "invalid_form_data" => MembersError::InvalidFormData,
            "invalid_post_type" => MembersError::InvalidPostType,
            "missing_post_type" => MembersError::MissingPostType,
            "team_added_to_org" => MembersError::TeamAddedToOrg,
            "invalid_json" => MembersError::InvalidJson,
            "json_not_object" => MembersError::JsonNotObject,
            "request_timeout" => MembersError::RequestTimeout,
            "upgrade_required" => MembersError::UpgradeRequired,
            _ => MembersError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for MembersError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MembersError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            MembersError::InvalidLimit => write!(f, "Server returned error invalid_limit"),
            MembersError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            MembersError::FetchMembersFailed => {
                write!(f, "Server returned error fetch_members_failed")
            }
            MembersError::NotAuthed => write!(f, "Server returned error not_authed"),
            MembersError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            MembersError::AccountInactive => write!(f, "Server returned error account_inactive"),
            MembersError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            MembersError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            MembersError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            MembersError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            MembersError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            MembersError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            MembersError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            MembersError::InvalidJson => write!(f, "Server returned error invalid_json"),
            MembersError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            MembersError::RequestTimeout => write!(f, "Server returned error request_timeout"),
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
