// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Get info on your team's Slack channels, create or archive channels, invite
//! users, set the topic and purpose, and mark a channel as read.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive
pub fn archive<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<ArchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("channels.archive", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ArchiveResponse;

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create
pub fn create<R: SlackWebRequestSender>(client: &R, token: &str, name: &str) -> ApiResult<CreateResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    let response = try!(client.send_authed("channels.create", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateResponse {
    pub channel: super::Channel,
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history
pub fn history<R: SlackWebRequestSender>(client: &R,
               token: &str,
               channel: &str,
               latest: Option<&str>,
               oldest: Option<&str>,
               inclusive: Option<bool>,
               count: Option<u32>)
               -> ApiResult<HistoryResponse> {
    let count = count.map(|c| c.to_string());
    let mut params = HashMap::new();
    params.insert("channel", channel);
    if let Some(latest) = latest {
        params.insert("latest", latest);
    }
    if let Some(oldest) = oldest {
        params.insert("oldest", oldest);
    }
    if let Some(inclusive) = inclusive {
        params.insert("inclusive",
                      if inclusive {
                          "1"
                      } else {
                          "0"
                      });
    }
    if let Some(ref count) = count {
        params.insert("count", count);
    }
    let response = try!(client.send_authed("channels.history", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: Option<String>,
    pub oldest: Option<String>,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info
pub fn info<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("channels.info", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub channel: super::Channel,
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite
pub fn invite<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, user: &str) -> ApiResult<InviteResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    let response = try!(client.send_authed("channels.invite", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InviteResponse {
    pub channel: super::Channel,
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join
pub fn join<R: SlackWebRequestSender>(client: &R, token: &str, name: &str) -> ApiResult<JoinResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    let response = try!(client.send_authed("channels.join", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct JoinResponse {
    pub already_in_channel: Option<bool>,
    pub channel: super::Channel,
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick
pub fn kick<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, user: &str) -> ApiResult<KickResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    let response = try!(client.send_authed("channels.kick", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct KickResponse;

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave
pub fn leave<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<LeaveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("channels.leave", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LeaveResponse {
    pub not_in_channel: Option<bool>,
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str, exclude_archived: Option<bool>) -> ApiResult<ListResponse> {
    let mut params = HashMap::new();
    if let Some(exclude_archived) = exclude_archived {
        params.insert("exclude_archived",
                      if exclude_archived {
                          "1"
                      } else {
                          "0"
                      });
    }
    let response = try!(client.send_authed("channels.list", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub channels: Vec<super::Channel>,
}

/// Sets the read cursor in a channel.
///
/// https://api.slack.com/methods/channels.mark
pub fn mark<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, ts: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("ts", ts);
    let response = try!(client.send_authed("channels.mark", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename
pub fn rename<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, name: &str) -> ApiResult<RenameResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("name", name);
    let response = try!(client.send_authed("channels.rename", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AbridgedChannel {
    pub id: String,
    pub is_channel: bool,
    pub name: String,
    pub created: i32,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RenameResponse {
    pub channel: AbridgedChannel,
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose
pub fn set_purpose<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, purpose: &str) -> ApiResult<SetPurposeResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("purpose", purpose);
    let response = try!(client.send_authed("channels.setPurpose", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPurposeResponse {
    pub purpose: String,
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic
pub fn set_topic<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, topic: &str) -> ApiResult<SetTopicResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("topic", topic);
    let response = try!(client.send_authed("channels.setTopic", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetTopicResponse {
    pub topic: String,
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive
pub fn unarchive<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<UnarchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("channels.unarchive", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct UnarchiveResponse;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Message;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = info(&client, "TEST_TOKEN", "TEST_CHANNEL");
        assert!(result.is_err());
    }

    #[test]
    fn archive_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = archive(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn create_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#);
        let result = create(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    #[test]
    fn history_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "messages": [
                {
                    "type": "message",
                    "user": "U024BE7LH",
                    "text": "lol",
                    "ts": "1444078138.000084"
                },
                {
                    "type": "message",
                    "user": "U024BE7LH",
                    "text": "Hello world",
                    "ts": "1444078079.000083"
                }
            ],
            "has_more": true
        }"#);
        let result = history(&client,
                             "TEST_TOKEN",
                             "TEST_CHANNEL",
                             None,
                             None,
                             None,
                             None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            Message::Standard { text, .. } => {
                assert_eq!(text.unwrap(), "lol");
            }
            _ => panic!("Message decoded into incorrect variant."),
        }
    }

    #[test]
    fn info_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#);
        let result = info(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    #[test]
    fn invite_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#);
        let result = invite(&client, "TEST_TOKEN", "TEST_CHANNEL", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    #[test]
    fn join_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91L",
                "name": "testing",
                "is_channel": true,
                "created": 1444102158,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_general": false,
                "is_member": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                },
                "purpose": {
                    "value": "",
                    "creator": "",
                    "last_set": 0
                }
            }
        }"#);
        let result = join(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.id == "C024BE91L");
    }

    #[test]
    fn kick_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = kick(&client, "TEST_TOKEN", "TEST_CHANNEL", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn leave_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = leave(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channels": [
                {
                    "id": "C024BE91L",
                    "name": "testing",
                    "is_channel": true,
                    "created": 1444102158,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_general": false,
                    "is_member": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    },
                    "purpose": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    }
                },
                {
                    "id": "C024BE91J",
                    "name": "testing",
                    "is_channel": true,
                    "created": 1444102158,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_general": false,
                    "is_member": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    },
                    "purpose": {
                        "value": "",
                        "creator": "",
                        "last_set": 0
                    }
                }
            ]
        }"#);
        let result = list(&client, "TEST_TOKEN", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channels[1].id == "C024BE91J");
    }

    #[test]
    fn mark_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = mark(&client, "TEST_TOKEN", "TEST_CHANNEL", "1234567890.123456");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn rename_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "C024BE91J",
                "is_channel": true,
                "name": "NEW_NAME",
                "created": 1444102158
            }
        }"#);
        let result = rename(&client, "TEST_TOKEN", "TEST_CHANNEL", "newname");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().channel.name == "NEW_NAME")
    }

    #[test]
    fn set_purpose_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "purpose": "This is the new purpose!"
        }"#);
        let result = set_purpose(&client,
                                 "TEST_TOKEN",
                                 "TEST_CHANNEL",
                                 "This is the new purpose!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().purpose == "This is the new purpose!")
    }

    #[test]
    fn set_topic_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "topic": "This is the new topic!"
        }"#);
        let result = set_topic(&client,
                               "TEST_TOKEN",
                               "TEST_CHANNEL",
                               "This is the new topic!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().topic == "This is the new topic!")
    }

    #[test]
    fn unarchive_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = unarchive(&client, "TEST_TOKEN", "TEST_CHANNEL");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
