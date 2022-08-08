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

pub use crate::mod_types::oauth_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Exchanges a temporary OAuth code for an API token.
///
/// Wraps https://api.slack.com/methods/oauth.access

pub fn access<R>(
    client: &R,
    request: &AccessRequest<'_>,
) -> Result<AccessResponse, AccessError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("client_id", request.client_id)),
        Some(("client_secret", request.client_secret)),
        Some(("code", request.code)),
        request
            .redirect_uri
            .map(|redirect_uri| ("redirect_uri", redirect_uri)),
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("oauth.access");
    client
        .send(&url, &params[..])
        .map_err(AccessError::Client)
        .and_then(|result| {
            serde_json::from_str::<AccessResponse>(&result)
                .map_err(|e| AccessError::MalformedResponse(result, e))
        })
}
