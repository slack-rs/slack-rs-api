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

//! Get info on your direct messages.
//!
//! For more information, see [Slack's API
//! documentation](https://api.slack.com/methods).

use std::collections::HashMap;

use super::{ApiResult, SlackWebRequestSender, parse_slack_response};

/// Close a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.close
pub fn close<R: SlackWebRequestSender>(client: &R, token: &str, channel_id: &str) -> ApiResult<CloseResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
    let response = try!(client.send_authed("im.close", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct CloseResponse {
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

/// Fetches history of messages and events from direct message channel.
///
/// Wraps https://api.slack.com/methods/im.history
pub fn history<R: SlackWebRequestSender>(client: &R,
               token: &str,
               channel_id: &str,
               latest: Option<&str>,
               oldest: Option<&str>,
               inclusive: Option<bool>,
               count: Option<u32>)
               -> ApiResult<HistoryResponse> {
    let count = count.map(|c| c.to_string());
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
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
    let response = try!(client.send_authed("im.history", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct HistoryResponse {
    pub latest: String,
    pub messages: Vec<super::Message>,
    pub has_more: bool,
}

/// Lists direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/im.list
pub fn list<R: SlackWebRequestSender>(client: &R, token: &str) -> ApiResult<ListResponse> {
    let response = try!(client.send_authed("im.list", token, HashMap::new()));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ListResponse {
    pub ims: Vec<super::Im>,
}

/// Sets the read cursor in a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.mark
pub fn mark<R: SlackWebRequestSender>(client: &R, token: &str, channel_id: &str, timestamp: &str) -> ApiResult<MarkResponse> {
    let mut params = HashMap::new();
    params.insert("channel", channel_id);
    params.insert("timestamp", timestamp);
    let response = try!(client.send_authed("im.mark", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct MarkResponse;

/// Opens a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.open
pub fn open<R: SlackWebRequestSender>(client: &R, token: &str, user_id: &str) -> ApiResult<OpenResponse> {
    let mut params = HashMap::new();
    params.insert("user", user_id);
    let response = try!(client.send_authed("im.open", token, params));
    parse_slack_response(response, true)
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct ChannelId {
    pub id: String,
}

#[derive(Clone,Debug,RustcDecodable)]
pub struct OpenResponse {
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
    pub channel: ChannelId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Message;
    use super::super::test_helpers::*;

    #[test]
    fn general_api_error_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": false, "err": "some_error"}"#);
        let result = close(&client, "TEST_TOKEN", "D12345678");
        assert!(result.is_err());
    }

    #[test]
    fn close_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = close(&client, "TEST_TOKEN", "D12345678");
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
        let result = close(&client, "TEST_TOKEN", "D12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().already_closed.unwrap(), true);
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
        let result = history(&client, "TEST_TOKEN", "D12345678", None, None, None, None);
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        match result.unwrap().messages[0].clone() {
            Message::Standard { ts: _, channel: _, user: _, text, is_starred: _, pinned_to: _, reactions: _, edited: _, attachments: _, .. } => {
                assert_eq!(text.unwrap(), "Hello");
            }
            _ => panic!("Message decoded into incorrect variant."),
        }
    }

    #[test]
    fn list_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "ims": [
                {
                   "id": "D024BFF1M",
                   "is_im": true,
                   "user": "USLACKBOT",
                   "created": 1372105335,
                   "is_user_deleted": false
                },
                {
                   "id": "D024BE7RE",
                   "is_im": true,
                   "user": "U024BE7LH",
                   "created": 1356250715,
                   "is_user_deleted": false
                }
            ]
        }"#);
        let result = list(&client, "TEST_TOKEN");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert!(result.unwrap().ims[1].id == "D024BE7RE");
    }

    #[test]
    fn mark_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{"ok": true}"#);
        let result = mark(&client, "TEST_TOKEN", "D12345678", "1234567890.123456");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
    }

    #[test]
    fn open_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "channel": {
                "id": "D024BFF1M"
            }
        }"#);
        let result = open(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        assert_eq!(result.unwrap().channel.id, "D024BFF1M");
    }

    #[test]
    fn open_already_open_ok_response() {
        let client = MockSlackWebRequestSender::respond_with(r#"{
            "ok": true,
            "no_op": true,
            "already_open": true,
            "channel": {
                "id": "D024BFF1M"
            }
        }"#);
        let result = open(&client, "TEST_TOKEN", "U12345678");
        if let Err(err) = result {
            panic!(format!("{:?}", err));
        }
        let result = result.unwrap();
        assert_eq!(result.channel.id, "D024BFF1M");
        assert_eq!(result.already_open.unwrap(), true);
    }
}
