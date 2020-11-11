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

pub mod profile;

pub use crate::mod_types::users::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// List conversations the calling user may access.
///
/// Wraps https://api.slack.com/methods/users.conversations

pub fn conversations<R>(
    client: &R,
    token: Option<&str>,
    request: &ConversationsRequest<'_>,
) -> Result<ConversationsResponse, ConversationsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let exclude_archived: Option<Cow<'_, str>> = request
        .exclude_archived
        .as_ref()
        .map(|exclude_archived| exclude_archived.to_string().into());
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        exclude_archived
            .as_ref()
            .map(|exclude_archived| ("exclude_archived", exclude_archived.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        request
            .types
            .as_ref()
            .map(|types| ("types", types.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.conversations");
    client
        .get(&url, &params[..])
        .map_err(ConversationsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConversationsResponse>(&result)
                .map_err(|e| ConversationsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

pub fn delete_photo<R>(
    client: &R,
    token: &str,
    _request: &DeletePhotoRequest,
) -> Result<DeletePhotoResponse, DeletePhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.deletePhoto");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(DeletePhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeletePhotoResponse>(&result)
                .map_err(|e| DeletePhotoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

pub fn get_presence<R>(
    client: &R,
    token: &str,
    request: &GetPresenceRequest<'_>,
) -> Result<GetPresenceResponse, GetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.getPresence");
    client
        .get(&url, &params[..])
        .map_err(GetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result)
                .map_err(|e| GetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

pub fn identity<R>(
    client: &R,
    token: Option<&str>,
    _request: &IdentityRequest,
) -> Result<IdentityResponse, IdentityError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![token.map(|token| ("token", token))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.identity");
    client
        .get(&url, &params[..])
        .map_err(IdentityError::Client)
        .and_then(|result| {
            serde_json::from_str::<IdentityResponse>(&result)
                .map_err(|e| IdentityError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_locale: Option<Cow<'_, str>> = request
        .include_locale
        .as_ref()
        .map(|include_locale| include_locale.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        include_locale
            .as_ref()
            .map(|include_locale| ("include_locale", include_locale.as_ref())),
        request.user.as_ref().map(|user| ("user", user.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

pub fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest<'_>,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let include_locale: Option<Cow<'_, str>> = request
        .include_locale
        .as_ref()
        .map(|include_locale| include_locale.to_string().into());
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        token.map(|token| ("token", token)),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        include_locale
            .as_ref()
            .map(|include_locale| ("include_locale", include_locale.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Find a user with an email address.
///
/// Wraps https://api.slack.com/methods/users.lookupByEmail

pub fn lookup_by_email<R>(
    client: &R,
    token: &str,
    request: &LookupByEmailRequest<'_>,
) -> Result<LookupByEmailResponse, LookupByEmailError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("email", request.email.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.lookupByEmail");
    client
        .get(&url, &params[..])
        .map_err(LookupByEmailError::Client)
        .and_then(|result| {
            serde_json::from_str::<LookupByEmailResponse>(&result)
                .map_err(|e| LookupByEmailError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Marked a user as active. Deprecated and non-functional.
///
/// Wraps https://api.slack.com/methods/users.setActive

pub fn set_active<R>(
    client: &R,
    token: &str,
    _request: &SetActiveRequest,
) -> Result<SetActiveResponse, SetActiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setActive");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetActiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetActiveResponse>(&result)
                .map_err(|e| SetActiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the user profile photo
///
/// Wraps https://api.slack.com/methods/users.setPhoto

pub fn set_photo<R>(
    client: &R,
    token: &str,
    request: &SetPhotoRequest<'_>,
) -> Result<SetPhotoResponse, SetPhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .crop_w
            .as_ref()
            .map(|crop_w| ("crop_w", crop_w.as_ref())),
        request
            .crop_x
            .as_ref()
            .map(|crop_x| ("crop_x", crop_x.as_ref())),
        request
            .crop_y
            .as_ref()
            .map(|crop_y| ("crop_y", crop_y.as_ref())),
        request
            .image
            .as_ref()
            .map(|image| ("image", image.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setPhoto");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetPhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPhotoResponse>(&result)
                .map_err(|e| SetPhotoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

pub fn set_presence<R>(
    client: &R,
    token: &str,
    request: &SetPresenceRequest<'_>,
) -> Result<SetPresenceResponse, SetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![Some(("presence", request.presence.as_ref()))];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setPresence");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(SetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPresenceResponse>(&result)
                .map_err(|e| SetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
