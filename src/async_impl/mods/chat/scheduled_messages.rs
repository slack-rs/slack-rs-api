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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::chat::scheduled_messages_types::*;
use std::borrow::Cow;

/// Returns a list of scheduled messages.
///
/// Wraps https://api.slack.com/methods/chat.scheduledMessages.list

pub async fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let latest: Option<Cow<'_, str>> = request
        .latest
        .as_ref()
        .map(|latest| latest.to_string().into());
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let oldest: Option<Cow<'_, str>> = request
        .oldest
        .as_ref()
        .map(|oldest| oldest.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        latest.as_ref().map(|latest| ("latest", latest.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        oldest.as_ref().map(|oldest| ("oldest", oldest.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.scheduledMessages.list");
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
