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

//! Get info on your direct messages.

pub use crate::mod_types::im_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Close a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.close

pub fn close<R>(
    client: &R,
    token: &str,
    request: &CloseRequest<'_>,
) -> Result<CloseResponse, CloseError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.close");
    client
        .send(&url, &params[..])
        .map_err(CloseError::Client)
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result)
                .map_err(|e| CloseError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Fetches history of messages and events from direct message channel.
///
/// Wraps https://api.slack.com/methods/im.history

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
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.history");
    client
        .send(&url, &params[..])
        .map_err(HistoryError::Client)
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result)
                .map_err(|e| HistoryError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/im.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit = request.limit.map(|limit| limit.to_string());
    let params = vec![
        Some(("token", token)),
        request.cursor.map(|cursor| ("cursor", cursor)),
        limit.as_ref().map(|limit| ("limit", &limit[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sets the read cursor in a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.mark

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
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.mark");
    client
        .send(&url, &params[..])
        .map_err(MarkError::Client)
        .and_then(|result| {
            serde_json::from_str::<MarkResponse>(&result)
                .map_err(|e| MarkError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Opens a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.open

pub fn open<R>(
    client: &R,
    token: &str,
    request: &OpenRequest<'_>,
) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("user", request.user)),
        request
            .return_im
            .map(|return_im| ("return_im", if return_im { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.open");
    client
        .send(&url, &params[..])
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Retrieve a thread of messages posted to a direct message conversation
///
/// Wraps https://api.slack.com/methods/im.replies

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
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("im.replies");
    client
        .send(&url, &params[..])
        .map_err(RepliesError::Client)
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result)
                .map_err(|e| RepliesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
