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

pub mod ekm;
pub mod restrict_access;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::admin::conversations::*;
use std::borrow::Cow;

/// Archive a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.archive

pub async fn archive<R>(
    client: &R,
    token: &str,
    request: &ArchiveRequest<'_>,
) -> Result<ArchiveResponse, ArchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![Some(("channel_id", request.channel_id.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.archive");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(ArchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|e| ArchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Convert a public channel to a private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.convertToPrivate

pub async fn convert_to_private<R>(
    client: &R,
    token: &str,
    request: &ConvertToPrivateRequest<'_>,
) -> Result<ConvertToPrivateResponse, ConvertToPrivateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![Some(("channel_id", request.channel_id.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.convertToPrivate");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(ConvertToPrivateError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConvertToPrivateResponse>(&result)
                .map_err(|e| ConvertToPrivateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Create a public or private channel-based conversation.
///
/// Wraps https://api.slack.com/methods/admin.conversations.create

pub async fn create<R>(
    client: &R,
    token: &str,
    request: &CreateRequest<'_>,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let is_private: Option<Cow<'_, str>> = Some(request.is_private.to_string().into());
    let org_wide: Option<Cow<'_, str>> = request
        .org_wide
        .as_ref()
        .map(|org_wide| org_wide.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .description
            .as_ref()
            .map(|description| ("description", description.as_ref())),
        is_private
            .as_ref()
            .map(|is_private| ("is_private", is_private.as_ref())),
        Some(("name", request.name.as_ref())),
        org_wide
            .as_ref()
            .map(|org_wide| ("org_wide", org_wide.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.create");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Delete a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.delete

pub async fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest<'_>,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![Some(("channel_id", request.channel_id.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.delete");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Disconnect a connected channel from one or more workspaces.
///
/// Wraps https://api.slack.com/methods/admin.conversations.disconnectShared

pub async fn disconnect_shared<R>(
    client: &R,
    token: &str,
    request: &DisconnectSharedRequest<'_>,
) -> Result<DisconnectSharedResponse, DisconnectSharedError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        request
            .leaving_team_ids
            .as_ref()
            .map(|leaving_team_ids| ("leaving_team_ids", leaving_team_ids.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.disconnectShared");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(DisconnectSharedError::Client)
        .and_then(|result| {
            serde_json::from_str::<DisconnectSharedResponse>(&result)
                .map_err(|e| DisconnectSharedError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Get conversation preferences for a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.getConversationPrefs

pub async fn get_conversation_prefs<R>(
    client: &R,
    token: &str,
    request: &GetConversationPrefsRequest<'_>,
) -> Result<GetConversationPrefsResponse, GetConversationPrefsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("channel_id", request.channel_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.getConversationPrefs");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetConversationPrefsError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetConversationPrefsResponse>(&result)
                .map_err(|e| GetConversationPrefsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Get all the workspaces a given public or private channel is connected to within this Enterprise org.
///
/// Wraps https://api.slack.com/methods/admin.conversations.getTeams

pub async fn get_teams<R>(
    client: &R,
    token: &str,
    request: &GetTeamsRequest<'_>,
) -> Result<GetTeamsResponse, GetTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("channel_id", request.channel_id.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.getTeams");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetTeamsResponse>(&result)
                .map_err(|e| GetTeamsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Invite a user to a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.invite

pub async fn invite<R>(
    client: &R,
    token: &str,
    request: &InviteRequest<'_>,
) -> Result<InviteResponse, InviteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        Some(("user_ids", request.user_ids.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.invite");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Rename a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.rename

pub async fn rename<R>(
    client: &R,
    token: &str,
    request: &RenameRequest<'_>,
) -> Result<RenameResponse, RenameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        Some(("name", request.name.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.rename");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Search for public or private channels in an Enterprise organization.
///
/// Wraps https://api.slack.com/methods/admin.conversations.search

pub async fn search<R>(
    client: &R,
    token: &str,
    request: &SearchRequest<'_>,
) -> Result<SearchResponse, SearchError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        request
            .query
            .as_ref()
            .map(|query| ("query", query.as_ref())),
        request
            .search_channel_types
            .as_ref()
            .map(|search_channel_types| ("search_channel_types", search_channel_types.as_ref())),
        request.sort.as_ref().map(|sort| ("sort", sort.as_ref())),
        request
            .sort_dir
            .as_ref()
            .map(|sort_dir| ("sort_dir", sort_dir.as_ref())),
        request
            .team_ids
            .as_ref()
            .map(|team_ids| ("team_ids", team_ids.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.search");
    client
        .get(&url, &params[..])
        .await
        .map_err(SearchError::Client)
        .and_then(|result| {
            serde_json::from_str::<SearchResponse>(&result)
                .map_err(|e| SearchError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the posting permissions for a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.setConversationPrefs

pub async fn set_conversation_prefs<R>(
    client: &R,
    token: &str,
    request: &SetConversationPrefsRequest<'_>,
) -> Result<SetConversationPrefsResponse, SetConversationPrefsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        Some(("prefs", request.prefs.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.setConversationPrefs");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(SetConversationPrefsError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetConversationPrefsResponse>(&result)
                .map_err(|e| SetConversationPrefsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the workspaces in an Enterprise grid org that connect to a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.setTeams

pub async fn set_teams<R>(
    client: &R,
    token: &str,
    request: &SetTeamsRequest<'_>,
) -> Result<SetTeamsResponse, SetTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let org_channel: Option<Cow<'_, str>> = request
        .org_channel
        .as_ref()
        .map(|org_channel| org_channel.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel_id", request.channel_id.as_ref())),
        org_channel
            .as_ref()
            .map(|org_channel| ("org_channel", org_channel.as_ref())),
        request
            .target_team_ids
            .as_ref()
            .map(|target_team_ids| ("target_team_ids", target_team_ids.as_ref())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.setTeams");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(SetTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetTeamsResponse>(&result)
                .map_err(|e| SetTeamsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Unarchive a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.unarchive

pub async fn unarchive<R>(
    client: &R,
    token: &str,
    request: &UnarchiveRequest<'_>,
) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![Some(("channel_id", request.channel_id.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.unarchive");
    client
        .post(&url, &params[..], &[("token", token)])
        .await
        .map_err(UnarchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result)
                .map_err(|e| UnarchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
