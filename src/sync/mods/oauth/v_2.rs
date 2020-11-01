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

pub use crate::mod_types::oauth::v_2_types::*;
use crate::sync::SlackWebRequestSender;

/// Exchanges a temporary OAuth verifier code for an access token.
///
/// Wraps https://api.slack.com/methods/oauth.v2.access

pub fn access<R>(
    client: &R,
    request: &AccessRequest,
) -> Result<AccessResponse, AccessError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .client_id
            .as_ref()
            .map(|client_id| ("client_id", client_id.to_string())),
        request
            .client_secret
            .as_ref()
            .map(|client_secret| ("client_secret", client_secret.to_string())),
        Some(("code", request.code.to_string())),
        request
            .redirect_uri
            .as_ref()
            .map(|redirect_uri| ("redirect_uri", redirect_uri.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/oauth.v2.access");
    client
        .get(&url, &params[..])
        .map_err(AccessError::Client)
        .and_then(|result| {
            serde_json::from_str::<AccessResponse>(&result)
                .map_err(|e| AccessError::MalformedResponse(result, e))
        })
}
