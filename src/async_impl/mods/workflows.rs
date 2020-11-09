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
pub use crate::mod_types::workflows_types::*;

/// Indicate that an app's step in a workflow completed execution.
///
/// Wraps https://api.slack.com/methods/workflows.stepCompleted

pub async fn step_completed<R>(
    client: &R,
    token: &str,
    request: &StepCompletedRequest,
) -> Result<StepCompletedResponse, StepCompletedError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .outputs
            .as_ref()
            .map(|outputs| ("outputs", outputs.to_string())),
        Some((
            "workflow_step_execute_id",
            request.workflow_step_execute_id.to_string(),
        )),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/workflows.stepCompleted");
    client
        .get(&url, &params[..])
        .await
        .map_err(StepCompletedError::Client)
        .and_then(|result| {
            serde_json::from_str::<StepCompletedResponse>(&result)
                .map_err(|e| StepCompletedError::MalformedResponse(result, e))
        })
}
/// Indicate that an app's step in a workflow failed to execute.
///
/// Wraps https://api.slack.com/methods/workflows.stepFailed

pub async fn step_failed<R>(
    client: &R,
    token: &str,
    request: &StepFailedRequest,
) -> Result<StepFailedResponse, StepFailedError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("error", request.error.to_string())),
        Some((
            "workflow_step_execute_id",
            request.workflow_step_execute_id.to_string(),
        )),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/workflows.stepFailed");
    client
        .get(&url, &params[..])
        .await
        .map_err(StepFailedError::Client)
        .and_then(|result| {
            serde_json::from_str::<StepFailedResponse>(&result)
                .map_err(|e| StepFailedError::MalformedResponse(result, e))
        })
}
/// Update the configuration for a workflow extension step.
///
/// Wraps https://api.slack.com/methods/workflows.updateStep

pub async fn update_step<R>(
    client: &R,
    token: &str,
    request: &UpdateStepRequest,
) -> Result<UpdateStepResponse, UpdateStepError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .inputs
            .as_ref()
            .map(|inputs| ("inputs", inputs.to_string())),
        request
            .outputs
            .as_ref()
            .map(|outputs| ("outputs", outputs.to_string())),
        request
            .step_image_url
            .as_ref()
            .map(|step_image_url| ("step_image_url", step_image_url.to_string())),
        request
            .step_name
            .as_ref()
            .map(|step_name| ("step_name", step_name.to_string())),
        Some((
            "workflow_step_edit_id",
            request.workflow_step_edit_id.to_string(),
        )),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/workflows.updateStep");
    client
        .get(&url, &params[..])
        .await
        .map_err(UpdateStepError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateStepResponse>(&result)
                .map_err(|e| UpdateStepError::MalformedResponse(result, e))
        })
}
