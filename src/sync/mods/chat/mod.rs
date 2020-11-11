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

pub mod scheduled_messages;

pub use crate::mod_types::chat::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete

pub fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest<'_>,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let as_user: Option<Cow<'_, str>> = request
        .as_user
        .as_ref()
        .map(|as_user| as_user.to_string().into());
    let ts: Option<Cow<'_, str>> = Some(request.ts.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        Some(("channel", request.channel.as_ref())),
        ts.as_ref().map(|ts| ("ts", ts.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.delete");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Deletes a pending scheduled message from the queue.
///
/// Wraps https://api.slack.com/methods/chat.deleteScheduledMessage

pub fn delete_scheduled_message<R>(
    client: &R,
    token: &str,
    request: &DeleteScheduledMessageRequest<'_>,
) -> Result<DeleteScheduledMessageResponse, DeleteScheduledMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let as_user: Option<Cow<'_, str>> = request
        .as_user
        .as_ref()
        .map(|as_user| as_user.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        Some(("channel", request.channel.as_ref())),
        Some((
            "scheduled_message_id",
            request.scheduled_message_id.as_ref(),
        )),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.deleteScheduledMessage");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(DeleteScheduledMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteScheduledMessageResponse>(&result)
                .map_err(|e| DeleteScheduledMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Retrieve a permalink URL for a specific extant message
///
/// Wraps https://api.slack.com/methods/chat.getPermalink

pub fn get_permalink<R>(
    client: &R,
    token: &str,
    request: &GetPermalinkRequest<'_>,
) -> Result<GetPermalinkResponse, GetPermalinkError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        Some(("channel", request.channel.as_ref())),
        Some(("message_ts", request.message_ts.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.getPermalink");
    client
        .get(&url, &params[..])
        .map_err(GetPermalinkError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPermalinkResponse>(&result)
                .map_err(|e| GetPermalinkError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Share a me message into a channel.
///
/// Wraps https://api.slack.com/methods/chat.meMessage

pub fn me_message<R>(
    client: &R,
    token: &str,
    request: &MeMessageRequest<'_>,
) -> Result<MeMessageResponse, MeMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel", request.channel.as_ref())),
        Some(("text", request.text.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.meMessage");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(MeMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result)
                .map_err(|e| MeMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Sends an ephemeral message to a user in a channel.
///
/// Wraps https://api.slack.com/methods/chat.postEphemeral

pub fn post_ephemeral<R>(
    client: &R,
    token: &str,
    request: &PostEphemeralRequest<'_>,
) -> Result<PostEphemeralResponse, PostEphemeralError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let as_user: Option<Cow<'_, str>> = request
        .as_user
        .as_ref()
        .map(|as_user| as_user.to_string().into());
    let link_names: Option<Cow<'_, str>> = request
        .link_names
        .as_ref()
        .map(|link_names| link_names.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.as_ref())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.as_ref())),
        Some(("channel", request.channel.as_ref())),
        request
            .icon_emoji
            .as_ref()
            .map(|icon_emoji| ("icon_emoji", icon_emoji.as_ref())),
        request
            .icon_url
            .as_ref()
            .map(|icon_url| ("icon_url", icon_url.as_ref())),
        link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.as_ref())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.as_ref())),
        request.text.as_ref().map(|text| ("text", text.as_ref())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.as_ref())),
        Some(("user", request.user.as_ref())),
        request
            .username
            .as_ref()
            .map(|username| ("username", username.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.postEphemeral");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(PostEphemeralError::Client)
        .and_then(|result| {
            serde_json::from_str::<PostEphemeralResponse>(&result)
                .map_err(|e| PostEphemeralError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage

pub fn post_message<R>(
    client: &R,
    token: &str,
    request: &PostMessageRequest<'_>,
) -> Result<PostMessageResponse, PostMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let link_names: Option<Cow<'_, str>> = request
        .link_names
        .as_ref()
        .map(|link_names| link_names.to_string().into());
    let mrkdwn: Option<Cow<'_, str>> = request
        .mrkdwn
        .as_ref()
        .map(|mrkdwn| mrkdwn.to_string().into());
    let reply_broadcast: Option<Cow<'_, str>> = request
        .reply_broadcast
        .as_ref()
        .map(|reply_broadcast| reply_broadcast.to_string().into());
    let unfurl_links: Option<Cow<'_, str>> = request
        .unfurl_links
        .as_ref()
        .map(|unfurl_links| unfurl_links.to_string().into());
    let unfurl_media: Option<Cow<'_, str>> = request
        .unfurl_media
        .as_ref()
        .map(|unfurl_media| unfurl_media.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.as_ref())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.as_ref())),
        Some(("channel", request.channel.as_ref())),
        request
            .icon_emoji
            .as_ref()
            .map(|icon_emoji| ("icon_emoji", icon_emoji.as_ref())),
        request
            .icon_url
            .as_ref()
            .map(|icon_url| ("icon_url", icon_url.as_ref())),
        link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.as_ref())),
        mrkdwn.as_ref().map(|mrkdwn| ("mrkdwn", mrkdwn.as_ref())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.as_ref())),
        reply_broadcast
            .as_ref()
            .map(|reply_broadcast| ("reply_broadcast", reply_broadcast.as_ref())),
        Some(("text", request.text.as_ref())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.as_ref())),
        unfurl_links
            .as_ref()
            .map(|unfurl_links| ("unfurl_links", unfurl_links.as_ref())),
        unfurl_media
            .as_ref()
            .map(|unfurl_media| ("unfurl_media", unfurl_media.as_ref())),
        request
            .username
            .as_ref()
            .map(|username| ("username", username.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.postMessage");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(PostMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<PostMessageResponse>(&result)
                .map_err(|e| PostMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Schedules a message to be sent to a channel.
///
/// Wraps https://api.slack.com/methods/chat.scheduleMessage

pub fn schedule_message<R>(
    client: &R,
    token: Option<&str>,
    request: &ScheduleMessageRequest<'_>,
) -> Result<ScheduleMessageResponse, ScheduleMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let as_user: Option<Cow<'_, str>> = request
        .as_user
        .as_ref()
        .map(|as_user| as_user.to_string().into());
    let link_names: Option<Cow<'_, str>> = request
        .link_names
        .as_ref()
        .map(|link_names| link_names.to_string().into());
    let reply_broadcast: Option<Cow<'_, str>> = request
        .reply_broadcast
        .as_ref()
        .map(|reply_broadcast| reply_broadcast.to_string().into());
    let thread_ts: Option<Cow<'_, str>> = request
        .thread_ts
        .as_ref()
        .map(|thread_ts| thread_ts.to_string().into());
    let unfurl_links: Option<Cow<'_, str>> = request
        .unfurl_links
        .as_ref()
        .map(|unfurl_links| unfurl_links.to_string().into());
    let unfurl_media: Option<Cow<'_, str>> = request
        .unfurl_media
        .as_ref()
        .map(|unfurl_media| unfurl_media.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.as_ref())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.as_ref())),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.as_ref())),
        link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.as_ref())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.as_ref())),
        request
            .post_at
            .as_ref()
            .map(|post_at| ("post_at", post_at.as_ref())),
        reply_broadcast
            .as_ref()
            .map(|reply_broadcast| ("reply_broadcast", reply_broadcast.as_ref())),
        request.text.as_ref().map(|text| ("text", text.as_ref())),
        thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.as_ref())),
        unfurl_links
            .as_ref()
            .map(|unfurl_links| ("unfurl_links", unfurl_links.as_ref())),
        unfurl_media
            .as_ref()
            .map(|unfurl_media| ("unfurl_media", unfurl_media.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.scheduleMessage");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t)]),
        )
        .map_err(ScheduleMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<ScheduleMessageResponse>(&result)
                .map_err(|e| ScheduleMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Provide custom unfurl behavior for user-posted URLs
///
/// Wraps https://api.slack.com/methods/chat.unfurl

pub fn unfurl<R>(
    client: &R,
    token: &str,
    request: &UnfurlRequest<'_>,
) -> Result<UnfurlResponse, UnfurlError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let user_auth_required: Option<Cow<'_, str>> = request
        .user_auth_required
        .as_ref()
        .map(|user_auth_required| user_auth_required.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("channel", request.channel.as_ref())),
        Some(("ts", request.ts.as_ref())),
        Some(("unfurls", request.unfurls.as_ref())),
        request
            .user_auth_message
            .as_ref()
            .map(|user_auth_message| ("user_auth_message", user_auth_message.as_ref())),
        user_auth_required
            .as_ref()
            .map(|user_auth_required| ("user_auth_required", user_auth_required.as_ref())),
        request
            .user_auth_url
            .as_ref()
            .map(|user_auth_url| ("user_auth_url", user_auth_url.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.unfurl");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(UnfurlError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnfurlResponse>(&result)
                .map_err(|e| UnfurlError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update

pub fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest<'_>,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: Vec<Option<(&str, &str)>> = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.as_ref())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.as_ref())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.as_ref())),
        Some(("channel", request.channel.as_ref())),
        request
            .link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.as_ref())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.as_ref())),
        request.text.as_ref().map(|text| ("text", text.as_ref())),
        Some(("ts", request.ts.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.update");
    client
        .post(&url, &params[..], &[("token", token)])
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
