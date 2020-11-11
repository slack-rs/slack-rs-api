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

pub use crate::mod_types::users::profile_types::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

pub fn get<R>(
    client: &R,
    token: &str,
    request: &GetRequest<'_>,
) -> Result<GetResponse, GetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_labels: Option<Cow<'_, str>> = request
        .include_labels
        .as_ref()
        .map(|include_labels| include_labels.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        include_labels
            .as_ref()
            .map(|include_labels| ("include_labels", include_labels.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.profile.get");
    client
        .get(&url, &params[..])
        .map_err(GetError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result)
                .map_err(|e| GetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the profile information for a user.
///
/// Wraps https://api.slack.com/methods/users.profile.set

pub fn set<R>(
    client: &R,
    token: &str,
    request: &SetRequest<'_>,
) -> Result<SetResponse, SetError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request.name.as_ref().map(|name| ("name", name.as_ref())),
        request
            .profile
            .as_ref()
            .map(|profile| ("profile", profile.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
        request
            .value
            .as_ref()
            .map(|value| ("value", value.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.profile.set");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetResponse>(&result)
                .map_err(|e| SetError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
