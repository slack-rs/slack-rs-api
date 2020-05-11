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

//! Get info on your multiparty direct messages.

pub use crate::mod_types::mpim_types::*;
use crate::requests::SlackWebRequestSender;

/// Closes a multiparty direct message channel.
///
/// Wraps https://api.slack.com/methods/mpim.close

pub async fn close<R>(
    client: &R,
    token: &str,
    request: &CloseRequest<'_>,
) -> Result<CloseResponse, CloseError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("channel", request.channel))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("mpim.close");
    client
        .send(&url, &params[..])
        .await
        .map_err(CloseError::Client)
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result)
                .map_err(|e| CloseError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Fetches history of messages and events from a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.history

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
    let url = crate::get_slack_url_for_method("mpim.history");
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

/// Lists multiparty direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/mpim.list

pub async fn list<R>(client: &R, token: &str) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("mpim.list");
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

/// Sets the read cursor in a multiparty direct message channel.
///
/// Wraps https://api.slack.com/methods/mpim.mark

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
    let url = crate::get_slack_url_for_method("mpim.mark");
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

/// This method opens a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.open

pub async fn open<R>(
    client: &R,
    token: &str,
    request: &OpenRequest<'_>,
) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("token", token)), Some(("users", request.users))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("mpim.open");
    client
        .send(&url, &params[..])
        .await
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Retrieve a thread of messages posted to a direct message conversation from a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.replies

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
    let url = crate::get_slack_url_for_method("mpim.replies");
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
