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

pub use crate::mod_types::auth_types::*;
use crate::requests::SlackWebRequestSender;

/// Revokes a token.
///
/// Wraps https://api.slack.com/methods/auth.revoke

pub async fn revoke<R>(
    client: &R,
    token: &str,
    request: &RevokeRequest,
) -> Result<RevokeResponse, RevokeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        request
            .test
            .map(|test| ("test", if test { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("auth.revoke");
    client
        .send(&url, &params[..])
        .await
        .map_err(RevokeError::Client)
        .and_then(|result| {
            serde_json::from_str::<RevokeResponse>(&result)
                .map_err(|e| RevokeError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test

pub async fn test<R>(client: &R, token: &str) -> Result<TestResponse, TestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = &[("token", token)];
    let url = crate::get_slack_url_for_method("auth.test");
    client
        .send(&url, &params[..])
        .await
        .map_err(TestError::Client)
        .and_then(|result| {
            serde_json::from_str::<TestResponse>(&result)
                .map_err(|e| TestError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
