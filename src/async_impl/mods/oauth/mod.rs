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

pub mod v_2;

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::oauth::*;
use std::borrow::Cow;

/// Exchanges a temporary OAuth verifier code for an access token.
///
/// Wraps https://api.slack.com/methods/oauth.access

pub async fn access<R>(
    client: &R,
    request: &AccessRequest<'_>,
) -> Result<AccessResponse, AccessError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let single_channel: Option<Cow<'_, str>> = request
        .single_channel
        .as_ref()
        .map(|single_channel| single_channel.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .client_id
            .as_ref()
            .map(|client_id| ("client_id", client_id.as_ref())),
        request
            .client_secret
            .as_ref()
            .map(|client_secret| ("client_secret", client_secret.as_ref())),
        request.code.as_ref().map(|code| ("code", code.as_ref())),
        request
            .redirect_uri
            .as_ref()
            .map(|redirect_uri| ("redirect_uri", redirect_uri.as_ref())),
        single_channel
            .as_ref()
            .map(|single_channel| ("single_channel", single_channel.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/oauth.access");
    client
        .get(&url, &params[..])
        .await
        .map_err(AccessError::Client)
        .and_then(|result| {
            serde_json::from_str::<AccessResponse>(&result)
                .map_err(|e| AccessError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Exchanges a temporary OAuth verifier code for a workspace token.
///
/// Wraps https://api.slack.com/methods/oauth.token

pub async fn token<R>(
    client: &R,
    request: &TokenRequest<'_>,
) -> Result<TokenResponse, TokenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let single_channel: Option<Cow<'_, str>> = request
        .single_channel
        .as_ref()
        .map(|single_channel| single_channel.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .client_id
            .as_ref()
            .map(|client_id| ("client_id", client_id.as_ref())),
        request
            .client_secret
            .as_ref()
            .map(|client_secret| ("client_secret", client_secret.as_ref())),
        request.code.as_ref().map(|code| ("code", code.as_ref())),
        request
            .redirect_uri
            .as_ref()
            .map(|redirect_uri| ("redirect_uri", redirect_uri.as_ref())),
        single_channel
            .as_ref()
            .map(|single_channel| ("single_channel", single_channel.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/oauth.token");
    client
        .get(&url, &params[..])
        .await
        .map_err(TokenError::Client)
        .and_then(|result| {
            serde_json::from_str::<TokenResponse>(&result)
                .map_err(|e| TokenError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
