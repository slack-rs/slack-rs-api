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

//! Get info on your team's private channels.

pub use crate::mod_types::groups_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Archives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.archive

pub fn archive<R>(
    client: &R,
    token: &str,
    request: &ArchiveRequest<'_>,
) -> Result<ArchiveResponse, ArchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.archive");
    client
        .send(&url, &params[..])
        .map_err(ArchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<ArchiveResponse>(&result)
                .map_err(|e| ArchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Closes a private channel.
///
/// Wraps https://api.slack.com/methods/groups.close

pub fn close<R>(
    client: &R,
    token: &str,
    request: &CloseRequest<'_>,
) -> Result<CloseResponse, CloseError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.close");
    client
        .send(&url, &params[..])
        .map_err(CloseError::Client)
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result)
                .map_err(|e| CloseError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Creates a private channel.
///
/// Wraps https://api.slack.com/methods/groups.create

pub fn create<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.create");
    client
        .send(&url, &params[..])
        .map_err(CreateError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result)
                .map_err(|e| CreateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Clones and archives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.createChild

pub fn create_child<R>(
    client: &R,
    token: &str,
    request: &CreateChildRequest<'_>,
) -> Result<CreateChildResponse, CreateChildError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.createChild");
    client
        .send(&url, &params[..])
        .map_err(CreateChildError::Client)
        .and_then(|result| {
            serde_json::from_str::<CreateChildResponse>(&result)
                .map_err(|e| CreateChildError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Fetches history of messages and events from a private channel.
///
/// Wraps https://api.slack.com/methods/groups.history

pub fn history<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.history");
    client
        .send(&url, &params[..])
        .map_err(HistoryError::Client)
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(|e| HistoryError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Gets information about a private channel.
///
/// Wraps https://api.slack.com/methods/groups.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.info");
    client
        .send(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Invites a user to a private channel.
///
/// Wraps https://api.slack.com/methods/groups.invite

pub fn invite<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.invite");
    client
        .send(&url, &params[..])
        .map_err(InviteError::Client)
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result)
                .map_err(|e| InviteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Removes a user from a private channel.
///
/// Wraps https://api.slack.com/methods/groups.kick

pub fn kick<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.kick");
    client
        .send(&url, &params[..])
        .map_err(KickError::Client)
        .and_then(|result| {
            serde_json::from_str::<KickResponse>(&result)
                .map_err(|e| KickError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Leaves a private channel.
///
/// Wraps https://api.slack.com/methods/groups.leave

pub fn leave<R>(
    client: &R,
    token: &str,
    request: &LeaveRequest<'_>,
) -> Result<LeaveResponse, LeaveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.leave");
    client
        .send(&url, &params[..])
        .map_err(LeaveError::Client)
        .and_then(|result| {
            serde_json::from_str::<LeaveResponse>(&result)
                .map_err(|e| LeaveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists private channels that the calling user has access to.
///
/// Wraps https://api.slack.com/methods/groups.list

pub fn list<R>(
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
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the read cursor in a private channel.
///
/// Wraps https://api.slack.com/methods/groups.mark

pub fn mark<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.mark");
    client
        .send(&url, &params[..])
        .map_err(MarkError::Client)
        .and_then(|result| {
            serde_json::from_str::<MarkResponse>(&result)
                .map_err(|e| MarkError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Opens a private channel.
///
/// Wraps https://api.slack.com/methods/groups.open

pub fn open<R>(
    client: &R,
    token: &str,
    request: &OpenRequest<'_>,
) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.open");
    client
        .send(&url, &params[..])
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Renames a private channel.
///
/// Wraps https://api.slack.com/methods/groups.rename

pub fn rename<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.rename");
    client
        .send(&url, &params[..])
        .map_err(RenameError::Client)
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result)
                .map_err(|e| RenameError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Retrieve a thread of messages posted to a private channel
///
/// Wraps https://api.slack.com/methods/groups.replies

pub fn replies<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.replies");
    client
        .send(&url, &params[..])
        .map_err(RepliesError::Client)
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(|e| RepliesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the purpose for a private channel.
///
/// Wraps https://api.slack.com/methods/groups.setPurpose

pub fn set_purpose<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.setPurpose");
    client
        .send(&url, &params[..])
        .map_err(SetPurposeError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result)
                .map_err(|e| SetPurposeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the topic for a private channel.
///
/// Wraps https://api.slack.com/methods/groups.setTopic

pub fn set_topic<R>(
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
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.setTopic");
    client
        .send(&url, &params[..])
        .map_err(SetTopicError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result)
                .map_err(|e| SetTopicError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Unarchives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.unarchive

pub fn unarchive<R>(
    client: &R,
    token: &str,
    request: &UnarchiveRequest<'_>,
) -> Result<UnarchiveResponse, UnarchiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("groups.unarchive");
    client
        .send(&url, &params[..])
        .map_err(UnarchiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnarchiveResponse>(&result)
                .map_err(|e| UnarchiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
