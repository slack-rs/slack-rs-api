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

//! Get info on your team's private groups.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Archives a private group.
///
/// Wraps https://api.slack.com/methods/groups.archive
pub fn archive<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<ArchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.archive", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ArchiveResponse;

/// Closes a private group.
///
/// Wraps https://api.slack.com/methods/groups.close
pub fn close<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<CloseResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.close", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CloseResponse {
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

/// Creates a private group.
///
/// Wraps https://api.slack.com/methods/groups.create
pub fn create<R: SlackWebRequestSender>(client: &R, token: &str, name: &str) -> ApiResult<CreateResponse> {
    let mut params = HashMap::new();
    params.insert("name", name);
    let response = try!(client.send_authed("groups.create", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateResponse {
    pub group: super::Group,
}

/// Clones and archives a private group.
///
/// Wraps https://api.slack.com/methods/groups.createChild
pub fn create_child<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<CreateChildResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.createChild", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CreateChildResponse {
    pub group: super::Group,
}

/// Fetches history of messages and events from a private group.
///
/// Wraps https://api.slack.com/methods/groups.history
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
    let response = try!(client.send_authed("groups.history", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: Option<String>,
    pub oldest: Option<String>,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
    pub is_limited: Option<bool>,
}

/// Gets information about a private group.
///
/// Wraps https://api.slack.com/methods/groups.info
pub fn info<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<InfoResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.info", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InfoResponse {
    pub group: super::Group,
}

/// Invites a user to a private group.
///
/// Wraps https://api.slack.com/methods/groups.invite
pub fn invite<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, user: &str) -> ApiResult<InviteResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    let response = try!(client.send_authed("groups.invite", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct InviteResponse {
    pub group: super::Group,
    pub already_in_group: Option<bool>,
}

/// Removes a user from a private group.
///
/// Wraps https://api.slack.com/methods/groups.kick
pub fn kick<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, user: &str) -> ApiResult<KickResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("user", user);
    let response = try!(client.send_authed("groups.kick", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct KickResponse;

/// Leaves a private group.
///
/// Wraps https://api.slack.com/methods/groups.leave
pub fn leave<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<LeaveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.leave", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct LeaveResponse;

/// Lists private groups that the calling user has access to.
///
/// Wraps https://api.slack.com/methods/groups.list
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
    let response = try!(client.send_authed("groups.list", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub groups: Vec<super::Group>,
}

/// Sets the read cursor in a private group.
///
/// Wraps https://api.slack.com/methods/groups.mark
pub fn mark<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, ts: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("ts", ts);
    let response = try!(client.send_authed("groups.mark", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Opens a private group.
///
/// Wraps https://api.slack.com/methods/groups.open
pub fn open<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<OpenResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.open", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct OpenResponse {
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
}

/// Renames a private group.
///
/// Wraps https://api.slack.com/methods/groups.rename
pub fn rename<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, name: &str) -> ApiResult<RenameResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("name", name);
    let response = try!(client.send_authed("groups.rename", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct AbridgedGroup {
    pub id: String,
    pub is_group: bool,
    pub name: String,
    pub created: i32,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct RenameResponse {
    pub channel: AbridgedGroup,
}

/// Sets the purpose for a private group.
///
/// Wraps https://api.slack.com/methods/groups.setPurpose
pub fn set_purpose<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, purpose: &str) -> ApiResult<SetPurposeResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("purpose", purpose);
    let response = try!(client.send_authed("groups.setPurpose", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetPurposeResponse {
    pub purpose: String,
}

/// Sets the topic for a private group.
///
/// Wraps https://api.slack.com/methods/groups.setTopic
pub fn set_topic<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str, topic: &str) -> ApiResult<SetTopicResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    params.insert("topic", topic);
    let response = try!(client.send_authed("groups.setTopic", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct SetTopicResponse {
    pub topic: String,
}

/// Unarchives a private group.
///
/// Wraps https://api.slack.com/methods/groups.unarchive
pub fn unarchive<R: SlackWebRequestSender>(client: &R, token: &str, channel: &str) -> ApiResult<UnarchiveResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel);
    let response = try!(client.send_authed("groups.unarchive", token, params));
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
        let result = info(&client, "TEST_TOKEN", "G12345678");
        assert!(result.is_err());
    }

    #[test]
    fn archive_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = archive(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
    
    #[test]
    fn close_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = close(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn close_already_closed_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "no_op": true,
            "already_closed": true
        }"#);
        let result = close(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().already_closed.unwrap(), true);
    }

    #[test]
    fn create_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#);
        let result = create(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    #[test]
    fn create_child_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#);
        let result = create_child(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    #[test]
    fn history_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "latest": "1358547726.000003",
            "messages": [
                {
                    "type": "message",
                    "ts": "1358546515.000008",
                    "user": "U2147483896",
                    "text": "Hello"
                },
                {
                    "type": "message",
                    "ts": "1358546515.000007",
                    "user": "U2147483896",
                    "text": "World",
                    "is_starred": true
                },
                {
                    "type": "something_else",
                    "ts": "1358546515.000007",
                    "wibblr": true
                }
            ],
            "has_more": false
        }"#);
        let result = history(&client, "TEST_TOKEN", "G12345678", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _, .. } => {
                assert_eq!(text.unwrap(), "Hello")
            }
            _ => panic!("Message decoded into incorrect variant."),
        }
    }

    #[test]
    fn info_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#);
        let result = info(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }

    #[test]
    fn invite_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#);
        let result = invite(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().group.id == "G12345678");
    }
    
    #[test]
    fn invite_already_in_channel_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "already_in_group": true,
            "group": {
                "id": "G12345678",
                "name": "secretplans",
                "is_group": true,
                "created": 1360782804,
                "creator": "U024BE7LH",
                "is_archived": false,
                "is_open": true,
                "last_read": "0000000000.000000",
                "latest": null,
                "unread_count": 0,
                "unread_count_display": 0,
                "members": [
                    "U024BE7LH"
                ],
                "topic": {
                    "value": "Secret plans on hold",
                    "creator": "U024BE7LV",
                    "last_set": 1369677212
                },
                "purpose": {
                    "value": "Discuss secret plans that no-one else should know",
                    "creator": "U024BE7LH",
                    "last_set": 1360782804
                }
            }
        }"#);
        let result = invite(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().already_in_group.unwrap() == true);
    }

    #[test]
    fn kick_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = kick(&client, "TEST_TOKEN", "G12345678", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn leave_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = leave(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "groups": [
                {
                    "id": "G12345678",
                    "name": "secretplans",
                    "is_group": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_open": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "Secret plans on hold",
                        "creator": "U024BE7LV",
                        "last_set": 1369677212
                    },
                    "purpose": {
                        "value": "Discuss secret plans that no-one else should know",
                        "creator": "U024BE7LH",
                        "last_set": 1360782804
                    }
                },
                {
                    "id": "G87654321",
                    "name": "secretplans",
                    "is_group": true,
                    "created": 1360782804,
                    "creator": "U024BE7LH",
                    "is_archived": false,
                    "is_open": true,
                    "last_read": "0000000000.000000",
                    "latest": null,
                    "unread_count": 0,
                    "unread_count_display": 0,
                    "members": [
                        "U024BE7LH"
                    ],
                    "topic": {
                        "value": "Secret plans on hold",
                        "creator": "U024BE7LV",
                        "last_set": 1369677212
                    },
                    "purpose": {
                        "value": "Discuss secret plans that no-one else should know",
                        "creator": "U024BE7LH",
                        "last_set": 1360782804
                    }
                }
            ]
        }"#);
        let result = list(&client, "TEST_TOKEN", None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().groups[1].id == "G87654321");
    }

    #[test]
    fn mark_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = mark(&client, "TEST_TOKEN", "G12345678", "1234567890.123456");
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
                "is_group": true,
                "name": "NEW_NAME",
                "created": 1444102158
            }
        }"#);
        let result = rename(&client, "TEST_TOKEN", "G12345678", "newname");
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
                                 "G12345678",
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
        let result = set_topic(&client, "TEST_TOKEN", "G12345678", "This is the new topic!");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().topic == "This is the new topic!")
    }

    #[test]
    fn unarchive_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = unarchive(&client, "TEST_TOKEN", "G12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }
}
