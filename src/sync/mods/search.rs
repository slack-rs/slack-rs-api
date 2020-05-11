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

//! Search your team's files and messages.

pub use crate::mod_types::search_types::*;
use crate::sync::requests::SlackWebRequestSender;
use serde_json;

/// Searches for messages and files matching a query.
///
/// Wraps https://api.slack.com/methods/search.all

pub fn all<R>(
    client: &R,
    token: &str,
    request: &AllRequest<'_>,
) -> Result<AllResponse, AllError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request
            .highlight
            .map(|highlight| ("highlight", if highlight { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("search.all");
    client
        .send(&url, &params[..])
        .map_err(AllError::Client)
        .and_then(|result| {
            serde_json::from_str::<AllResponse>(&result)
                .map_err(|e| AllError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Searches for files matching a query.
///
/// Wraps https://api.slack.com/methods/search.files

pub fn files<R>(
    client: &R,
    token: &str,
    request: &FilesRequest<'_>,
) -> Result<FilesResponse, FilesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request
            .highlight
            .map(|highlight| ("highlight", if highlight { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("search.files");
    client
        .send(&url, &params[..])
        .map_err(FilesError::Client)
        .and_then(|result| {
            serde_json::from_str::<FilesResponse>(&result)
                .map_err(|e| FilesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages

pub fn messages<R>(
    client: &R,
    token: &str,
    request: &MessagesRequest<'_>,
) -> Result<MessagesResponse, MessagesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        Some(("query", request.query)),
        request.sort.map(|sort| ("sort", sort)),
        request.sort_dir.map(|sort_dir| ("sort_dir", sort_dir)),
        request
            .highlight
            .map(|highlight| ("highlight", if highlight { "1" } else { "0" })),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("search.messages");
    client
        .send(&url, &params[..])
        .map_err(MessagesError::Client)
        .and_then(|result| {
            serde_json::from_str::<MessagesResponse>(&result)
                .map_err(|e| MessagesError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
