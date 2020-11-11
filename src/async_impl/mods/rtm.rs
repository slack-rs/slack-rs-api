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
pub use crate::mod_types::rtm_types::*;
use std::borrow::Cow;

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.connect

pub async fn connect<R>(
    client: &R,
    token: &str,
    request: &ConnectRequest,
) -> Result<ConnectResponse, ConnectError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let batch_presence_aware: Option<Cow<'_, str>> = request
        .batch_presence_aware
        .as_ref()
        .map(|batch_presence_aware| batch_presence_aware.to_string().into());
    let presence_sub: Option<Cow<'_, str>> = request
        .presence_sub
        .as_ref()
        .map(|presence_sub| presence_sub.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        batch_presence_aware
            .as_ref()
            .map(|batch_presence_aware| ("batch_presence_aware", batch_presence_aware.as_ref())),
        presence_sub
            .as_ref()
            .map(|presence_sub| ("presence_sub", presence_sub.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/rtm.connect");
    client
        .get(&url, &params[..])
        .await
        .map_err(ConnectError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConnectResponse>(&result)
                .map_err(|e| ConnectError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
