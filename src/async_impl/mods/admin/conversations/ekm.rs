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
pub use crate::mod_types::admin::conversations::ekm_types::*;
use std::borrow::Cow;

/// List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.
///
/// Wraps https://api.slack.com/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo

pub async fn list_original_connected_channel_info<R>(
    client: &R,
    token: &str,
    request: &ListOriginalConnectedChannelInfoRequest<'_>,
) -> Result<ListOriginalConnectedChannelInfoResponse, ListOriginalConnectedChannelInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let limit: Option<Cow<'_, str>> = request.limit.as_ref().map(|limit| limit.to_string().into());
    let params: Vec<Option<(&str, &str)>> = vec![
        Some(("token", token)),
        request
            .channel_ids
            .as_ref()
            .map(|channel_ids| ("channel_ids", channel_ids.as_ref())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.as_ref())),
        limit.as_ref().map(|limit| ("limit", limit.as_ref())),
        request
            .team_ids
            .as_ref()
            .map(|team_ids| ("team_ids", team_ids.as_ref())),
    ];
    let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method(
        "/admin.conversations.ekm.listOriginalConnectedChannelInfo",
    );
    client
        .get(&url, &params[..])
        .await
        .map_err(ListOriginalConnectedChannelInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListOriginalConnectedChannelInfoResponse>(&result)
                .map_err(|e| ListOriginalConnectedChannelInfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
