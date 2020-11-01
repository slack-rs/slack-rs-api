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
pub use crate::mod_types::dialog_types::*;

/// Open a dialog with a user
///
/// Wraps https://api.slack.com/methods/dialog.open

pub async fn open<R>(client: &R, request: &OpenRequest) -> Result<OpenResponse, OpenError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("dialog", request.dialog.to_string())),
        Some(("trigger_id", request.trigger_id.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/dialog.open");
    client
        .get(&url, &params[..])
        .await
        .map_err(OpenError::Client)
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result)
                .map_err(|e| OpenError::MalformedResponse(result, e))
        })
}
