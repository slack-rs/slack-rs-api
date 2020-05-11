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

//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

pub use crate::mod_types::channels_types::*;
use crate::requests::SlackWebRequestSender;
use serde_json;

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive

pub async fn archive<R>(
    client: &R,
    token: &str,
    request: &ArchiveRequest<'_>,
) -> Result<ArchiveResponse, ArchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.archive");
    client
        .send(&url, &params[..])
        .await
        .map_err(ArchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|e| ArchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create

pub async fn create<R>(
    client: &R,
    token: &str,
    request: &CreateRequest<'_>,
) -> Result<CreateResponse, CreateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request
            .validate
            .map(|validate| ("validate", if validate { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.create");
    client
        .send(&url, &params[..])
        .await
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history

pub async fn history<R>(
    client: &R,
    token: &str,
    request: &HistoryRequest<'_>,
) -> Result<HistoryResponse, HistoryError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let latest = request.latest.as_ref().map(|t| t.to_param_value());
    let oldest = request.oldest.as_ref().map(|t| t.to_param_value());
    let count = request.count.map(|count| count.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        latest.as_ref().map(|latest| ("latest", &latest[..])),
        oldest.as_ref().map(|oldest| ("oldest", &oldest[..])),
        request
            .inclusive
            .map(|inclusive| ("inclusive", if inclusive { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        request
            .unreads
            .map(|unreads| ("unreads", if unreads { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.history");
    client
        .send(&url, &params[..])
        .await
        .map_err(HistoryError::Client)
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(|e| HistoryError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info

pub async fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.info");
    client
        .send(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite

pub async fn invite<R>(
    client: &R,
    token: &str,
    request: &InviteRequest<'_>,
) -> Result<InviteResponse, InviteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("user", request.user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.invite");
    client
        .send(&url, &params[..])
        .await
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join

pub async fn join<R>(
    client: &R,
    token: &str,
    request: &JoinRequest<'_>,
) -> Result<JoinResponse, JoinError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("name", request.name)),
        request
            .validate
            .map(|validate| ("validate", if validate { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.join");
    client
        .send(&url, &params[..])
        .await
        .map_err(JoinError::Client)
        .and_then(|result| {
            serde_json::from_str::<JoinResponse>(&result)
                .map_err(|e| JoinError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick

pub async fn kick<R>(
    client: &R,
    token: &str,
    request: &KickRequest<'_>,
) -> Result<KickResponse, KickError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("user", request.user)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.kick");
    client
        .send(&url, &params[..])
        .await
        .map_err(KickError::Client)
        .and_then(|result| {
            serde_json::from_str::<KickResponse>(&result)
                .map_err(|e| KickError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave

pub async fn leave<R>(
    client: &R,
    token: &str,
    request: &LeaveRequest<'_>,
) -> Result<LeaveResponse, LeaveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.leave");
    client
        .send(&url, &params[..])
        .await
        .map_err(LeaveError::Client)
        .and_then(|result| {
            serde_json::from_str::<LeaveResponse>(&result)
                .map_err(|e| LeaveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list

pub async fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request
            .exclude_archived
            .map(|exclude_archived| ("exclude_archived", if exclude_archived { "1" } else { "0" })),
        request
            .exclude_members
            .map(|exclude_members| ("exclude_members", if exclude_members { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.list");
    client
        .send(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/channels.mark

pub async fn mark<R>(
    client: &R,
    token: &str,
    request: &MarkRequest<'_>,
) -> Result<MarkResponse, MarkError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let ts = request.ts.to_param_value();
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("ts", &ts[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.mark");
    client
        .send(&url, &params[..])
        .await
        .map_err(MarkError::Client)
        .and_then(|result| {
            serde_json::from_str::<MarkResponse>(&result)
                .map_err(|e| MarkError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename

pub async fn rename<R>(
    client: &R,
    token: &str,
    request: &RenameRequest<'_>,
) -> Result<RenameResponse, RenameError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("name", request.name)),
        request
            .validate
            .map(|validate| ("validate", if validate { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.rename");
    client
        .send(&url, &params[..])
        .await
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Retrieve a thread of messages posted to a channel
///
/// Wraps https://api.slack.com/methods/channels.replies

pub async fn replies<R>(
    client: &R,
    token: &str,
    request: &RepliesRequest<'_>,
) -> Result<RepliesResponse, RepliesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let thread_ts = request.thread_ts.to_param_value();
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("thread_ts", &thread_ts[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.replies");
    client
        .send(&url, &params[..])
        .await
        .map_err(RepliesError::Client)
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(|e| RepliesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose

pub async fn set_purpose<R>(
    client: &R,
    token: &str,
    request: &SetPurposeRequest<'_>,
) -> Result<SetPurposeResponse, SetPurposeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("purpose", request.purpose)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.setPurpose");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetPurposeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result)
                .map_err(|e| SetPurposeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic

pub async fn set_topic<R>(
    client: &R,
    token: &str,
    request: &SetTopicRequest<'_>,
) -> Result<SetTopicResponse, SetTopicError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("topic", request.topic)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.setTopic");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetTopicError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result)
                .map_err(|e| SetTopicError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive

pub async fn unarchive<R>(
    client: &R,
    token: &str,
    request: &UnarchiveRequest<'_>,
) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("channels.unarchive");
    client
        .send(&url, &params[..])
        .await
        .map_err(UnarchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result)
                .map_err(|e| UnarchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
