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
pub use crate::mod_types::api_types::*;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test

pub async fn test<R>(client: &R, request: &TestRequest) -> Result<TestResponse, TestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .error
            .as_ref()
            .map(|error| ("error", error.to_string())),
        request.foo.as_ref().map(|foo| ("foo", foo.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/api.test");
    client
        .get(&url, &params[..])
        .await
        .map_err(TestError::Client)
        .and_then(|result| {
            serde_json::from_str::<TestResponse>(&result)
                .map_err(|e| TestError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
