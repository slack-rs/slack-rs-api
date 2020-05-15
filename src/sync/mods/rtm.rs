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

pub use crate::mod_types::rtm_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.connect

pub fn connect<R>(client: &R, token: &str) -> Result<ConnectResponse, ConnectError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("rtm.connect");
    client
        .send(&url, &params[..])
        .map_err(ConnectError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConnectResponse>(&result)
                .map_err(|e| ConnectError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.start

pub fn start<R>(
    client: &R,
    token: &str,
    request: &StartRequest,
) -> Result<StartResponse, StartError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request
            .no_unreads
            .map(|no_unreads| ("no_unreads", if no_unreads { "1" } else { "0" })),
        request
            .mpim_aware
            .map(|mpim_aware| ("mpim_aware", if mpim_aware { "1" } else { "0" })),
        request
            .no_latest
            .map(|no_latest| ("no_latest", if no_latest { "1" } else { "0" })),
        request.batch_presence_aware.map(|batch_presence_aware| {
            (
                "batch_presence_aware",
                if batch_presence_aware { "1" } else { "0" },
            )
        }),
        request
            .include_locale
            .map(|include_locale| ("include_locale", if include_locale { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("rtm.start");
    client
        .send(&url, &params[..])
        .map_err(StartError::Client)
        .and_then(|result| {
            serde_json::from_str::<StartResponse>(&result)
                .map_err(|e| StartError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
