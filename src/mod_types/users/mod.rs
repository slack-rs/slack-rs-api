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

pub mod profile_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct ConversationsRequest<'a> {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// Set to `true` to exclude archived channels from the list
    pub exclude_archived: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
    pub limit: Option<u64>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    pub types: Option<Cow<'a, str>>,
    /// Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsDisplayCountsInner {
    pub display_counts: u64,
    pub guest_counts: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: ConversationsIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<ConversationsReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsFileInner {
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
    pub reactions: Option<Vec<ConversationsReactions1Inner>>,
    pub shares: Option<ConversationsSharesInner>,
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
pub struct ConversationsReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsFilesInner {
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
    pub reactions: Option<Vec<ConversationsReactions2Inner>>,
    pub shares: Option<ConversationsShares1Inner>,
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
pub struct ConversationsIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsUserProfileInner {
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
pub struct ConversationsLatestInner {
    pub attachments: Option<Vec<ConversationsAttachmentsInner>>,
    pub blocks: Option<Vec<ConversationsBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<ConversationsBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<ConversationsCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<ConversationsFileInner>,
    pub files: Option<Vec<ConversationsFilesInner>>,
    pub icons: Option<ConversationsIcons1Inner>,
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
    pub reactions: Option<Vec<ConversationsReactions3Inner>>,
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
    pub user_profile: Option<ConversationsUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsCurrentInner {
    pub date_started: u64,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsExternalOrgMigrationsInner {
    pub current: Vec<ConversationsCurrentInner>,
    pub date_updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsIconInner {
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
pub struct ConversationsPrimaryOwnerInner {
    pub email: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsSsoProviderInner {
    pub label: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsTeamInner {
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
    pub external_org_migrations: Option<ConversationsExternalOrgMigrationsInner>,
    pub has_compliance_export: Option<bool>,
    pub icon: ConversationsIconInner,
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
    pub primary_owner: Option<ConversationsPrimaryOwnerInner>,
    pub sso_provider: Option<ConversationsSsoProviderInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsShares2Inner {
    pub accepted_user: Option<String>,
    pub is_active: bool,
    pub team: ConversationsTeamInner,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsChannelsInner {
    pub accepted_user: Option<String>,
    pub connected_team_ids: Option<Vec<String>>,
    pub conversation_host_id: Option<String>,
    pub created: u64,
    pub creator: String,
    pub display_counts: Option<ConversationsDisplayCountsInner>,
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
    pub latest: Option<Vec<ConversationsLatestInner>>,
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
    pub purpose: ConversationsPurposeInner,
    pub shared_team_ids: Option<Vec<String>>,
    pub shares: Option<Vec<ConversationsShares2Inner>>,
    pub timezone_count: Option<u64>,
    pub topic: ConversationsTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
    pub use_case: Option<String>,
    pub user: Option<String>,
    pub version: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConversationsResponse {
    pub callstack: Option<String>,
    pub channels: Vec<Vec<ConversationsChannelsInner>>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub response_metadata: Option<ConversationsResponseMetadataInner>,
}

impl<E: Error> Into<Result<ConversationsResponse, ConversationsError<E>>>
    for ConversationsResponse
{
    fn into(self) -> Result<ConversationsResponse, ConversationsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum ConversationsError<E: Error> {
    AccountInactive,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidCursor,
    InvalidFormData,
    InvalidJson,
    InvalidLimit,
    InvalidPostType,
    InvalidTypes,
    JsonNotObject,
    MethodNotSupportedForChannelType,
    MissingPostType,
    MissingScope,
    NoPermission,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ConversationsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => ConversationsError::AccountInactive,
            "fatal_error" => ConversationsError::FatalError,
            "invalid_arg_name" => ConversationsError::InvalidArgName,
            "invalid_array_arg" => ConversationsError::InvalidArrayArg,
            "invalid_auth" => ConversationsError::InvalidAuth,
            "invalid_charset" => ConversationsError::InvalidCharset,
            "invalid_cursor" => ConversationsError::InvalidCursor,
            "invalid_form_data" => ConversationsError::InvalidFormData,
            "invalid_json" => ConversationsError::InvalidJson,
            "invalid_limit" => ConversationsError::InvalidLimit,
            "invalid_post_type" => ConversationsError::InvalidPostType,
            "invalid_types" => ConversationsError::InvalidTypes,
            "json_not_object" => ConversationsError::JsonNotObject,
            "method_not_supported_for_channel_type" => {
                ConversationsError::MethodNotSupportedForChannelType
            }
            "missing_post_type" => ConversationsError::MissingPostType,
            "missing_scope" => ConversationsError::MissingScope,
            "no_permission" => ConversationsError::NoPermission,
            "not_authed" => ConversationsError::NotAuthed,
            "request_timeout" => ConversationsError::RequestTimeout,
            "team_added_to_org" => ConversationsError::TeamAddedToOrg,
            "token_revoked" => ConversationsError::TokenRevoked,
            "upgrade_required" => ConversationsError::UpgradeRequired,
            _ => ConversationsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ConversationsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConversationsError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            ConversationsError::FatalError => write!(f, "Server returned error fatal_error"),
            ConversationsError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            ConversationsError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            ConversationsError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ConversationsError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            ConversationsError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            ConversationsError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            ConversationsError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ConversationsError::InvalidLimit => write!(f, "Server returned error invalid_limit"),
            ConversationsError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            ConversationsError::InvalidTypes => write!(f, "Server returned error invalid_types"),
            ConversationsError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ConversationsError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            ConversationsError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            ConversationsError::MissingScope => write!(f, "Server returned error missing_scope"),
            ConversationsError::NoPermission => write!(f, "Server returned error no_permission"),
            ConversationsError::NotAuthed => write!(f, "Server returned error not_authed"),
            ConversationsError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            ConversationsError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            ConversationsError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            ConversationsError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            ConversationsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ConversationsError::Unknown(ref s) => write!(f, "{}", s),
            ConversationsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ConversationsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ConversationsError::MalformedResponse(_, ref e) => Some(e),
            ConversationsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct DeletePhotoRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePhotoResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DeletePhotoResponse, DeletePhotoError<E>>> for DeletePhotoResponse {
    fn into(self) -> Result<DeletePhotoResponse, DeletePhotoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DeletePhotoError<E: Error> {
    AccountInactive,
    FatalError,
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
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DeletePhotoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => DeletePhotoError::AccountInactive,
            "fatal_error" => DeletePhotoError::FatalError,
            "invalid_arg_name" => DeletePhotoError::InvalidArgName,
            "invalid_array_arg" => DeletePhotoError::InvalidArrayArg,
            "invalid_auth" => DeletePhotoError::InvalidAuth,
            "invalid_charset" => DeletePhotoError::InvalidCharset,
            "invalid_form_data" => DeletePhotoError::InvalidFormData,
            "invalid_json" => DeletePhotoError::InvalidJson,
            "invalid_post_type" => DeletePhotoError::InvalidPostType,
            "json_not_object" => DeletePhotoError::JsonNotObject,
            "missing_post_type" => DeletePhotoError::MissingPostType,
            "no_permission" => DeletePhotoError::NoPermission,
            "not_authed" => DeletePhotoError::NotAuthed,
            "org_login_required" => DeletePhotoError::OrgLoginRequired,
            "request_timeout" => DeletePhotoError::RequestTimeout,
            "team_added_to_org" => DeletePhotoError::TeamAddedToOrg,
            "token_revoked" => DeletePhotoError::TokenRevoked,
            "upgrade_required" => DeletePhotoError::UpgradeRequired,
            "user_is_bot" => DeletePhotoError::UserIsBot,
            _ => DeletePhotoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeletePhotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeletePhotoError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            DeletePhotoError::FatalError => write!(f, "Server returned error fatal_error"),
            DeletePhotoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            DeletePhotoError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            DeletePhotoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            DeletePhotoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            DeletePhotoError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            DeletePhotoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            DeletePhotoError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            DeletePhotoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            DeletePhotoError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            DeletePhotoError::NoPermission => write!(f, "Server returned error no_permission"),
            DeletePhotoError::NotAuthed => write!(f, "Server returned error not_authed"),
            DeletePhotoError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            DeletePhotoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            DeletePhotoError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            DeletePhotoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            DeletePhotoError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            DeletePhotoError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            DeletePhotoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DeletePhotoError::Unknown(ref s) => write!(f, "{}", s),
            DeletePhotoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DeletePhotoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeletePhotoError::MalformedResponse(_, ref e) => Some(e),
            DeletePhotoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetPresenceRequest<'a> {
    /// User to get presence info on. Defaults to the authed user.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetPresenceResponse {
    pub auto_away: Option<bool>,
    pub connection_count: Option<u64>,
    error: Option<String>,
    pub last_activity: Option<u64>,
    pub manual_away: Option<bool>,
    #[serde(default)]
    ok: bool,
    pub online: Option<bool>,
    pub presence: String,
}

impl<E: Error> Into<Result<GetPresenceResponse, GetPresenceError<E>>> for GetPresenceResponse {
    fn into(self) -> Result<GetPresenceResponse, GetPresenceError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetPresenceError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for GetPresenceError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => GetPresenceError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetPresenceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetPresenceError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetPresenceError::Unknown(ref s) => write!(f, "{}", s),
            GetPresenceError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetPresenceError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetPresenceError::MalformedResponse(_, ref e) => Some(e),
            GetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct IdentityRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityTeamInner {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityUserInner {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityResultInner {
    #[serde(default)]
    ok: bool,
    pub team: IdentityTeamInner,
    pub user: IdentityUserInner,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityResponse {
    pub result: IdentityResultInner,
}

impl<E: Error> Into<Result<IdentityResponse, IdentityError<E>>> for IdentityResponse {
    fn into(self) -> Result<IdentityResponse, IdentityError<E>> {
        if self.result.ok {
            Ok(self)
        } else {
            Err(IdentityError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum IdentityError<E: Error> {
    AccountInactive,
    FatalError,
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
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for IdentityError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => IdentityError::AccountInactive,
            "fatal_error" => IdentityError::FatalError,
            "invalid_arg_name" => IdentityError::InvalidArgName,
            "invalid_array_arg" => IdentityError::InvalidArrayArg,
            "invalid_auth" => IdentityError::InvalidAuth,
            "invalid_charset" => IdentityError::InvalidCharset,
            "invalid_form_data" => IdentityError::InvalidFormData,
            "invalid_json" => IdentityError::InvalidJson,
            "invalid_post_type" => IdentityError::InvalidPostType,
            "json_not_object" => IdentityError::JsonNotObject,
            "missing_post_type" => IdentityError::MissingPostType,
            "no_permission" => IdentityError::NoPermission,
            "not_authed" => IdentityError::NotAuthed,
            "org_login_required" => IdentityError::OrgLoginRequired,
            "request_timeout" => IdentityError::RequestTimeout,
            "team_added_to_org" => IdentityError::TeamAddedToOrg,
            "token_revoked" => IdentityError::TokenRevoked,
            "upgrade_required" => IdentityError::UpgradeRequired,
            "user_is_bot" => IdentityError::UserIsBot,
            _ => IdentityError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for IdentityError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IdentityError::AccountInactive => write!(f, "Server returned error account_inactive"),
            IdentityError::FatalError => write!(f, "Server returned error fatal_error"),
            IdentityError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            IdentityError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            IdentityError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            IdentityError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            IdentityError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            IdentityError::InvalidJson => write!(f, "Server returned error invalid_json"),
            IdentityError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            IdentityError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            IdentityError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            IdentityError::NoPermission => write!(f, "Server returned error no_permission"),
            IdentityError::NotAuthed => write!(f, "Server returned error not_authed"),
            IdentityError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            IdentityError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            IdentityError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            IdentityError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            IdentityError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            IdentityError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            IdentityError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            IdentityError::Unknown(ref s) => write!(f, "{}", s),
            IdentityError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for IdentityError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            IdentityError::MalformedResponse(_, ref e) => Some(e),
            IdentityError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Set this to `true` to receive the locale for this user. Defaults to `false`
    pub include_locale: Option<bool>,
    /// User to get info on
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoEnterpriseUserInner {
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub id: String,
    pub is_admin: bool,
    pub is_owner: bool,
    pub teams: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoProfileInner {
    pub always_active: Option<bool>,
    pub api_app_id: Option<String>,
    pub avatar_hash: String,
    pub bot_id: Option<String>,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub fields: Vec<serde_json::Value>,
    pub first_name: Option<String>,
    pub guest_expiration_ts: Option<u64>,
    pub guest_invited_by: Option<String>,
    pub image_1024: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub is_app_user: Option<bool>,
    pub is_custom_image: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub last_avatar_image_hash: Option<String>,
    pub last_name: Option<String>,
    pub memberships_count: Option<u64>,
    pub name: Option<String>,
    pub phone: String,
    pub pronouns: Option<String>,
    pub real_name: String,
    pub real_name_normalized: String,
    pub skype: String,
    pub status_default_emoji: Option<String>,
    pub status_default_text: Option<String>,
    pub status_default_text_canonical: Option<String>,
    pub status_emoji: String,
    pub status_expiration: Option<u64>,
    pub status_text: String,
    pub status_text_canonical: Option<String>,
    pub team: Option<String>,
    pub title: String,
    pub updated: Option<u64>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoOptionsInner {
    pub is_custom: Option<bool>,
    pub is_multiple_entry: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_scim: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoFieldsInner {
    pub field_name: Option<String>,
    pub hint: String,
    pub id: String,
    pub is_hidden: Option<bool>,
    pub label: String,
    pub options: Option<Vec<InfoOptionsInner>>,
    pub ordering: f64,
    pub possible_values: Option<Vec<String>>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoTeamProfileInner {
    pub fields: Vec<InfoFieldsInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoUserInner {
    pub color: Option<String>,
    pub deleted: Option<bool>,
    pub enterprise_user: Option<InfoEnterpriseUserInner>,
    pub has_2fa: Option<bool>,
    pub id: String,
    pub is_admin: Option<bool>,
    pub is_app_user: bool,
    pub is_bot: bool,
    pub is_external: Option<bool>,
    pub is_forgotten: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub locale: Option<String>,
    pub name: String,
    pub presence: Option<String>,
    pub profile: InfoProfileInner,
    pub real_name: Option<String>,
    pub team: Option<String>,
    pub team_id: Option<String>,
    pub team_profile: Option<InfoTeamProfileInner>,
    pub two_factor_type: Option<String>,
    pub tz: Option<Vec<String>>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<f64>,
    pub updated: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub user: Vec<InfoUserInner>,
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
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    MissingPostType,
    NotAuthed,
    RequestTimeout,
    TeamAddedToOrg,
    UpgradeRequired,
    UserNotFound,
    UserNotVisible,
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
            "invalid_arg_name" => InfoError::InvalidArgName,
            "invalid_array_arg" => InfoError::InvalidArrayArg,
            "invalid_auth" => InfoError::InvalidAuth,
            "invalid_charset" => InfoError::InvalidCharset,
            "invalid_form_data" => InfoError::InvalidFormData,
            "invalid_json" => InfoError::InvalidJson,
            "invalid_post_type" => InfoError::InvalidPostType,
            "json_not_object" => InfoError::JsonNotObject,
            "missing_post_type" => InfoError::MissingPostType,
            "not_authed" => InfoError::NotAuthed,
            "request_timeout" => InfoError::RequestTimeout,
            "team_added_to_org" => InfoError::TeamAddedToOrg,
            "upgrade_required" => InfoError::UpgradeRequired,
            "user_not_found" => InfoError::UserNotFound,
            "user_not_visible" => InfoError::UserNotVisible,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            InfoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            InfoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            InfoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            InfoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            InfoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            InfoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            InfoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            InfoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            InfoError::NotAuthed => write!(f, "Server returned error not_authed"),
            InfoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            InfoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            InfoError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            InfoError::UserNotFound => write!(f, "Server returned error user_not_found"),
            InfoError::UserNotVisible => write!(f, "Server returned error user_not_visible"),
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
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<Cow<'a, str>>,
    /// Set this to `true` to receive the locale for users. Defaults to `false`
    pub include_locale: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. Providing no `limit` value will result in Slack attempting to deliver you the entire result set. If the collection is too large you may experience `limit_required` or HTTP 500 errors.
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListEnterpriseUserInner {
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub id: String,
    pub is_admin: bool,
    pub is_owner: bool,
    pub teams: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListProfileInner {
    pub always_active: Option<bool>,
    pub api_app_id: Option<String>,
    pub avatar_hash: String,
    pub bot_id: Option<String>,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub fields: Vec<serde_json::Value>,
    pub first_name: Option<String>,
    pub guest_expiration_ts: Option<u64>,
    pub guest_invited_by: Option<String>,
    pub image_1024: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub is_app_user: Option<bool>,
    pub is_custom_image: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub last_avatar_image_hash: Option<String>,
    pub last_name: Option<String>,
    pub memberships_count: Option<u64>,
    pub name: Option<String>,
    pub phone: String,
    pub pronouns: Option<String>,
    pub real_name: String,
    pub real_name_normalized: String,
    pub skype: String,
    pub status_default_emoji: Option<String>,
    pub status_default_text: Option<String>,
    pub status_default_text_canonical: Option<String>,
    pub status_emoji: String,
    pub status_expiration: Option<u64>,
    pub status_text: String,
    pub status_text_canonical: Option<String>,
    pub team: Option<String>,
    pub title: String,
    pub updated: Option<u64>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListOptionsInner {
    pub is_custom: Option<bool>,
    pub is_multiple_entry: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_scim: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListFieldsInner {
    pub field_name: Option<String>,
    pub hint: String,
    pub id: String,
    pub is_hidden: Option<bool>,
    pub label: String,
    pub options: Option<Vec<ListOptionsInner>>,
    pub ordering: f64,
    pub possible_values: Option<Vec<String>>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListTeamProfileInner {
    pub fields: Vec<ListFieldsInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListMembersInner {
    pub color: Option<String>,
    pub deleted: Option<bool>,
    pub enterprise_user: Option<ListEnterpriseUserInner>,
    pub has_2fa: Option<bool>,
    pub id: String,
    pub is_admin: Option<bool>,
    pub is_app_user: bool,
    pub is_bot: bool,
    pub is_external: Option<bool>,
    pub is_forgotten: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub locale: Option<String>,
    pub name: String,
    pub presence: Option<String>,
    pub profile: ListProfileInner,
    pub real_name: Option<String>,
    pub team: Option<String>,
    pub team_id: Option<String>,
    pub team_profile: Option<ListTeamProfileInner>,
    pub two_factor_type: Option<String>,
    pub tz: Option<Vec<String>>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<f64>,
    pub updated: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    pub cache_ts: u64,
    pub callstack: Option<String>,
    error: Option<String>,
    pub members: Vec<Vec<ListMembersInner>>,
    #[serde(default)]
    ok: bool,
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
    AccountInactive,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidCursor,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    JsonNotObject,
    LimitRequired,
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
            "fatal_error" => ListError::FatalError,
            "invalid_arg_name" => ListError::InvalidArgName,
            "invalid_array_arg" => ListError::InvalidArrayArg,
            "invalid_auth" => ListError::InvalidAuth,
            "invalid_charset" => ListError::InvalidCharset,
            "invalid_cursor" => ListError::InvalidCursor,
            "invalid_form_data" => ListError::InvalidFormData,
            "invalid_json" => ListError::InvalidJson,
            "invalid_post_type" => ListError::InvalidPostType,
            "json_not_object" => ListError::JsonNotObject,
            "limit_required" => ListError::LimitRequired,
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
            ListError::FatalError => write!(f, "Server returned error fatal_error"),
            ListError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            ListError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            ListError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            ListError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            ListError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            ListError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            ListError::InvalidJson => write!(f, "Server returned error invalid_json"),
            ListError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            ListError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            ListError::LimitRequired => write!(f, "Server returned error limit_required"),
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
pub struct LookupByEmailRequest<'a> {
    /// An email address belonging to a user in the workspace
    pub email: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailEnterpriseUserInner {
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub id: String,
    pub is_admin: bool,
    pub is_owner: bool,
    pub teams: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailProfileInner {
    pub always_active: Option<bool>,
    pub api_app_id: Option<String>,
    pub avatar_hash: String,
    pub bot_id: Option<String>,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub fields: Vec<serde_json::Value>,
    pub first_name: Option<String>,
    pub guest_expiration_ts: Option<u64>,
    pub guest_invited_by: Option<String>,
    pub image_1024: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub is_app_user: Option<bool>,
    pub is_custom_image: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub last_avatar_image_hash: Option<String>,
    pub last_name: Option<String>,
    pub memberships_count: Option<u64>,
    pub name: Option<String>,
    pub phone: String,
    pub pronouns: Option<String>,
    pub real_name: String,
    pub real_name_normalized: String,
    pub skype: String,
    pub status_default_emoji: Option<String>,
    pub status_default_text: Option<String>,
    pub status_default_text_canonical: Option<String>,
    pub status_emoji: String,
    pub status_expiration: Option<u64>,
    pub status_text: String,
    pub status_text_canonical: Option<String>,
    pub team: Option<String>,
    pub title: String,
    pub updated: Option<u64>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailOptionsInner {
    pub is_custom: Option<bool>,
    pub is_multiple_entry: Option<bool>,
    pub is_protected: Option<bool>,
    pub is_scim: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailFieldsInner {
    pub field_name: Option<String>,
    pub hint: String,
    pub id: String,
    pub is_hidden: Option<bool>,
    pub label: String,
    pub options: Option<Vec<LookupByEmailOptionsInner>>,
    pub ordering: f64,
    pub possible_values: Option<Vec<String>>,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailTeamProfileInner {
    pub fields: Vec<LookupByEmailFieldsInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailUserInner {
    pub color: Option<String>,
    pub deleted: Option<bool>,
    pub enterprise_user: Option<LookupByEmailEnterpriseUserInner>,
    pub has_2fa: Option<bool>,
    pub id: String,
    pub is_admin: Option<bool>,
    pub is_app_user: bool,
    pub is_bot: bool,
    pub is_external: Option<bool>,
    pub is_forgotten: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub locale: Option<String>,
    pub name: String,
    pub presence: Option<String>,
    pub profile: LookupByEmailProfileInner,
    pub real_name: Option<String>,
    pub team: Option<String>,
    pub team_id: Option<String>,
    pub team_profile: Option<LookupByEmailTeamProfileInner>,
    pub two_factor_type: Option<String>,
    pub tz: Option<Vec<String>>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<f64>,
    pub updated: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LookupByEmailResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub user: Vec<LookupByEmailUserInner>,
}

impl<E: Error> Into<Result<LookupByEmailResponse, LookupByEmailError<E>>>
    for LookupByEmailResponse
{
    fn into(self) -> Result<LookupByEmailResponse, LookupByEmailError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum LookupByEmailError<E: Error> {
    AccountInactive,
    EnterpriseIsRestricted,
    FatalError,
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
    UsersNotFound,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for LookupByEmailError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => LookupByEmailError::AccountInactive,
            "enterprise_is_restricted" => LookupByEmailError::EnterpriseIsRestricted,
            "fatal_error" => LookupByEmailError::FatalError,
            "invalid_arg_name" => LookupByEmailError::InvalidArgName,
            "invalid_array_arg" => LookupByEmailError::InvalidArrayArg,
            "invalid_auth" => LookupByEmailError::InvalidAuth,
            "invalid_charset" => LookupByEmailError::InvalidCharset,
            "invalid_form_data" => LookupByEmailError::InvalidFormData,
            "invalid_json" => LookupByEmailError::InvalidJson,
            "invalid_post_type" => LookupByEmailError::InvalidPostType,
            "json_not_object" => LookupByEmailError::JsonNotObject,
            "missing_post_type" => LookupByEmailError::MissingPostType,
            "no_permission" => LookupByEmailError::NoPermission,
            "not_authed" => LookupByEmailError::NotAuthed,
            "request_timeout" => LookupByEmailError::RequestTimeout,
            "team_added_to_org" => LookupByEmailError::TeamAddedToOrg,
            "upgrade_required" => LookupByEmailError::UpgradeRequired,
            "users_not_found" => LookupByEmailError::UsersNotFound,
            _ => LookupByEmailError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for LookupByEmailError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LookupByEmailError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            LookupByEmailError::EnterpriseIsRestricted => {
                write!(f, "Server returned error enterprise_is_restricted")
            }
            LookupByEmailError::FatalError => write!(f, "Server returned error fatal_error"),
            LookupByEmailError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            LookupByEmailError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            LookupByEmailError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            LookupByEmailError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            LookupByEmailError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            LookupByEmailError::InvalidJson => write!(f, "Server returned error invalid_json"),
            LookupByEmailError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            LookupByEmailError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            LookupByEmailError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            LookupByEmailError::NoPermission => write!(f, "Server returned error no_permission"),
            LookupByEmailError::NotAuthed => write!(f, "Server returned error not_authed"),
            LookupByEmailError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            LookupByEmailError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            LookupByEmailError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            LookupByEmailError::UsersNotFound => write!(f, "Server returned error users_not_found"),
            LookupByEmailError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            LookupByEmailError::Unknown(ref s) => write!(f, "{}", s),
            LookupByEmailError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for LookupByEmailError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LookupByEmailError::MalformedResponse(_, ref e) => Some(e),
            LookupByEmailError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetActiveRequest {}

#[derive(Clone, Debug, Deserialize)]
pub struct SetActiveResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetActiveResponse, SetActiveError<E>>> for SetActiveResponse {
    fn into(self) -> Result<SetActiveResponse, SetActiveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetActiveError<E: Error> {
    AccountInactive,
    EkmAccessDenied,
    FatalError,
    InternalError,
    InvalidArgName,
    InvalidArguments,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidPostType,
    MissingPostType,
    MissingScope,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetActiveError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetActiveError::AccountInactive,
            "ekm_access_denied" => SetActiveError::EkmAccessDenied,
            "fatal_error" => SetActiveError::FatalError,
            "internal_error" => SetActiveError::InternalError,
            "invalid_arg_name" => SetActiveError::InvalidArgName,
            "invalid_arguments" => SetActiveError::InvalidArguments,
            "invalid_auth" => SetActiveError::InvalidAuth,
            "invalid_charset" => SetActiveError::InvalidCharset,
            "invalid_form_data" => SetActiveError::InvalidFormData,
            "invalid_post_type" => SetActiveError::InvalidPostType,
            "missing_post_type" => SetActiveError::MissingPostType,
            "missing_scope" => SetActiveError::MissingScope,
            "no_permission" => SetActiveError::NoPermission,
            "not_authed" => SetActiveError::NotAuthed,
            "org_login_required" => SetActiveError::OrgLoginRequired,
            "request_timeout" => SetActiveError::RequestTimeout,
            "team_added_to_org" => SetActiveError::TeamAddedToOrg,
            "token_revoked" => SetActiveError::TokenRevoked,
            _ => SetActiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetActiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetActiveError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetActiveError::EkmAccessDenied => write!(f, "Server returned error ekm_access_denied"),
            SetActiveError::FatalError => write!(f, "Server returned error fatal_error"),
            SetActiveError::InternalError => write!(f, "Server returned error internal_error"),
            SetActiveError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetActiveError::InvalidArguments => {
                write!(f, "Server returned error invalid_arguments")
            }
            SetActiveError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetActiveError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetActiveError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetActiveError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetActiveError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetActiveError::MissingScope => write!(f, "Server returned error missing_scope"),
            SetActiveError::NoPermission => write!(f, "Server returned error no_permission"),
            SetActiveError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetActiveError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SetActiveError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetActiveError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetActiveError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetActiveError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetActiveError::Unknown(ref s) => write!(f, "{}", s),
            SetActiveError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetActiveError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetActiveError::MalformedResponse(_, ref e) => Some(e),
            SetActiveError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetPhotoRequest<'a> {
    /// Width/height of crop box (always square)
    pub crop_w: Option<Cow<'a, str>>,
    /// X coordinate of top-left corner of crop box
    pub crop_x: Option<Cow<'a, str>>,
    /// Y coordinate of top-left corner of crop box
    pub crop_y: Option<Cow<'a, str>>,
    /// File contents via `multipart/form-data`.
    pub image: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPhotoProfileInner {
    pub avatar_hash: String,
    pub image_1024: String,
    pub image_192: String,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_512: String,
    pub image_72: String,
    pub image_original: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPhotoResponse {
    pub callstack: Option<String>,
    pub debug_step: Option<String>,
    pub dims: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub profile: SetPhotoProfileInner,
    pub time_ident: Option<u64>,
}

impl<E: Error> Into<Result<SetPhotoResponse, SetPhotoError<E>>> for SetPhotoResponse {
    fn into(self) -> Result<SetPhotoResponse, SetPhotoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetPhotoError<E: Error> {
    AccountInactive,
    BadImage,
    FatalError,
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
    NotFound,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    TooLarge,
    TooManyFrames,
    UpgradeRequired,
    UserIsBot,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetPhotoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetPhotoError::AccountInactive,
            "bad_image" => SetPhotoError::BadImage,
            "fatal_error" => SetPhotoError::FatalError,
            "invalid_arg_name" => SetPhotoError::InvalidArgName,
            "invalid_array_arg" => SetPhotoError::InvalidArrayArg,
            "invalid_auth" => SetPhotoError::InvalidAuth,
            "invalid_charset" => SetPhotoError::InvalidCharset,
            "invalid_form_data" => SetPhotoError::InvalidFormData,
            "invalid_json" => SetPhotoError::InvalidJson,
            "invalid_post_type" => SetPhotoError::InvalidPostType,
            "json_not_object" => SetPhotoError::JsonNotObject,
            "missing_post_type" => SetPhotoError::MissingPostType,
            "no_permission" => SetPhotoError::NoPermission,
            "not_authed" => SetPhotoError::NotAuthed,
            "not_found" => SetPhotoError::NotFound,
            "org_login_required" => SetPhotoError::OrgLoginRequired,
            "request_timeout" => SetPhotoError::RequestTimeout,
            "team_added_to_org" => SetPhotoError::TeamAddedToOrg,
            "token_revoked" => SetPhotoError::TokenRevoked,
            "too_large" => SetPhotoError::TooLarge,
            "too_many_frames" => SetPhotoError::TooManyFrames,
            "upgrade_required" => SetPhotoError::UpgradeRequired,
            "user_is_bot" => SetPhotoError::UserIsBot,
            _ => SetPhotoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPhotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetPhotoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            SetPhotoError::BadImage => write!(f, "Server returned error bad_image"),
            SetPhotoError::FatalError => write!(f, "Server returned error fatal_error"),
            SetPhotoError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetPhotoError::InvalidArrayArg => write!(f, "Server returned error invalid_array_arg"),
            SetPhotoError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetPhotoError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetPhotoError::InvalidFormData => write!(f, "Server returned error invalid_form_data"),
            SetPhotoError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetPhotoError::InvalidPostType => write!(f, "Server returned error invalid_post_type"),
            SetPhotoError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetPhotoError::MissingPostType => write!(f, "Server returned error missing_post_type"),
            SetPhotoError::NoPermission => write!(f, "Server returned error no_permission"),
            SetPhotoError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetPhotoError::NotFound => write!(f, "Server returned error not_found"),
            SetPhotoError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SetPhotoError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetPhotoError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            SetPhotoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetPhotoError::TooLarge => write!(f, "Server returned error too_large"),
            SetPhotoError::TooManyFrames => write!(f, "Server returned error too_many_frames"),
            SetPhotoError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            SetPhotoError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            SetPhotoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetPhotoError::Unknown(ref s) => write!(f, "{}", s),
            SetPhotoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetPhotoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetPhotoError::MalformedResponse(_, ref e) => Some(e),
            SetPhotoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetPresenceRequest<'a> {
    /// Either `auto` or `away`
    pub presence: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetPresenceResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetPresenceResponse, SetPresenceError<E>>> for SetPresenceResponse {
    fn into(self) -> Result<SetPresenceResponse, SetPresenceError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetPresenceError<E: Error> {
    AccountInactive,
    FatalError,
    InvalidArgName,
    InvalidArrayArg,
    InvalidAuth,
    InvalidCharset,
    InvalidFormData,
    InvalidJson,
    InvalidPostType,
    InvalidPresence,
    JsonNotObject,
    MissingPostType,
    NoPermission,
    NotAuthed,
    OrgLoginRequired,
    RequestTimeout,
    TeamAddedToOrg,
    TokenRevoked,
    UpgradeRequired,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetPresenceError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => SetPresenceError::AccountInactive,
            "fatal_error" => SetPresenceError::FatalError,
            "invalid_arg_name" => SetPresenceError::InvalidArgName,
            "invalid_array_arg" => SetPresenceError::InvalidArrayArg,
            "invalid_auth" => SetPresenceError::InvalidAuth,
            "invalid_charset" => SetPresenceError::InvalidCharset,
            "invalid_form_data" => SetPresenceError::InvalidFormData,
            "invalid_json" => SetPresenceError::InvalidJson,
            "invalid_post_type" => SetPresenceError::InvalidPostType,
            "invalid_presence" => SetPresenceError::InvalidPresence,
            "json_not_object" => SetPresenceError::JsonNotObject,
            "missing_post_type" => SetPresenceError::MissingPostType,
            "no_permission" => SetPresenceError::NoPermission,
            "not_authed" => SetPresenceError::NotAuthed,
            "org_login_required" => SetPresenceError::OrgLoginRequired,
            "request_timeout" => SetPresenceError::RequestTimeout,
            "team_added_to_org" => SetPresenceError::TeamAddedToOrg,
            "token_revoked" => SetPresenceError::TokenRevoked,
            "upgrade_required" => SetPresenceError::UpgradeRequired,
            _ => SetPresenceError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetPresenceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetPresenceError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            SetPresenceError::FatalError => write!(f, "Server returned error fatal_error"),
            SetPresenceError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            SetPresenceError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            SetPresenceError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SetPresenceError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            SetPresenceError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            SetPresenceError::InvalidJson => write!(f, "Server returned error invalid_json"),
            SetPresenceError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            SetPresenceError::InvalidPresence => {
                write!(f, "Server returned error invalid_presence")
            }
            SetPresenceError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            SetPresenceError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            SetPresenceError::NoPermission => write!(f, "Server returned error no_permission"),
            SetPresenceError::NotAuthed => write!(f, "Server returned error not_authed"),
            SetPresenceError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            SetPresenceError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            SetPresenceError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            SetPresenceError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            SetPresenceError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            SetPresenceError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetPresenceError::Unknown(ref s) => write!(f, "{}", s),
            SetPresenceError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetPresenceError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetPresenceError::MalformedResponse(_, ref e) => Some(e),
            SetPresenceError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
