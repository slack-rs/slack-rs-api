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
pub use crate::mod_types::search_types::*;
use std::borrow::Cow;

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages

pub async fn messages<R>(
    client: &R,
    token: &str,
    request: &MessagesRequest<'_>,
) -> Result<MessagesResponse, MessagesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count: Option<Cow<'_, str>> = request.count.as_ref().map(|count| count.to_string().into());
    let highlight: Option<Cow<'_, str>> = request
        .highlight
        .as_ref()
        .map(|highlight| highlight.to_string().into());
    let page: Option<Cow<'_, str>> = request.page.as_ref().map(|page| page.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        count.as_ref().map(|count| ("count", count.as_ref())),
        highlight
            .as_ref()
            .map(|highlight| ("highlight", highlight.as_ref())),
        page.as_ref().map(|page| ("page", page.as_ref())),
        Some(("query", request.query.as_ref())),
        request.sort.as_ref().map(|sort| ("sort", sort.as_ref())),
        request
            .sort_dir
            .as_ref()
            .map(|sort_dir| ("sort_dir", sort_dir.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/search.messages");
    client
        .get(&url, &params[..])
        .await
        .map_err(MessagesError::Client)
        .and_then(|result| {
            serde_json::from_str::<MessagesResponse>(&result)
                .map_err(|e| MessagesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
