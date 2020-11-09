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

pub use crate::mod_types::admin::conversations::*;
use crate::sync::SlackWebRequestSender;

/// Archive a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.archive

pub fn archive<R>(
    client: &R,
    token: &str,
    request: &ArchiveRequest,
) -> Result<ArchiveResponse, ArchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("channel_id", request.channel_id.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.archive");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(ArchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|e| ArchiveError::MalformedResponse(result, e))
        })
}
/// Convert a public channel to a private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.convertToPrivate

pub fn convert_to_private<R>(
    client: &R,
    token: &str,
    request: &ConvertToPrivateRequest,
) -> Result<ConvertToPrivateResponse, ConvertToPrivateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("channel_id", request.channel_id.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.convertToPrivate");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(ConvertToPrivateError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConvertToPrivateResponse>(&result)
                .map_err(|e| ConvertToPrivateError::MalformedResponse(result, e))
        })
}
/// Create a public or private channel-based conversation.
///
/// Wraps https://api.slack.com/methods/admin.conversations.create

pub fn create<R>(
    client: &R,
    token: &str,
    request: &CreateRequest,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .description
            .as_ref()
            .map(|description| ("description", description.to_string())),
        Some(("is_private", request.is_private.to_string())),
        Some(("name", request.name.to_string())),
        request
            .org_wide
            .as_ref()
            .map(|org_wide| ("org_wide", org_wide.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.create");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
}
/// Delete a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.delete

pub fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("channel_id", request.channel_id.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.delete");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
}
/// Disconnect a connected channel from one or more workspaces.
///
/// Wraps https://api.slack.com/methods/admin.conversations.disconnectShared

pub fn disconnect_shared<R>(
    client: &R,
    token: &str,
    request: &DisconnectSharedRequest,
) -> Result<DisconnectSharedResponse, DisconnectSharedError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        request
            .leaving_team_ids
            .as_ref()
            .map(|leaving_team_ids| ("leaving_team_ids", leaving_team_ids.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.disconnectShared");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(DisconnectSharedError::Client)
        .and_then(|result| {
            serde_json::from_str::<DisconnectSharedResponse>(&result)
                .map_err(|e| DisconnectSharedError::MalformedResponse(result, e))
        })
}
/// Get conversation preferences for a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.getConversationPrefs

pub fn get_conversation_prefs<R>(
    client: &R,
    token: &str,
    request: &GetConversationPrefsRequest,
) -> Result<GetConversationPrefsResponse, GetConversationPrefsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("channel_id", request.channel_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.getConversationPrefs");
    client
        .get(&url, &params[..])
        .map_err(GetConversationPrefsError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetConversationPrefsResponse>(&result)
                .map_err(|e| GetConversationPrefsError::MalformedResponse(result, e))
        })
}
/// Get all the workspaces a given public or private channel is connected to within this Enterprise org.
///
/// Wraps https://api.slack.com/methods/admin.conversations.getTeams

pub fn get_teams<R>(
    client: &R,
    token: &str,
    request: &GetTeamsRequest,
) -> Result<GetTeamsResponse, GetTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("channel_id", request.channel_id.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.getTeams");
    client
        .get(&url, &params[..])
        .map_err(GetTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetTeamsResponse>(&result)
                .map_err(|e| GetTeamsError::MalformedResponse(result, e))
        })
}
/// Invite a user to a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.invite

pub fn invite<R>(
    client: &R,
    token: &str,
    request: &InviteRequest,
) -> Result<InviteResponse, InviteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        Some(("user_ids", request.user_ids.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.invite");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
}
/// Rename a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.rename

pub fn rename<R>(
    client: &R,
    token: &str,
    request: &RenameRequest,
) -> Result<RenameResponse, RenameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        Some(("name", request.name.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.rename");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
}
/// Search for public or private channels in an Enterprise organization.
///
/// Wraps https://api.slack.com/methods/admin.conversations.search

pub fn search<R>(
    client: &R,
    token: &str,
    request: &SearchRequest,
) -> Result<SearchResponse, SearchError<R::Error>>
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
        request
            .query
            .as_ref()
            .map(|query| ("query", query.to_string())),
        request
            .search_channel_types
            .as_ref()
            .map(|search_channel_types| ("search_channel_types", search_channel_types.to_string())),
        request.sort.as_ref().map(|sort| ("sort", sort.to_string())),
        request
            .sort_dir
            .as_ref()
            .map(|sort_dir| ("sort_dir", sort_dir.to_string())),
        request
            .team_ids
            .as_ref()
            .map(|team_ids| ("team_ids", team_ids.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.search");
    client
        .get(&url, &params[..])
        .map_err(SearchError::Client)
        .and_then(|result| {
            serde_json::from_str::<SearchResponse>(&result)
                .map_err(|e| SearchError::MalformedResponse(result, e))
        })
}
/// Set the posting permissions for a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.setConversationPrefs

pub fn set_conversation_prefs<R>(
    client: &R,
    token: &str,
    request: &SetConversationPrefsRequest,
) -> Result<SetConversationPrefsResponse, SetConversationPrefsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        Some(("prefs", request.prefs.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.setConversationPrefs");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetConversationPrefsError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetConversationPrefsResponse>(&result)
                .map_err(|e| SetConversationPrefsError::MalformedResponse(result, e))
        })
}
/// Set the workspaces in an Enterprise grid org that connect to a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.setTeams

pub fn set_teams<R>(
    client: &R,
    token: &str,
    request: &SetTeamsRequest,
) -> Result<SetTeamsResponse, SetTeamsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel_id", request.channel_id.to_string())),
        request
            .org_channel
            .as_ref()
            .map(|org_channel| ("org_channel", org_channel.to_string())),
        request
            .target_team_ids
            .as_ref()
            .map(|target_team_ids| ("target_team_ids", target_team_ids.to_string())),
        request
            .team_id
            .as_ref()
            .map(|team_id| ("team_id", team_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.setTeams");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetTeamsError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetTeamsResponse>(&result)
                .map_err(|e| SetTeamsError::MalformedResponse(result, e))
        })
}
/// Unarchive a public or private channel.
///
/// Wraps https://api.slack.com/methods/admin.conversations.unarchive

pub fn unarchive<R>(
    client: &R,
    token: &str,
    request: &UnarchiveRequest,
) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("channel_id", request.channel_id.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/admin.conversations.unarchive");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(UnarchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result)
                .map_err(|e| UnarchiveError::MalformedResponse(result, e))
        })
}
