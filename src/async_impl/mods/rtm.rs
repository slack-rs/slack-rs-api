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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::rtm_types::*;

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.connect

pub async fn connect<R>(
    client: &R,
    request: &ConnectRequest,
) -> Result<ConnectResponse, ConnectError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .batch_presence_aware
            .as_ref()
            .map(|batch_presence_aware| ("batch_presence_aware", batch_presence_aware.to_string())),
        request
            .presence_sub
            .as_ref()
            .map(|presence_sub| ("presence_sub", presence_sub.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/rtm.connect");
    client
        .get(&url, &params[..])
        .await
        .map_err(ConnectError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConnectResponse>(&result)
                .map_err(|e| ConnectError::MalformedResponse(result, e))
        })
}
