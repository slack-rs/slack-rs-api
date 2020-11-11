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
pub struct AccessLogsRequest<'a> {
    /// End of time range of logs to include in results (inclusive).
    pub before: Option<Cow<'a, str>>,
    pub count: Option<Cow<'a, str>>,
    pub page: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessLogsLoginsInner {
    pub count: u64,
    pub country: String,
    pub date_first: u64,
    pub date_last: u64,
    pub ip: String,
    pub isp: String,
    pub region: String,
    pub user_agent: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessLogsPagingInner {
    pub count: Option<u64>,
    pub page: u64,
    pub pages: Option<u64>,
    pub per_page: Option<u64>,
    pub spill: Option<u64>,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessLogsResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub logins: Vec<AccessLogsLoginsInner>,
    #[serde(default)]
    ok: bool,
    pub paging: AccessLogsPagingInner,
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
    OverPaginationLimit,
    PaidOnly,
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

impl<'a, E: Error> From<&'a str> for AccessLogsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => AccessLogsError::AccountInactive,
            "fatal_error" => AccessLogsError::FatalError,
            "invalid_arg_name" => AccessLogsError::InvalidArgName,
            "invalid_array_arg" => AccessLogsError::InvalidArrayArg,
            "invalid_auth" => AccessLogsError::InvalidAuth,
            "invalid_charset" => AccessLogsError::InvalidCharset,
            "invalid_form_data" => AccessLogsError::InvalidFormData,
            "invalid_json" => AccessLogsError::InvalidJson,
            "invalid_post_type" => AccessLogsError::InvalidPostType,
            "json_not_object" => AccessLogsError::JsonNotObject,
            "missing_post_type" => AccessLogsError::MissingPostType,
            "no_permission" => AccessLogsError::NoPermission,
            "not_authed" => AccessLogsError::NotAuthed,
            "org_login_required" => AccessLogsError::OrgLoginRequired,
            "over_pagination_limit" => AccessLogsError::OverPaginationLimit,
            "paid_only" => AccessLogsError::PaidOnly,
            "request_timeout" => AccessLogsError::RequestTimeout,
            "team_added_to_org" => AccessLogsError::TeamAddedToOrg,
            "token_revoked" => AccessLogsError::TokenRevoked,
            "upgrade_required" => AccessLogsError::UpgradeRequired,
            "user_is_bot" => AccessLogsError::UserIsBot,
            _ => AccessLogsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AccessLogsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AccessLogsError::AccountInactive => write!(f, "Server returned error account_inactive"),
            AccessLogsError::FatalError => write!(f, "Server returned error fatal_error"),
            AccessLogsError::InvalidArgName => write!(f, "Server returned error invalid_arg_name"),
            AccessLogsError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            AccessLogsError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            AccessLogsError::InvalidCharset => write!(f, "Server returned error invalid_charset"),
            AccessLogsError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            AccessLogsError::InvalidJson => write!(f, "Server returned error invalid_json"),
            AccessLogsError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            AccessLogsError::JsonNotObject => write!(f, "Server returned error json_not_object"),
            AccessLogsError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            AccessLogsError::NoPermission => write!(f, "Server returned error no_permission"),
            AccessLogsError::NotAuthed => write!(f, "Server returned error not_authed"),
            AccessLogsError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            AccessLogsError::OverPaginationLimit => {
                write!(f, "Server returned error over_pagination_limit")
            }
            AccessLogsError::PaidOnly => write!(f, "Server returned error paid_only"),
            AccessLogsError::RequestTimeout => write!(f, "Server returned error request_timeout"),
            AccessLogsError::TeamAddedToOrg => write!(f, "Server returned error team_added_to_org"),
            AccessLogsError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            AccessLogsError::UpgradeRequired => write!(f, "Server returned error upgrade_required"),
            AccessLogsError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            AccessLogsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AccessLogsError::Unknown(ref s) => write!(f, "{}", s),
            AccessLogsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AccessLogsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AccessLogsError::MalformedResponse(_, ref e) => Some(e),
            AccessLogsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct BillableInfoRequest<'a> {
    /// A user to retrieve the billable information for. Defaults to all users.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BillableInfoResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<BillableInfoResponse, BillableInfoError<E>>> for BillableInfoResponse {
    fn into(self) -> Result<BillableInfoResponse, BillableInfoError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(BillableInfoError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum BillableInfoError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for BillableInfoError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => BillableInfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for BillableInfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BillableInfoError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            BillableInfoError::Unknown(ref s) => write!(f, "{}", s),
            BillableInfoError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for BillableInfoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            BillableInfoError::MalformedResponse(_, ref e) => Some(e),
            BillableInfoError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InfoRequest<'a> {
    /// Team to get info on, if omitted, will return information about the current team. Will only return team that the authenticated token is allowed to see through external shared channels
    pub team: Option<Cow<'a, str>>,
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
pub struct InfoResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub team: InfoTeamInner,
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
    TokenRevoked,
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
            "fatal_error" => InfoError::FatalError,
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
            "token_revoked" => InfoError::TokenRevoked,
            "upgrade_required" => InfoError::UpgradeRequired,
            _ => InfoError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InfoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InfoError::AccountInactive => write!(f, "Server returned error account_inactive"),
            InfoError::FatalError => write!(f, "Server returned error fatal_error"),
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
            InfoError::TokenRevoked => write!(f, "Server returned error token_revoked"),
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
pub struct IntegrationLogsRequest<'a> {
    /// Filter logs to this Slack app. Defaults to all logs.
    pub app_id: Option<Cow<'a, str>>,
    /// Filter logs with this change type. Defaults to all logs.
    pub change_type: Option<Cow<'a, str>>,
    pub count: Option<Cow<'a, str>>,
    pub page: Option<Cow<'a, str>>,
    /// Filter logs to this service. Defaults to all logs.
    pub service_id: Option<Cow<'a, str>>,
    /// Filter logs generated by this user’s actions. Defaults to all logs.
    pub user: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationLogsLogsInner {
    pub admin_app_id: Option<String>,
    pub app_id: String,
    pub app_type: String,
    pub change_type: String,
    pub channel: Option<String>,
    pub date: String,
    pub scope: String,
    pub service_id: Option<String>,
    pub service_type: Option<String>,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationLogsPagingInner {
    pub count: Option<u64>,
    pub page: u64,
    pub pages: Option<u64>,
    pub per_page: Option<u64>,
    pub spill: Option<u64>,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationLogsResponse {
    pub callstack: Option<String>,
    error: Option<String>,
    pub logs: Vec<IntegrationLogsLogsInner>,
    #[serde(default)]
    ok: bool,
    pub paging: IntegrationLogsPagingInner,
}

impl<E: Error> Into<Result<IntegrationLogsResponse, IntegrationLogsError<E>>>
    for IntegrationLogsResponse
{
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

impl<'a, E: Error> From<&'a str> for IntegrationLogsError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "account_inactive" => IntegrationLogsError::AccountInactive,
            "fatal_error" => IntegrationLogsError::FatalError,
            "invalid_arg_name" => IntegrationLogsError::InvalidArgName,
            "invalid_array_arg" => IntegrationLogsError::InvalidArrayArg,
            "invalid_auth" => IntegrationLogsError::InvalidAuth,
            "invalid_charset" => IntegrationLogsError::InvalidCharset,
            "invalid_form_data" => IntegrationLogsError::InvalidFormData,
            "invalid_json" => IntegrationLogsError::InvalidJson,
            "invalid_post_type" => IntegrationLogsError::InvalidPostType,
            "json_not_object" => IntegrationLogsError::JsonNotObject,
            "missing_post_type" => IntegrationLogsError::MissingPostType,
            "no_permission" => IntegrationLogsError::NoPermission,
            "not_authed" => IntegrationLogsError::NotAuthed,
            "org_login_required" => IntegrationLogsError::OrgLoginRequired,
            "request_timeout" => IntegrationLogsError::RequestTimeout,
            "team_added_to_org" => IntegrationLogsError::TeamAddedToOrg,
            "token_revoked" => IntegrationLogsError::TokenRevoked,
            "upgrade_required" => IntegrationLogsError::UpgradeRequired,
            "user_is_bot" => IntegrationLogsError::UserIsBot,
            _ => IntegrationLogsError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for IntegrationLogsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IntegrationLogsError::AccountInactive => {
                write!(f, "Server returned error account_inactive")
            }
            IntegrationLogsError::FatalError => write!(f, "Server returned error fatal_error"),
            IntegrationLogsError::InvalidArgName => {
                write!(f, "Server returned error invalid_arg_name")
            }
            IntegrationLogsError::InvalidArrayArg => {
                write!(f, "Server returned error invalid_array_arg")
            }
            IntegrationLogsError::InvalidAuth => write!(f, "Server returned error invalid_auth"),
            IntegrationLogsError::InvalidCharset => {
                write!(f, "Server returned error invalid_charset")
            }
            IntegrationLogsError::InvalidFormData => {
                write!(f, "Server returned error invalid_form_data")
            }
            IntegrationLogsError::InvalidJson => write!(f, "Server returned error invalid_json"),
            IntegrationLogsError::InvalidPostType => {
                write!(f, "Server returned error invalid_post_type")
            }
            IntegrationLogsError::JsonNotObject => {
                write!(f, "Server returned error json_not_object")
            }
            IntegrationLogsError::MissingPostType => {
                write!(f, "Server returned error missing_post_type")
            }
            IntegrationLogsError::NoPermission => write!(f, "Server returned error no_permission"),
            IntegrationLogsError::NotAuthed => write!(f, "Server returned error not_authed"),
            IntegrationLogsError::OrgLoginRequired => {
                write!(f, "Server returned error org_login_required")
            }
            IntegrationLogsError::RequestTimeout => {
                write!(f, "Server returned error request_timeout")
            }
            IntegrationLogsError::TeamAddedToOrg => {
                write!(f, "Server returned error team_added_to_org")
            }
            IntegrationLogsError::TokenRevoked => write!(f, "Server returned error token_revoked"),
            IntegrationLogsError::UpgradeRequired => {
                write!(f, "Server returned error upgrade_required")
            }
            IntegrationLogsError::UserIsBot => write!(f, "Server returned error user_is_bot"),
            IntegrationLogsError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            IntegrationLogsError::Unknown(ref s) => write!(f, "{}", s),
            IntegrationLogsError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for IntegrationLogsError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            IntegrationLogsError::MalformedResponse(_, ref e) => Some(e),
            IntegrationLogsError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
