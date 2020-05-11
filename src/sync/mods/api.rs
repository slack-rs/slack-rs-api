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

pub use crate::mod_types::api_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test

pub fn test<R>(client: &R, request: &TestRequest<'_>) -> Result<TestResponse, TestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request.error.map(|error| ("error", error)),
        request.foo.map(|foo| ("foo", foo)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("api.test");
    client
        .send(&url, &params[..])
        .map_err(TestError::Client)
        .and_then(|result| {
            serde_json::from_str::<TestResponse>(&result)
                .map_err(|e| TestError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
