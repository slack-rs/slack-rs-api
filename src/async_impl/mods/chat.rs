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

//! Post chat messages to Slack.

pub use crate::mod_types::chat_types::*;
use crate::requests::SlackWebRequestSender;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete

pub async fn delete<R>(
    client: &R,
    token: &str,
    request: &DeleteRequest<'_>,
) -> Result<DeleteResponse, DeleteError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let ts = request.ts.to_param_value();
    let params = vec![
        Some(("token", token)),
        Some(("ts", &ts[..])),
        Some(("channel", request.channel)),
        request
            .as_user
            .map(|as_user| ("as_user", if as_user { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("chat.delete");
    client
        .send(&url, &params[..])
        .await
        .map_err(DeleteError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result)
                .map_err(|e| DeleteError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Share a me message into a channel.
///
/// Wraps https://api.slack.com/methods/chat.meMessage

pub async fn me_message<R>(
    client: &R,
    token: &str,
    request: &MeMessageRequest<'_>,
) -> Result<MeMessageResponse, MeMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("text", request.text)),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("chat.meMessage");
    client
        .send(&url, &params[..])
        .await
        .map_err(MeMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result)
                .map_err(|e| MeMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage

pub async fn post_message<R>(
    client: &R,
    token: &str,
    request: &PostMessageRequest<'_>,
) -> Result<PostMessageResponse, PostMessageError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let thread_ts = request.thread_ts.as_ref().map(|t| t.to_param_value());
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("text", request.text)),
        request.parse.map(|parse| ("parse", parse)),
        request
            .link_names
            .map(|link_names| ("link_names", if link_names { "1" } else { "0" })),
        request
            .attachments
            .map(|attachments| ("attachments", attachments)),
        request
            .unfurl_links
            .map(|unfurl_links| ("unfurl_links", if unfurl_links { "1" } else { "0" })),
        request
            .unfurl_media
            .map(|unfurl_media| ("unfurl_media", if unfurl_media { "1" } else { "0" })),
        request.username.map(|username| ("username", username)),
        request
            .as_user
            .map(|as_user| ("as_user", if as_user { "1" } else { "0" })),
        request.icon_url.map(|icon_url| ("icon_url", icon_url)),
        request
            .icon_emoji
            .map(|icon_emoji| ("icon_emoji", icon_emoji)),
        thread_ts
            .as_ref()
            .map(|thread_ts| ("thread_ts", &thread_ts[..])),
        request
            .reply_broadcast
            .map(|reply_broadcast| ("reply_broadcast", if reply_broadcast { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("chat.postMessage");
    client
        .send(&url, &params[..])
        .await
        .map_err(PostMessageError::Client)
        .and_then(|result| {
            serde_json::from_str::<PostMessageResponse>(&result)
                .map_err(|e| PostMessageError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Unfurl a URL that a user posted
///
/// Wraps https://api.slack.com/methods/chat.unfurl

pub async fn unfurl<R>(
    client: &R,
    token: &str,
    request: &UnfurlRequest<'_>,
) -> Result<UnfurlResponse, UnfurlError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token)),
        Some(("channel", request.channel)),
        Some(("ts", request.ts)),
        Some(("unfurls", request.unfurls)),
        request.user_auth_required.map(|user_auth_required| {
            (
                "user_auth_required",
                if user_auth_required { "1" } else { "0" },
            )
        }),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("chat.unfurl");
    client
        .send(&url, &params[..])
        .await
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

pub async fn update<R>(
    client: &R,
    token: &str,
    request: &UpdateRequest<'_>,
) -> Result<UpdateResponse, UpdateError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let ts = request.ts.to_param_value();
    let params = vec![
        Some(("token", token)),
        Some(("ts", &ts[..])),
        Some(("channel", request.channel)),
        Some(("text", request.text)),
        request
            .attachments
            .map(|attachments| ("attachments", attachments)),
        request.parse.map(|parse| ("parse", parse)),
        request
            .link_names
            .map(|link_names| ("link_names", if link_names { "1" } else { "0" })),
        request
            .as_user
            .map(|as_user| ("as_user", if as_user { "1" } else { "0" })),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("chat.update");
    client
        .send(&url, &params[..])
        .await
        .map_err(UpdateError::Client)
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result)
                .map_err(|e| UpdateError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
