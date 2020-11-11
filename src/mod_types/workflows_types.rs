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

use std::borrow::Cow;
use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct StepCompletedRequest<'a> {
    /// Key-value object of outputs from your step. Keys of this object reflect the configured `key` properties of your [`outputs`](/reference/workflows/workflow_step#output) array from your `workflow_step` object.
    pub outputs: Option<Cow<'a, str>>,
    /// Context identifier that maps to the correct workflow step execution.
    pub workflow_step_execute_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StepCompletedResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<StepCompletedResponse, StepCompletedError<E>>>
    for StepCompletedResponse
{
    fn into(self) -> Result<StepCompletedResponse, StepCompletedError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(StepCompletedError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum StepCompletedError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for StepCompletedError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => StepCompletedError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for StepCompletedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StepCompletedError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            StepCompletedError::Unknown(ref s) => write!(f, "{}", s),
            StepCompletedError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for StepCompletedError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            StepCompletedError::MalformedResponse(_, ref e) => Some(e),
            StepCompletedError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct StepFailedRequest<'a> {
    /// A JSON-based object with a `message` property that should contain a human readable error message.
    pub error: Cow<'a, str>,
    /// Context identifier that maps to the correct workflow step execution.
    pub workflow_step_execute_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StepFailedResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<StepFailedResponse, StepFailedError<E>>> for StepFailedResponse {
    fn into(self) -> Result<StepFailedResponse, StepFailedError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(StepFailedError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum StepFailedError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for StepFailedError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => StepFailedError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for StepFailedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StepFailedError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            StepFailedError::Unknown(ref s) => write!(f, "{}", s),
            StepFailedError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for StepFailedError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            StepFailedError::MalformedResponse(_, ref e) => Some(e),
            StepFailedError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct UpdateStepRequest<'a> {
    /// A JSON key-value map of inputs required from a user during configuration. This is the data your app expects to receive when the workflow step starts. **Please note**: the embedded variable format is set and replaced by the workflow system. You cannot create custom variables that will be replaced at runtime. [Read more about variables in workflow steps here](/workflows/steps#variables).
    pub inputs: Option<Cow<'a, str>>,
    /// An JSON array of output objects used during step execution. This is the data your app agrees to provide when your workflow step was executed.
    pub outputs: Option<Cow<'a, str>>,
    /// An optional field that can be used to override app image that is shown in the Workflow Builder.
    pub step_image_url: Option<Cow<'a, str>>,
    /// An optional field that can be used to override the step name that is shown in the Workflow Builder.
    pub step_name: Option<Cow<'a, str>>,
    /// A context identifier provided with `view_submission` payloads used to call back to `workflows.updateStep`.
    pub workflow_step_edit_id: Cow<'a, str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateStepResponse {
    #[serde(default)]
    ok: bool,
}

impl<E: Error> Into<Result<UpdateStepResponse, UpdateStepError<E>>> for UpdateStepResponse {
    fn into(self) -> Result<UpdateStepResponse, UpdateStepError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(UpdateStepError::Unknown(
                "Server failed without providing an error message.".into(),
            ))
        }
    }
}

#[derive(Debug)]
pub enum UpdateStepError<E: Error> {
    /// The response was not parseable as the expected object
    MalformedResponse(String, serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for UpdateStepError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            _ => UpdateStepError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for UpdateStepError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UpdateStepError::MalformedResponse(_, ref e) => write!(f, "{}", e),
            UpdateStepError::Unknown(ref s) => write!(f, "{}", s),
            UpdateStepError::Client(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl<E: Error + 'static> Error for UpdateStepError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UpdateStepError::MalformedResponse(_, ref e) => Some(e),
            UpdateStepError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
