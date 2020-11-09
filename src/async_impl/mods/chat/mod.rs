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

use crate::async_impl::SlackWebRequestSender;
pub use crate::mod_types::chat::*;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete

pub async fn delete<R>(
    client: &R,
    token: Option<&str>,
    request: &DeleteRequest,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.ts.as_ref().map(|ts| ("ts", ts.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.delete");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
}
/// Deletes a pending scheduled message from the queue.
///
/// Wraps https://api.slack.com/methods/chat.deleteScheduledMessage

pub async fn delete_scheduled_message<R>(
    client: &R,
    token: &str,
    request: &DeleteScheduledMessageRequest,
) -> Result<DeleteScheduledMessageResponse, DeleteScheduledMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        Some(("channel", request.channel.to_string())),
        Some((
            "scheduled_message_id",
            request.scheduled_message_id.to_string(),
        )),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.deleteScheduledMessage");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(DeleteScheduledMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteScheduledMessageResponse>(&result)
                .map_err(|e| DeleteScheduledMessageError::MalformedResponse(result, e))
        })
}
/// Retrieve a permalink URL for a specific extant message
///
/// Wraps https://api.slack.com/methods/chat.getPermalink

pub async fn get_permalink<R>(
    client: &R,
    token: &str,
    request: &GetPermalinkRequest,
) -> Result<GetPermalinkResponse, GetPermalinkError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("channel", request.channel.to_string())),
        Some(("message_ts", request.message_ts.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.getPermalink");
    client
        .get(&url, &params[..])
        .await
        .map_err(GetPermalinkError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPermalinkResponse>(&result)
                .map_err(|e| GetPermalinkError::MalformedResponse(result, e))
        })
}
/// Share a me message into a channel.
///
/// Wraps https://api.slack.com/methods/chat.meMessage

pub async fn me_message<R>(
    client: &R,
    token: Option<&str>,
    request: &MeMessageRequest,
) -> Result<MeMessageResponse, MeMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request.text.as_ref().map(|text| ("text", text.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.meMessage");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(MeMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result)
                .map_err(|e| MeMessageError::MalformedResponse(result, e))
        })
}
/// Sends an ephemeral message to a user in a channel.
///
/// Wraps https://api.slack.com/methods/chat.postEphemeral

pub async fn post_ephemeral<R>(
    client: &R,
    token: &str,
    request: &PostEphemeralRequest,
) -> Result<PostEphemeralResponse, PostEphemeralError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.to_string())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.to_string())),
        Some(("channel", request.channel.to_string())),
        request
            .icon_emoji
            .as_ref()
            .map(|icon_emoji| ("icon_emoji", icon_emoji.to_string())),
        request
            .icon_url
            .as_ref()
            .map(|icon_url| ("icon_url", icon_url.to_string())),
        request
            .link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.to_string())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.to_string())),
        request.text.as_ref().map(|text| ("text", text.to_string())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.to_string())),
        Some(("user", request.user.to_string())),
        request
            .username
            .as_ref()
            .map(|username| ("username", username.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.postEphemeral");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(PostEphemeralError::Client)
        .and_then(|result| {
            serde_json::from_str::<PostEphemeralResponse>(&result)
                .map_err(|e| PostEphemeralError::MalformedResponse(result, e))
        })
}
/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage

pub async fn post_message<R>(
    client: &R,
    token: &str,
    request: &PostMessageRequest,
) -> Result<PostMessageResponse, PostMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.to_string())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.to_string())),
        Some(("channel", request.channel.to_string())),
        request
            .icon_emoji
            .as_ref()
            .map(|icon_emoji| ("icon_emoji", icon_emoji.to_string())),
        request
            .icon_url
            .as_ref()
            .map(|icon_url| ("icon_url", icon_url.to_string())),
        request
            .link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.to_string())),
        request
            .mrkdwn
            .as_ref()
            .map(|mrkdwn| ("mrkdwn", mrkdwn.to_string())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.to_string())),
        request
            .reply_broadcast
            .as_ref()
            .map(|reply_broadcast| ("reply_broadcast", reply_broadcast.to_string())),
        request.text.as_ref().map(|text| ("text", text.to_string())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.to_string())),
        request
            .unfurl_links
            .as_ref()
            .map(|unfurl_links| ("unfurl_links", unfurl_links.to_string())),
        request
            .unfurl_media
            .as_ref()
            .map(|unfurl_media| ("unfurl_media", unfurl_media.to_string())),
        request
            .username
            .as_ref()
            .map(|username| ("username", username.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.postMessage");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(PostMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<PostMessageResponse>(&result)
                .map_err(|e| PostMessageError::MalformedResponse(result, e))
        })
}
/// Schedules a message to be sent to a channel.
///
/// Wraps https://api.slack.com/methods/chat.scheduleMessage

