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

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub use crate::mod_types::admin::conversations::ekm_types::*;
use crate::sync::SlackWebRequestSender;

/// List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.
///
/// Wraps https://api.slack.com/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo

pub fn list_original_connected_channel_info<R>(
    client: &R,
    request: &ListOriginalConnectedChannelInfoRequest,
) -> Result<ListOriginalConnectedChannelInfoResponse, ListOriginalConnectedChannelInfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .channel_ids
            .as_ref()
            .map(|channel_ids| ("channel_ids", channel_ids.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .team_ids
            .as_ref()
            .map(|team_ids| ("team_ids", team_ids.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method(
        "/admin.conversations.ekm.listOriginalConnectedChannelInfo",
    );
    client
        .get(&url, &params[..])
        .map_err(ListOriginalConnectedChannelInfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListOriginalConnectedChannelInfoResponse>(&result)
                .map_err(|e| ListOriginalConnectedChannelInfoError::MalformedResponse(result, e))
        })
}
