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
pub use crate::mod_types::team::profile_types::*;

/// Retrieve a team's profile.
///
/// Wraps https://api.slack.com/methods/team.profile.get

pub async fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .visibility
            .as_ref()
            .map(|visibility| ("visibility", visibility.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/team.profile.get");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
}
