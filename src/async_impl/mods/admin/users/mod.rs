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

pub mod session;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::admin::users::*;

/// Add an Enterprise user to a workspace.
///
/// Wraps https://api.slack.com/methods/admin.users.assign

pub async fn assign<R>(
    client: &R,
    token: &str,
    request: &AssignRequest,
) -> Result<AssignResponse, AssignError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel_ids
            .as_ref()
            .map(|channel_ids| ("channel_ids", channel_ids.to_string())),
        request
            .is_restricted
            .as_ref()
            .map(|is_restricted| ("is_restricted", is_restricted.to_string())),
        request
            .is_ultra_restricted
            .as_ref()
            .map(|is_ultra_restricted| ("is_ultra_restricted", is_ultra_restricted.to_string())),
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.assign");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(AssignError::Client)
        .and_then(|result| {
            serde_json::from_str::<AssignResponse>(&result)
                .map_err(|e| AssignError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Invite a user to a workspace.
///
/// Wraps https://api.slack.com/methods/admin.users.invite

pub async fn invite<R>(
    client: &R,
    token: &str,
    request: &InviteRequest,
) -> Result<InviteResponse, InviteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_ids", request.channel_ids.to_string())),
        request
            .custom_message
            .as_ref()
            .map(|custom_message| ("custom_message", custom_message.to_string())),
        Some(("email", request.email.to_string())),
        request
            .guest_expiration_ts
            .as_ref()
            .map(|guest_expiration_ts| ("guest_expiration_ts", guest_expiration_ts.to_string())),
        request
            .is_restricted
            .as_ref()
            .map(|is_restricted| ("is_restricted", is_restricted.to_string())),
        request
            .is_ultra_restricted
            .as_ref()
            .map(|is_ultra_restricted| ("is_ultra_restricted", is_ultra_restricted.to_string())),
        request
            .real_name
            .as_ref()
            .map(|real_name| ("real_name", real_name.to_string())),
        request
            .resend
            .as_ref()
            .map(|resend| ("resend", resend.to_string())),
        Some(("team_id", request.team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.invite");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// List users on a workspace
///
/// Wraps https://api.slack.com/methods/admin.users.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        Some(("team_id", request.team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.list");
    client
        .get(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Remove a user from a workspace.
///
/// Wraps https://api.slack.com/methods/admin.users.remove

pub async fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.remove");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set an existing guest, regular user, or owner to be an admin user.
///
/// Wraps https://api.slack.com/methods/admin.users.setAdmin

pub async fn set_admin<R>(
    client: &R,
    token: &str,
    request: &SetAdminRequest,
) -> Result<SetAdminResponse, SetAdminError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.setAdmin");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(SetAdminError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetAdminResponse>(&result)
                .map_err(|e| SetAdminError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set an expiration for a guest user
///
/// Wraps https://api.slack.com/methods/admin.users.setExpiration

pub async fn set_expiration<R>(
    client: &R,
    token: &str,
    request: &SetExpirationRequest,
) -> Result<SetExpirationResponse, SetExpirationError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("expiration_ts", request.expiration_ts.to_string())),
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.setExpiration");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(SetExpirationError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetExpirationResponse>(&result)
                .map_err(|e| SetExpirationError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set an existing guest, regular user, or admin user to be a workspace owner.
///
/// Wraps https://api.slack.com/methods/admin.users.setOwner

pub async fn set_owner<R>(
    client: &R,
    token: &str,
    request: &SetOwnerRequest,
) -> Result<SetOwnerResponse, SetOwnerError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.setOwner");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(SetOwnerError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetOwnerResponse>(&result)
                .map_err(|e| SetOwnerError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set an existing guest user, admin user, or owner to be a regular user.
///
/// Wraps https://api.slack.com/methods/admin.users.setRegular

pub async fn set_regular<R>(
    client: &R,
    token: &str,
    request: &SetRegularRequest,
) -> Result<SetRegularResponse, SetRegularError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("team_id", request.team_id.to_string())),
        Some(("user_id", request.user_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.users.setRegular");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(SetRegularError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetRegularResponse>(&result)
                .map_err(|e| SetRegularError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
