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

pub use crate::mod_types::team_profile_types::*;
use crate::requests::SlackWebRequestSender;

/// Retrieve a team's profile.
///
/// Wraps https://api.slack.com/methods/team.profile.get

pub async fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest<'_>,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request
            .visibility
            .map(|visibility| ("visibility", visibility)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("team.profile.get");
    client
        .send(&url, &params[..])
        .await
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