pub async fn schedule_message<R>(
    client: &R,
    token: Option<&str>,
    request: &ScheduleMessageRequest,
) -> Result<ScheduleMessageResponse, ScheduleMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.to_string())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.to_string())),
        request
            .channel
            .as_ref()
            .map(|channel| ("channel", channel.to_string())),
        request
            .link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.to_string())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.to_string())),
        request
            .post_at
            .as_ref()
            .map(|post_at| ("post_at", post_at.to_string())),
        request
            .reply_broadcast
            .as_ref()
            .map(|reply_broadcast| ("reply_broadcast", reply_broadcast.to_string())),
        request.text.as_ref().map(|text| ("text", text.to_string())),
        request
            .thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", thread_ts.to_string())),
        request
            .unfurl_links
            .as_ref()
            .map(|unfurl_links| ("unfurl_links", unfurl_links.to_string())),
        request
            .unfurl_media
            .as_ref()
            .map(|unfurl_media| ("unfurl_media", unfurl_media.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.scheduleMessage");
    client
        .post(
            &url,
            &params[..],
            &token.map_or(vec![], |t| vec![("token", t.to_string())]),
        )
        .await
        .map_err(ScheduleMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<ScheduleMessageResponse>(&result)
                .map_err(|e| ScheduleMessageError::MalformedResponse(result, e))
        })
}
/// Provide custom unfurl behavior for user-posted URLs
///
/// Wraps https://api.slack.com/methods/chat.unfurl

pub async fn unfurl<R>(
    client: &R,
    token: &str,
    request: &UnfurlRequest,
) -> Result<UnfurlResponse, UnfurlError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("channel", request.channel.to_string())),
        Some(("ts", request.ts.to_string())),
        request
            .unfurls
            .as_ref()
            .map(|unfurls| ("unfurls", unfurls.to_string())),
        request
            .user_auth_message
            .as_ref()
            .map(|user_auth_message| ("user_auth_message", user_auth_message.to_string())),
        request
            .user_auth_required
            .as_ref()
            .map(|user_auth_required| ("user_auth_required", user_auth_required.to_string())),
        request
            .user_auth_url
            .as_ref()
            .map(|user_auth_url| ("user_auth_url", user_auth_url.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.unfurl");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(UnfurlError::Client)
        .and_then(|result| {
            serde_json::from_str::<UnfurlResponse>(&result)
                .map_err(|e| UnfurlError::MalformedResponse(result, e))
        })
}
/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update

pub async fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .as_user
            .as_ref()
            .map(|as_user| ("as_user", as_user.to_string())),
        request
            .attachments
            .as_ref()
            .map(|attachments| ("attachments", attachments.to_string())),
        request
            .blocks
            .as_ref()
            .map(|blocks| ("blocks", blocks.to_string())),
        Some(("channel", request.channel.to_string())),
        request
            .link_names
            .as_ref()
            .map(|link_names| ("link_names", link_names.to_string())),
        request
            .parse
            .as_ref()
            .map(|parse| ("parse", parse.to_string())),
        request.text.as_ref().map(|text| ("text", text.to_string())),
        Some(("ts", request.ts.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/chat.update");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .await
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
}
