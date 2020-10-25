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

pub mod ekm_types;
pub mod restrict_access_types;

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct UnarchiveRequest {
    /// The channel to unarchive.
    pub channel_id: String,
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

#[derive(Debug)]
pub enum UnarchiveError<E: Error> {
    FeatureNotEnabled,
    ChannelNotFound,
    ChannelNotArchived,
    ChannelTypeNotSupported,
    RestrictedAction,
    CouldNotUnarchiveChannel,
    DefaultOrgWideChannel,
    MissingScope,
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
            "feature_not_enabled" => UnarchiveError::FeatureNotEnabled,
            "channel_not_found" => UnarchiveError::ChannelNotFound,
            "channel_not_archived" => UnarchiveError::ChannelNotArchived,
            "channel_type_not_supported" => UnarchiveError::ChannelTypeNotSupported,
            "restricted_action" => UnarchiveError::RestrictedAction,
            "could_not_unarchive_channel" => UnarchiveError::CouldNotUnarchiveChannel,
            "default_org_wide_channel" => UnarchiveError::DefaultOrgWideChannel,
            "missing_scope" => UnarchiveError::MissingScope,
            _ => UnarchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UnarchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnarchiveError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            UnarchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            UnarchiveError::ChannelNotArchived => {
                write!(f, "Server returned error channel_not_archived")
            }
            UnarchiveError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            UnarchiveError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            UnarchiveError::CouldNotUnarchiveChannel => {
                write!(f, "Server returned error could_not_unarchive_channel")
            }
            UnarchiveError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            UnarchiveError::MissingScope => write!(f, "Server returned error missing_scope"),
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
pub struct DisconnectSharedRequest {
    /// The channel to be disconnected from some workspaces.
    pub channel_id: String,
    /// The team to be removed from the channel. Currently only a single team id can be specified.
    pub leaving_team_ids: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisconnectSharedResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<DisconnectSharedResponse, DisconnectSharedError<E>>>
    for DisconnectSharedResponse
{
    fn into(self) -> Result<DisconnectSharedResponse, DisconnectSharedError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum DisconnectSharedError<E: Error> {
    FeatureNotEnabled,
    NotAnAdmin,
    NotAnEnterprise,
    ChannelNotFound,
    NotSupported,
    TeamNotFound,
    RestrictedAction,
    MissingScope,
    LeavingTeamNotInChannel,
    NoTeamsToDisconnect,
    LeavingTeamRequired,
    CannotKickTeam,
    CannotKickHomeTeam,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for DisconnectSharedError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => DisconnectSharedError::FeatureNotEnabled,
            "not_an_admin" => DisconnectSharedError::NotAnAdmin,
            "not_an_enterprise" => DisconnectSharedError::NotAnEnterprise,
            "channel_not_found" => DisconnectSharedError::ChannelNotFound,
            "not_supported" => DisconnectSharedError::NotSupported,
            "team_not_found" => DisconnectSharedError::TeamNotFound,
            "restricted_action" => DisconnectSharedError::RestrictedAction,
            "missing_scope" => DisconnectSharedError::MissingScope,
            "leaving_team_not_in_channel" => DisconnectSharedError::LeavingTeamNotInChannel,
            "no_teams_to_disconnect" => DisconnectSharedError::NoTeamsToDisconnect,
            "leaving_team_required" => DisconnectSharedError::LeavingTeamRequired,
            "cannot_kick_team" => DisconnectSharedError::CannotKickTeam,
            "cannot_kick_home_team" => DisconnectSharedError::CannotKickHomeTeam,
            _ => DisconnectSharedError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DisconnectSharedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DisconnectSharedError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            DisconnectSharedError::NotAnAdmin => write!(f, "Server returned error not_an_admin"),
            DisconnectSharedError::NotAnEnterprise => {
                write!(f, "Server returned error not_an_enterprise")
            }
            DisconnectSharedError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            DisconnectSharedError::NotSupported => write!(f, "Server returned error not_supported"),
            DisconnectSharedError::TeamNotFound => {
                write!(f, "Server returned error team_not_found")
            }
            DisconnectSharedError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            DisconnectSharedError::MissingScope => write!(f, "Server returned error missing_scope"),
            DisconnectSharedError::LeavingTeamNotInChannel => {
                write!(f, "Server returned error leaving_team_not_in_channel")
            }
            DisconnectSharedError::NoTeamsToDisconnect => {
                write!(f, "Server returned error no_teams_to_disconnect")
            }
            DisconnectSharedError::LeavingTeamRequired => {
                write!(f, "Server returned error leaving_team_required")
            }
            DisconnectSharedError::CannotKickTeam => {
                write!(f, "Server returned error cannot_kick_team")
            }
            DisconnectSharedError::CannotKickHomeTeam => {
                write!(f, "Server returned error cannot_kick_home_team")
            }
            DisconnectSharedError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            DisconnectSharedError::Unknown(ref s) => write!(f, "{}", s),
            DisconnectSharedError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for DisconnectSharedError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DisconnectSharedError::MalformedResponse(_, ref e) => Some(e),
            DisconnectSharedError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetConversationPrefsRequest {
    /// The channel to get preferences for.
    pub channel_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationPrefsCanThreadInner {
    pub r#type: Option<Vec<String>>,
    pub user: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationPrefsWhoCanPostInner {
    pub r#type: Option<Vec<String>>,
    pub user: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationPrefsPrefsInner {
    pub can_thread: Option<GetConversationPrefsCanThreadInner>,
    pub who_can_post: Option<GetConversationPrefsWhoCanPostInner>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationPrefsResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub prefs: Option<GetConversationPrefsPrefsInner>,
}

impl<E: Error> Into<Result<GetConversationPrefsResponse, GetConversationPrefsError<E>>>
    for GetConversationPrefsResponse
{
    fn into(self) -> Result<GetConversationPrefsResponse, GetConversationPrefsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetConversationPrefsError<E: Error> {
    FeatureNotEnabled,
    NotAnAdmin,
    NotAnEnterprise,
    RestrictedAction,
    MissingScope,
    ChannelNotFound,
    ChannelTypeNotSupported,
    CouldNotGetConversationPrefs,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for GetConversationPrefsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => GetConversationPrefsError::FeatureNotEnabled,
            "not_an_admin" => GetConversationPrefsError::NotAnAdmin,
            "not_an_enterprise" => GetConversationPrefsError::NotAnEnterprise,
            "restricted_action" => GetConversationPrefsError::RestrictedAction,
            "missing_scope" => GetConversationPrefsError::MissingScope,
            "channel_not_found" => GetConversationPrefsError::ChannelNotFound,
            "channel_type_not_supported" => GetConversationPrefsError::ChannelTypeNotSupported,
            "could_not_get_conversation_prefs" => {
                GetConversationPrefsError::CouldNotGetConversationPrefs
            }
            _ => GetConversationPrefsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetConversationPrefsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetConversationPrefsError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            GetConversationPrefsError::NotAnAdmin => {
                write!(f, "Server returned error not_an_admin")
            }
            GetConversationPrefsError::NotAnEnterprise => {
                write!(f, "Server returned error not_an_enterprise")
            }
            GetConversationPrefsError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            GetConversationPrefsError::MissingScope => {
                write!(f, "Server returned error missing_scope")
            }
            GetConversationPrefsError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            GetConversationPrefsError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            GetConversationPrefsError::CouldNotGetConversationPrefs => {
                write!(f, "Server returned error could_not_get_conversation_prefs")
            }
            GetConversationPrefsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetConversationPrefsError::Unknown(ref s) => write!(f, "{}", s),
            GetConversationPrefsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetConversationPrefsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetConversationPrefsError::MalformedResponse(_, ref e) => Some(e),
            GetConversationPrefsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetTeamsRequest {
    /// The channel to determine connected workspaces within the organization for.
    pub channel_id: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetTeamsResponseMetadataInner {
    pub next_cursor: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetTeamsResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub response_metadata: Option<GetTeamsResponseMetadataInner>,
    pub team_ids: Vec<String>,
}

impl<E: Error> Into<Result<GetTeamsResponse, GetTeamsError<E>>> for GetTeamsResponse {
    fn into(self) -> Result<GetTeamsResponse, GetTeamsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum GetTeamsError<E: Error> {
    FeatureNotEnabled,
    ChannelNotFound,
    ChannelTypeNotSupported,
    UnsupportedTeamType,
    RestrictedAction,
    CouldNotGetTeams,
    InvalidCursor,
    InvalidLimit,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for GetTeamsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => GetTeamsError::FeatureNotEnabled,
            "channel_not_found" => GetTeamsError::ChannelNotFound,
            "channel_type_not_supported" => GetTeamsError::ChannelTypeNotSupported,
            "unsupported_team_type" => GetTeamsError::UnsupportedTeamType,
            "restricted_action" => GetTeamsError::RestrictedAction,
            "could_not_get_teams" => GetTeamsError::CouldNotGetTeams,
            "invalid_cursor" => GetTeamsError::InvalidCursor,
            "invalid_limit" => GetTeamsError::InvalidLimit,
            _ => GetTeamsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for GetTeamsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetTeamsError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            GetTeamsError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            GetTeamsError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            GetTeamsError::UnsupportedTeamType => {
                write!(f, "Server returned error unsupported_team_type")
            }
            GetTeamsError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            GetTeamsError::CouldNotGetTeams => {
                write!(f, "Server returned error could_not_get_teams")
            }
            GetTeamsError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            GetTeamsError::InvalidLimit => write!(f, "Server returned error invalid_limit"),
            GetTeamsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            GetTeamsError::Unknown(ref s) => write!(f, "{}", s),
            GetTeamsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for GetTeamsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GetTeamsError::MalformedResponse(_, ref e) => Some(e),
            GetTeamsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SearchRequest {
    /// Comma separated string of team IDs, signifying the workspaces to search through.
    pub team_ids: Option<String>,
    /// Name of the the channel to query by.
    pub query: Option<String>,
    /// Maximum number of items to be returned. Must be between 1 - 20 both inclusive. Default is 10.
    pub limit: Option<u64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    pub cursor: Option<String>,
    /// The type of channel to include or exclude in the search. For example `private` will search private channels, while `private_exclude` will exclude them. For a full list of types, check the [Types section](#types).
    pub search_channel_types: Option<String>,
    /// Possible values are `relevant` (search ranking based on what we think is closest), `name` (alphabetical), `member_count` (number of users in the channel), and `created` (date channel was created). You can optionally pair this with the `sort_dir` arg to change how it is sorted
    pub sort: Option<String>,
    /// Sort direction. Possible values are `asc` for ascending order like (1, 2, 3) or (a, b, c), and `desc` for descending order like (3, 2, 1) or (c, b, a)
    pub sort_dir: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchAttachmentsInner {
    pub fallback: Option<String>,
    pub id: u64,
    pub image_bytes: Option<u64>,
    pub image_height: Option<u64>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchBlocksInner {
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchIconsInner {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchBotProfileInner {
    pub app_id: String,
    pub deleted: bool,
    pub icons: SearchIconsInner,
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub updated: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchReactionsInner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchCommentInner {
    pub comment: String,
    pub created: u64,
    pub id: String,
    pub is_intro: bool,
    pub is_starred: Option<bool>,
    pub num_stars: Option<u64>,
    pub pinned_info: Option<serde_json::Value>,
    pub pinned_to: Option<Vec<String>>,
    pub reactions: Option<Vec<SearchReactionsInner>>,
    pub timestamp: u64,
    pub user: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchReactions1Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchSharesInner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchFileInner {
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
    pub reactions: Option<Vec<SearchReactions1Inner>>,
    pub shares: Option<SearchSharesInner>,
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
pub struct SearchReactions2Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchShares1Inner {
    pub private: Option<serde_json::Value>,
    pub public: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchFilesInner {
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
    pub reactions: Option<Vec<SearchReactions2Inner>>,
    pub shares: Option<SearchShares1Inner>,
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
pub struct SearchIcons1Inner {
    pub emoji: Option<String>,
    pub image_64: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchReactions3Inner {
    pub count: u64,
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchUserProfileInner {
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
pub struct SearchLatestInner {
    pub attachments: Option<Vec<SearchAttachmentsInner>>,
    pub blocks: Option<Vec<SearchBlocksInner>>,
    pub bot_id: Option<Vec<String>>,
    pub bot_profile: Option<SearchBotProfileInner>,
    pub client_msg_id: Option<String>,
    pub comment: Option<SearchCommentInner>,
    pub display_as_bot: Option<bool>,
    pub file: Option<SearchFileInner>,
    pub files: Option<Vec<SearchFilesInner>>,
    pub icons: Option<SearchIcons1Inner>,
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
    pub reactions: Option<Vec<SearchReactions3Inner>>,
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
    pub user_profile: Option<SearchUserProfileInner>,
    pub user_team: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchPurposeInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchTopicInner {
    pub creator: String,
    pub last_set: u64,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchChannelsInner {
    pub accepted_user: Option<String>,
    pub created: u64,
    pub creator: String,
    pub id: String,
    pub is_archived: Option<bool>,
    pub is_channel: bool,
    pub is_frozen: Option<bool>,
    pub is_general: Option<bool>,
    pub is_member: Option<bool>,
    pub is_moved: Option<u64>,
    pub is_mpim: bool,
    pub is_non_threadable: Option<bool>,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: bool,
    pub is_read_only: Option<bool>,
    pub is_shared: bool,
    pub is_thread_only: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Vec<SearchLatestInner>>,
    pub members: Vec<String>,
    pub name: String,
    pub name_normalized: String,
    pub num_members: Option<u64>,
    pub pending_shared: Option<Vec<String>>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<u64>,
    pub purpose: SearchPurposeInner,
    pub topic: SearchTopicInner,
    pub unlinked: Option<u64>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchResponse {
    pub channels: Vec<SearchChannelsInner>,
    error: Option<String>,
    pub next_cursor: String,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SearchResponse, SearchError<E>>> for SearchResponse {
    fn into(self) -> Result<SearchResponse, SearchError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SearchError<E: Error> {
    FeatureNotEnabled,
    NotAnAdmin,
    NotAnEnterprise,
    TeamNotFound,
    NotAllowed,
    InvalidAuth,
    InvalidCursor,
    InvalidSearchChannelType,
    InvalidSort,
    InvalidSortDir,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SearchError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => SearchError::FeatureNotEnabled,
            "not_an_admin" => SearchError::NotAnAdmin,
            "not_an_enterprise" => SearchError::NotAnEnterprise,
            "team_not_found" => SearchError::TeamNotFound,
            "not_allowed" => SearchError::NotAllowed,
            "invalid_auth" => SearchError::InvalidAuth,
            "invalid_cursor" => SearchError::InvalidCursor,
            "invalid_search_channel_type" => SearchError::InvalidSearchChannelType,
            "invalid_sort" => SearchError::InvalidSort,
            "invalid_sort_dir" => SearchError::InvalidSortDir,
            _ => SearchError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SearchError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SearchError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            SearchError::NotAnAdmin => write!(f, "Server returned error not_an_admin"),
            SearchError::NotAnEnterprise => write!(f, "Server returned error not_an_enterprise"),
            SearchError::TeamNotFound => write!(f, "Server returned error team_not_found"),
            SearchError::NotAllowed => write!(f, "Server returned error not_allowed"),
            SearchError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            SearchError::InvalidCursor => write!(f, "Server returned error invalid_cursor"),
            SearchError::InvalidSearchChannelType => {
                write!(f, "Server returned error invalid_search_channel_type")
            }
            SearchError::InvalidSort => write!(f, "Server returned error invalid_sort"),
            SearchError::InvalidSortDir => write!(f, "Server returned error invalid_sort_dir"),
            SearchError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SearchError::Unknown(ref s) => write!(f, "{}", s),
            SearchError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SearchError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SearchError::MalformedResponse(_, ref e) => Some(e),
            SearchError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InviteRequest {
    /// The users to invite.
    pub user_ids: String,
    /// The channel that the users will be invited to.
    pub channel_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteResponse {
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

#[derive(Debug)]
pub enum InviteError<E: Error> {
    FeatureNotEnabled,
    ChannelNotFound,
    ChannelTypeNotSupported,
    DefaultOrgWideChannel,
    RestrictedAction,
    UserMustBeAdmin,
    FailedForSomeUsers,
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
            "feature_not_enabled" => InviteError::FeatureNotEnabled,
            "channel_not_found" => InviteError::ChannelNotFound,
            "channel_type_not_supported" => InviteError::ChannelTypeNotSupported,
            "default_org_wide_channel" => InviteError::DefaultOrgWideChannel,
            "restricted_action" => InviteError::RestrictedAction,
            "user_must_be_admin" => InviteError::UserMustBeAdmin,
            "failed_for_some_users" => InviteError::FailedForSomeUsers,
            _ => InviteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InviteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InviteError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            InviteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            InviteError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            InviteError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            InviteError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            InviteError::UserMustBeAdmin => write!(f, "Server returned error user_must_be_admin"),
            InviteError::FailedForSomeUsers => {
                write!(f, "Server returned error failed_for_some_users")
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
pub struct CreateRequest {
    /// Name of the public or private channel to create.
    pub name: String,
    /// Description of the public or private channel to create.
    pub description: Option<String>,
    /// When `true`, creates a private channel instead of a public channel
    pub is_private: bool,
    /// When `true`, the channel will be available org-wide. Note: if the channel is not `org_wide=true`, you must specify a `team_id` for this channel
    pub org_wide: Option<bool>,
    /// The workspace to create the channel in. Note: this argument is required unless you set `org_wide=true`.
    pub team_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    pub channel_id: Option<String>,
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

#[derive(Debug)]
pub enum CreateError<E: Error> {
    FeatureNotEnabled,
    NameTaken,
    RestrictedAction,
    TeamNotFound,
    InvalidTeam,
    InvalidName,
    CouldNotCreateChannel,
    TeamIdOrOrgRequired,
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
            "feature_not_enabled" => CreateError::FeatureNotEnabled,
            "name_taken" => CreateError::NameTaken,
            "restricted_action" => CreateError::RestrictedAction,
            "team_not_found" => CreateError::TeamNotFound,
            "invalid_team" => CreateError::InvalidTeam,
            "invalid_name" => CreateError::InvalidName,
            "could_not_create_channel" => CreateError::CouldNotCreateChannel,
            "team_id_or_org_required" => CreateError::TeamIdOrOrgRequired,
            _ => CreateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for CreateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CreateError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            CreateError::NameTaken => write!(f, "Server returned error name_taken"),
            CreateError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            CreateError::TeamNotFound => write!(f, "Server returned error team_not_found"),
            CreateError::InvalidTeam => write!(f, "Server returned error invalid_team"),
            CreateError::InvalidName => write!(f, "Server returned error invalid_name"),
            CreateError::CouldNotCreateChannel => {
                write!(f, "Server returned error could_not_create_channel")
            }
            CreateError::TeamIdOrOrgRequired => {
                write!(f, "Server returned error team_id_or_org_required")
            }
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
pub struct DeleteRequest {
    /// The channel to delete.
    pub channel_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteResponse {
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
    FeatureNotEnabled,
    NotAnAdmin,
    ChannelNotFound,
    ChannelTypeNotSupported,
    DefaultOrgWideChannel,
    RestrictedAction,
    CouldNotDeleteChannel,
    MissingScope,
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
            "feature_not_enabled" => DeleteError::FeatureNotEnabled,
            "not_an_admin" => DeleteError::NotAnAdmin,
            "channel_not_found" => DeleteError::ChannelNotFound,
            "channel_type_not_supported" => DeleteError::ChannelTypeNotSupported,
            "default_org_wide_channel" => DeleteError::DefaultOrgWideChannel,
            "restricted_action" => DeleteError::RestrictedAction,
            "could_not_delete_channel" => DeleteError::CouldNotDeleteChannel,
            "missing_scope" => DeleteError::MissingScope,
            _ => DeleteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for DeleteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DeleteError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            DeleteError::NotAnAdmin => write!(f, "Server returned error not_an_admin"),
            DeleteError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            DeleteError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            DeleteError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            DeleteError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            DeleteError::CouldNotDeleteChannel => {
                write!(f, "Server returned error could_not_delete_channel")
            }
            DeleteError::MissingScope => write!(f, "Server returned error missing_scope"),
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
pub struct SetConversationPrefsRequest {
    /// The channel to set the prefs for
    pub channel_id: String,
    /// The prefs for this channel in a stringified JSON format.
    pub prefs: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetConversationPrefsResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetConversationPrefsResponse, SetConversationPrefsError<E>>>
    for SetConversationPrefsResponse
{
    fn into(self) -> Result<SetConversationPrefsResponse, SetConversationPrefsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum SetConversationPrefsError<E: Error> {
    FeatureNotEnabled,
    NotAnAdmin,
    ChannelNotFound,
    ChannelTypeNotSupported,
    RestrictedAction,
    MissingScope,
    CouldNotSetChannelPref,
    DefaultOrgWideChannel,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetConversationPrefsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => SetConversationPrefsError::FeatureNotEnabled,
            "not_an_admin" => SetConversationPrefsError::NotAnAdmin,
            "channel_not_found" => SetConversationPrefsError::ChannelNotFound,
            "channel_type_not_supported" => SetConversationPrefsError::ChannelTypeNotSupported,
            "restricted_action" => SetConversationPrefsError::RestrictedAction,
            "missing_scope" => SetConversationPrefsError::MissingScope,
            "could_not_set_channel_pref" => SetConversationPrefsError::CouldNotSetChannelPref,
            "default_org_wide_channel" => SetConversationPrefsError::DefaultOrgWideChannel,
            _ => SetConversationPrefsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetConversationPrefsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetConversationPrefsError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            SetConversationPrefsError::NotAnAdmin => {
                write!(f, "Server returned error not_an_admin")
            }
            SetConversationPrefsError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            SetConversationPrefsError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            SetConversationPrefsError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            SetConversationPrefsError::MissingScope => {
                write!(f, "Server returned error missing_scope")
            }
            SetConversationPrefsError::CouldNotSetChannelPref => {
                write!(f, "Server returned error could_not_set_channel_pref")
            }
            SetConversationPrefsError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            SetConversationPrefsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetConversationPrefsError::Unknown(ref s) => write!(f, "{}", s),
            SetConversationPrefsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetConversationPrefsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetConversationPrefsError::MalformedResponse(_, ref e) => Some(e),
            SetConversationPrefsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RenameRequest {
    /// The channel to rename.
    pub channel_id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RenameResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
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
    FeatureNotEnabled,
    ChannelNotFound,
    ChannelTypeNotSupported,
    RestrictedAction,
    CouldNotRenameChannel,
    DefaultOrgWideChannel,
    NameTaken,
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
            "feature_not_enabled" => RenameError::FeatureNotEnabled,
            "channel_not_found" => RenameError::ChannelNotFound,
            "channel_type_not_supported" => RenameError::ChannelTypeNotSupported,
            "restricted_action" => RenameError::RestrictedAction,
            "could_not_rename_channel" => RenameError::CouldNotRenameChannel,
            "default_org_wide_channel" => RenameError::DefaultOrgWideChannel,
            "name_taken" => RenameError::NameTaken,
            _ => RenameError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RenameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RenameError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            RenameError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            RenameError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            RenameError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            RenameError::CouldNotRenameChannel => {
                write!(f, "Server returned error could_not_rename_channel")
            }
            RenameError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            RenameError::NameTaken => write!(f, "Server returned error name_taken"),
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
pub struct ArchiveRequest {
    /// The channel to archive.
    pub channel_id: String,
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

#[derive(Debug)]
pub enum ArchiveError<E: Error> {
    FeatureNotEnabled,
    ChannelNotFound,
    ChannelTypeNotSupported,
    DefaultOrgWideChannel,
    AlreadyArchived,
    CantArchiveGeneral,
    RestrictedAction,
    CouldNotArchiveChannel,
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
            "feature_not_enabled" => ArchiveError::FeatureNotEnabled,
            "channel_not_found" => ArchiveError::ChannelNotFound,
            "channel_type_not_supported" => ArchiveError::ChannelTypeNotSupported,
            "default_org_wide_channel" => ArchiveError::DefaultOrgWideChannel,
            "already_archived" => ArchiveError::AlreadyArchived,
            "cant_archive_general" => ArchiveError::CantArchiveGeneral,
            "restricted_action" => ArchiveError::RestrictedAction,
            "could_not_archive_channel" => ArchiveError::CouldNotArchiveChannel,
            _ => ArchiveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ArchiveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArchiveError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            ArchiveError::ChannelNotFound => write!(f, "Server returned error channel_not_found"),
            ArchiveError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            ArchiveError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            ArchiveError::AlreadyArchived => write!(f, "Server returned error already_archived"),
            ArchiveError::CantArchiveGeneral => {
                write!(f, "Server returned error cant_archive_general")
            }
            ArchiveError::RestrictedAction => write!(f, "Server returned error restricted_action"),
            ArchiveError::CouldNotArchiveChannel => {
                write!(f, "Server returned error could_not_archive_channel")
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
pub struct ConvertToPrivateRequest {
    /// The channel to convert to private.
    pub channel_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConvertToPrivateResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ConvertToPrivateResponse, ConvertToPrivateError<E>>>
    for ConvertToPrivateResponse
{
    fn into(self) -> Result<ConvertToPrivateResponse, ConvertToPrivateError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}

#[derive(Debug)]
pub enum ConvertToPrivateError<E: Error> {
    FeatureNotEnabled,
    RestrictedAction,
    NameTaken,
    ChannelNotFound,
    ChannelTypeNotSupported,
    DefaultOrgWideChannel,
    MethodNotSupportedForChannelType,
    CouldNotConvertChannel,
    ExternalChannelMigrating,
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for ConvertToPrivateError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "feature_not_enabled" => ConvertToPrivateError::FeatureNotEnabled,
            "restricted_action" => ConvertToPrivateError::RestrictedAction,
            "name_taken" => ConvertToPrivateError::NameTaken,
            "channel_not_found" => ConvertToPrivateError::ChannelNotFound,
            "channel_type_not_supported" => ConvertToPrivateError::ChannelTypeNotSupported,
            "default_org_wide_channel" => ConvertToPrivateError::DefaultOrgWideChannel,
            "method_not_supported_for_channel_type" => {
                ConvertToPrivateError::MethodNotSupportedForChannelType
            }
            "could_not_convert_channel" => ConvertToPrivateError::CouldNotConvertChannel,
            "external_channel_migrating" => ConvertToPrivateError::ExternalChannelMigrating,
            _ => ConvertToPrivateError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ConvertToPrivateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConvertToPrivateError::FeatureNotEnabled => {
                write!(f, "Server returned error feature_not_enabled")
            }
            ConvertToPrivateError::RestrictedAction => {
                write!(f, "Server returned error restricted_action")
            }
            ConvertToPrivateError::NameTaken => write!(f, "Server returned error name_taken"),
            ConvertToPrivateError::ChannelNotFound => {
                write!(f, "Server returned error channel_not_found")
            }
            ConvertToPrivateError::ChannelTypeNotSupported => {
                write!(f, "Server returned error channel_type_not_supported")
            }
            ConvertToPrivateError::DefaultOrgWideChannel => {
                write!(f, "Server returned error default_org_wide_channel")
            }
            ConvertToPrivateError::MethodNotSupportedForChannelType => write!(
                f,
                "Server returned error method_not_supported_for_channel_type"
            ),
            ConvertToPrivateError::CouldNotConvertChannel => {
                write!(f, "Server returned error could_not_convert_channel")
            }
            ConvertToPrivateError::ExternalChannelMigrating => {
                write!(f, "Server returned error external_channel_migrating")
            }
            ConvertToPrivateError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            ConvertToPrivateError::Unknown(ref s) => write!(f, "{}", s),
            ConvertToPrivateError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for ConvertToPrivateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ConvertToPrivateError::MalformedResponse(_, ref e) => Some(e),
            ConvertToPrivateError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetTeamsRequest {
    /// The encoded `channel_id` to add or remove to workspaces.
    pub channel_id: String,
    /// The workspace to which the channel belongs. Omit this argument if the channel is a cross-workspace shared channel.
    pub team_id: Option<String>,
    /// A comma-separated list of workspaces to which the channel should be shared. Not required if the channel is being shared org-wide.
    pub target_team_ids: Option<String>,
    /// True if channel has to be converted to an org channel
    pub org_channel: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetTeamsResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetTeamsResponse, SetTeamsError<E>>> for SetTeamsResponse {
    fn into(self) -> Result<SetTeamsResponse, SetTeamsError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetTeamsError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetTeamsError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetTeamsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetTeamsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetTeamsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetTeamsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetTeamsError::Unknown(ref s) => write!(f, "{}", s),
            SetTeamsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetTeamsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetTeamsError::MalformedResponse(_, ref e) => Some(e),
            SetTeamsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
