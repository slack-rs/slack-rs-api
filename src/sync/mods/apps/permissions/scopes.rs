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

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub use crate::mod_types::apps::permissions::scopes_types::*;
use crate::sync::SlackWebRequestSender;

/// Returns list of scopes this app has on a team.
///
/// Wraps https://api.slack.com/methods/apps.permissions.scopes.list

pub fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/apps.permissions.scopes.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
