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

pub use crate::mod_types::files_comments_types::*;
use crate::sync::requests::SlackWebRequestSender;
use serde_json;

/// Add a comment to an existing file.
///
/// Wraps https://api.slack.com/methods/files.comments.add

pub fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("file", request.file)),
        Some(("comment", request.comment)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.comments.add");
    client
        .send(&url, &params[..])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Deletes an existing comment on a file.
///
/// Wraps https://api.slack.com/methods/files.comments.delete

pub fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest<'_>,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("file", request.file)),
        Some(("id", request.id)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.comments.delete");
    client
        .send(&url, &params[..])
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Edit an existing file comment.
///
/// Wraps https://api.slack.com/methods/files.comments.edit

pub fn edit<R>(
    client: &R,
    token: &str,
    request: &EditRequest<'_>,
) -> Result<EditResponse, EditError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("file", request.file)),
        Some(("id", request.id)),
        Some(("comment", request.comment)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("files.comments.edit");
    client
        .send(&url, &params[..])
        .map_err(EditError::Client)
        .and_then(|result| {
            serde_json::from_str::<EditResponse>(&result)
                .map_err(|e| EditError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
