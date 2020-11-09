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

pub use crate::mod_types::reminders_types::*;
use crate::sync::SlackWebRequestSender;

/// Creates a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.add

pub fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("text", request.text.to_string())),
        Some(("time", request.time.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reminders.add");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
}
/// Marks a reminder as complete.
///
/// Wraps https://api.slack.com/methods/reminders.complete

pub fn complete<R>(
    client: &R,
    token: Option<&str>,
    request: &CompleteRequest,
) -> Result<CompleteResponse, CompleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .reminder
        .as_ref()
        .map(|reminder| ("reminder", reminder.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reminders.complete");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .map_err(CompleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<CompleteResponse>(&result)
                .map_err(|e| CompleteError::MalformedResponse(result, e))
        })
}
/// Deletes a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.delete

pub fn delete<R>(
    client: &R,
    token: Option<&str>,
    request: &DeleteRequest,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .reminder
        .as_ref()
        .map(|reminder| ("reminder", reminder.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reminders.delete");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
}
/// Gets information about a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.info

pub fn info<R>(
    client: &R,
    token: Option<&str>,
    request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .reminder
            .as_ref()
            .map(|reminder| ("reminder", reminder.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reminders.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
}
/// Lists all reminders created by or for a given user.
///
/// Wraps https://api.slack.com/methods/reminders.list

pub fn list<R>(
    client: &R,
    token: Option<&str>,
    _request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![token.map(|token| ("token", token.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/reminders.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
}
