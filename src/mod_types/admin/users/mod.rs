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

pub mod session_types;

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct AssignRequest<'a> {
    /// Comma separated values of channel IDs to add user in the new workspace.
    pub channel_ids: Option<Cow<'a, str>>,
    /// True if user should be added to the workspace as a guest.
    pub is_restricted: Option<bool>,
    /// True if user should be added to the workspace as a single-channel guest.
    pub is_ultra_restricted: Option<bool>,
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// The ID of the user to add to the workspace.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssignResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<AssignResponse, AssignError<E>>> for AssignResponse {
    fn into(self) -> Result<AssignResponse, AssignError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(AssignError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum AssignError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for AssignError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => AssignError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for AssignError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AssignError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            AssignError::Unknown(ref s) => write!(f, "{}", s),
            AssignError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for AssignError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AssignError::MalformedResponse(_, ref e) => Some(e),
            AssignError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct InviteRequest<'a> {
    /// A comma-separated list of `channel_id`s for this user to join. At least one channel is required.
    pub channel_ids: Cow<'a, str>,
    /// An optional message to send to the user in the invite email.
    pub custom_message: Option<Cow<'a, str>>,
    /// The email address of the person to invite.
    pub email: Cow<'a, str>,
    /// Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date.
    pub guest_expiration_ts: Option<Cow<'a, str>>,
    /// Is this user a multi-channel guest user? (default: false)
    pub is_restricted: Option<bool>,
    /// Is this user a single channel guest user? (default: false)
    pub is_ultra_restricted: Option<bool>,
    /// Full name of the user.
    pub real_name: Option<Cow<'a, str>>,
    /// Allow this invite to be resent in the future if a user has not signed up yet. (default: false)
    pub resend: Option<bool>,
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InviteResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<InviteResponse, InviteError<E>>> for InviteResponse {
    fn into(self) -> Result<InviteResponse, InviteError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(InviteError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum InviteError<E: Error> {
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
            _ => InviteError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for InviteError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
pub struct ListRequest<'a> {
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    pub cursor: Option<Cow<'a, str>>,
    /// Limit for how many users to be retrieved per page
    pub limit: Option<u64>,
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<ListResponse, ListError<E>>> for ListResponse {
    fn into(self) -> Result<ListResponse, ListError<E>> {
        if self.ok {
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
            _ => ListError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for ListError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// The ID of the user to remove.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<RemoveResponse, RemoveError<E>>> for RemoveResponse {
    fn into(self) -> Result<RemoveResponse, RemoveError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(RemoveError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum RemoveError<E: Error> {
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
            _ => RemoveError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RemoveError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
pub struct SetAdminRequest<'a> {
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// The ID of the user to designate as an admin.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetAdminResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetAdminResponse, SetAdminError<E>>> for SetAdminResponse {
    fn into(self) -> Result<SetAdminResponse, SetAdminError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetAdminError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetAdminError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetAdminError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetAdminError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetAdminError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetAdminError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetAdminError::Unknown(ref s) => write!(f, "{}", s),
            SetAdminError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetAdminError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetAdminError::MalformedResponse(_, ref e) => Some(e),
            SetAdminError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetExpirationRequest<'a> {
    /// Timestamp when guest account should be disabled.
    pub expiration_ts: u64,
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// The ID of the user to set an expiration for.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetExpirationResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetExpirationResponse, SetExpirationError<E>>>
    for SetExpirationResponse
{
    fn into(self) -> Result<SetExpirationResponse, SetExpirationError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetExpirationError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetExpirationError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetExpirationError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetExpirationError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetExpirationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetExpirationError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetExpirationError::Unknown(ref s) => write!(f, "{}", s),
            SetExpirationError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetExpirationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetExpirationError::MalformedResponse(_, ref e) => Some(e),
            SetExpirationError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetOwnerRequest<'a> {
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// Id of the user to promote to owner.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetOwnerResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetOwnerResponse, SetOwnerError<E>>> for SetOwnerResponse {
    fn into(self) -> Result<SetOwnerResponse, SetOwnerError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetOwnerError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetOwnerError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetOwnerError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetOwnerError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetOwnerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetOwnerError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetOwnerError::Unknown(ref s) => write!(f, "{}", s),
            SetOwnerError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetOwnerError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetOwnerError::MalformedResponse(_, ref e) => Some(e),
            SetOwnerError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SetRegularRequest<'a> {
    /// The ID (`T1234`) of the workspace.
    pub team_id: Cow<'a, str>,
    /// The ID of the user to designate as a regular user.
    pub user_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SetRegularResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<SetRegularResponse, SetRegularError<E>>> for SetRegularResponse {
    fn into(self) -> Result<SetRegularResponse, SetRegularError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(SetRegularError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum SetRegularError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for SetRegularError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => SetRegularError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for SetRegularError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SetRegularError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            SetRegularError::Unknown(ref s) => write!(f, "{}", s),
            SetRegularError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for SetRegularError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SetRegularError::MalformedResponse(_, ref e) => Some(e),
            SetRegularError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
